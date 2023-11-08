#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;
mod wrapper;

pub use wrapper::{sqli, xss};

use ngx::{
    core::{self, Pool, Status},
    ffi::{
        nginx_version, ngx_array_push, ngx_command_t, ngx_conf_set_flag_slot, ngx_conf_t,
        ngx_http_core_module, ngx_http_handler_pt, ngx_http_module_t,
        ngx_http_phases_NGX_HTTP_ACCESS_PHASE, ngx_http_request_t, ngx_int_t, ngx_module_t,
        ngx_str_t, ngx_uint_t, NGX_CONF_FLAG, NGX_CONF_TAKE2, NGX_HTTP_LOC_CONF, NGX_HTTP_MODULE,
        NGX_RS_HTTP_LOC_CONF_OFFSET, NGX_RS_MODULE_SIGNATURE,
    },
    http::{self, HTTPModule, MergeConfigError},
    http_request_handler, ngx_log_debug_http, ngx_modules, ngx_null_command, ngx_string,
};
use std::os::raw::{c_char, c_void};

struct Module;

impl http::HTTPModule for Module {
    type MainConf = ();
    type SrvConf = ();
    type LocConf = ModuleConfig;

    unsafe extern "C" fn postconfiguration(_cf: *mut ngx::ffi::ngx_conf_t) -> ngx::ffi::ngx_int_t {
        let htcf = http::ngx_http_conf_get_module_main_conf(_cf, &ngx_http_core_module);

        let h = ngx_array_push(
            &mut (*htcf).phases[ngx_http_phases_NGX_HTTP_ACCESS_PHASE as usize].handlers,
        ) as *mut ngx_http_handler_pt;
        if h.is_null() {
            return core::Status::NGX_ERROR.into();
        }

        // set an Access phase handler
        *h = Some(waf_access_handler);
        core::Status::NGX_OK.into()
    }

    unsafe extern "C" fn create_loc_conf(cf: *mut ngx_conf_t) -> *mut c_void {
        let mut pool = Pool::from_ngx_pool((*cf).pool);
        let conf = pool.allocate::<Self::LocConf>(Default::default()) as *mut c_void;

        if conf.is_null() {
            return std::ptr::null_mut();
        }

        conf
    }
}

#[derive(Debug, Default)]
struct ModuleConfig {
    enabled: bool,
    rules: String,
    xss_enabled: bool,
    sqli_enabled: bool,
}

impl http::Merge for ModuleConfig {
    fn merge(&mut self, prev: &ModuleConfig) -> Result<(), MergeConfigError> {
        if prev.enabled {
            self.enabled = true;
        }

        if self.rules.is_empty() {
            self.rules = String::from(if !prev.rules.is_empty() {
                &prev.rules
            } else {
                ""
            })
        }

        if self.enabled && self.rules.is_empty() {
            return Err(MergeConfigError::NoValue);
        }
        Ok(())
    }
}

#[no_mangle]
static mut ngx_http_waf_commands: [ngx_command_t; 3] = [
    ngx_command_t {
        name: ngx_string!("SecRulesEnabled"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_FLAG) as ngx_uint_t,
        set: Some(ngx_conf_set_flag_slot),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_command_t {
        name: ngx_string!("CheckRule"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_TAKE2) as ngx_uint_t,
        set: Some(ngx_http_waf_commands_set_method),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_null_command!(),
];

#[no_mangle]
extern "C" fn ngx_http_waf_commands_set_method(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        let rule = (*args.add(1)).to_str();
        // implement blocks and allows
        // let action = (*args.add(2)).to_str();

        conf.enabled = true;

        match rule {
            "SQLi" => conf.sqli_enabled = true,
            "XSS" => conf.xss_enabled = true,
            _ => (),
        }
    };

    std::ptr::null_mut()
}

#[no_mangle]
static ngx_http_waf_module_ctx: ngx_http_module_t = ngx_http_module_t {
    preconfiguration: Some(Module::preconfiguration),
    postconfiguration: Some(Module::postconfiguration),
    create_main_conf: Some(Module::create_main_conf),
    init_main_conf: Some(Module::init_main_conf),
    create_srv_conf: Some(Module::create_srv_conf),
    merge_srv_conf: Some(Module::merge_srv_conf),
    create_loc_conf: Some(Module::create_loc_conf),
    merge_loc_conf: Some(Module::merge_loc_conf),
};

// Create our module structure and export it with the `ngx_modules!` macro. For this simple
// handler, the ngx_module_t is predominately boilerplate save for setting the above context into
// this structure and setting our custom configuration command (defined below).
ngx_modules!(ngx_http_waf_module);

#[no_mangle]
pub static mut ngx_http_waf_module: ngx_module_t = ngx_module_t {
    ctx_index: ngx_uint_t::max_value(),
    index: ngx_uint_t::max_value(),
    name: std::ptr::null_mut(),
    spare0: 0,
    spare1: 0,
    version: nginx_version as ngx_uint_t,
    signature: NGX_RS_MODULE_SIGNATURE.as_ptr() as *const c_char,

    ctx: &ngx_http_waf_module_ctx as *const _ as *mut _,
    commands: unsafe { &ngx_http_waf_commands[0] as *const _ as *mut _ },
    type_: NGX_HTTP_MODULE as ngx_uint_t,

    init_master: None,
    init_module: None,
    init_process: None,
    init_thread: None,
    exit_thread: None,
    exit_process: None,
    exit_master: None,

    spare_hook0: 0,
    spare_hook1: 0,
    spare_hook2: 0,
    spare_hook3: 0,
    spare_hook4: 0,
    spare_hook5: 0,
    spare_hook6: 0,
    spare_hook7: 0,
};

http_request_handler!(waf_access_handler, |request: &mut http::Request| {
    let co = unsafe { request.get_module_loc_conf::<ModuleConfig>(&ngx_http_waf_module) };
    let co = co.expect("module config is none");

    ngx_log_debug_http!(request, "waf module enabled called");
    match co.enabled {
        true => {
            if request.method().as_str() == "GET" {
                let uri = request.unparsed_uri();
                let uri = uri.to_string_lossy();
                let (is_sqli, _) = sqli(&uri).unwrap();
                if is_sqli {
                    ngx_log_debug_http!(
                        request,
                        "SQL injection attempt detected in URI: {:?}",
                        uri
                    );
                    return http::HTTPStatus::FORBIDDEN.into();
                }
                return core::Status::NGX_OK;
            }
            http::HTTPStatus::FORBIDDEN.into()
        }
        false => core::Status::NGX_OK,
    }
});
