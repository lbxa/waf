---
description: NGINX modules act as either handlers, filters or load-balancers
---

# Modules in Depth

## Components

### Config Structs

Modules define up to three configuration structs, each for the main, server and location contexts of the `nginx.conf` file. The calling conventions for these are `ngx_http_(main|srv|loc)_conf_t`.&#x20;

For example, if we were to write a module targeting the server context we would instead use `ngx_http_X_srv_conf_t`.

```c
typedef struct {
    ngx_uint_t  methods;
    ngx_flag_t  create_full_put_path;
    ngx_uint_t  access;
} ngx_http_hello_world_loc_conf_t;
```

The C struct above shows a few NGINX data types that serve as wrappers to the common primitive types of the C language.

### Directives

Module directives populate the elements in the configuration structs and are called directly from the appropriate context of the configuration file. The structure is an array of commands. The exact definition `ngx_command_t` can be found [core/ngx\_conf\_file.h](https://lxr.nginx.org/source/xref/nginx/src/core/ngx\_conf\_file.c?r=8116%3A3108d4d668e4).&#x20;

This structure is where we would configure whether the directives take any arguments or have restricted access to specific contexts.&#x20;

{% tabs %}
{% tab title="C" %}
```c
static ngx_command_t  ngx_http_hello_world_commands[] = {
    { 
      ngx_string("hello_world"), /* directive */
      NGX_HTTP_LOC_CONF|NGX_CONF_NOARGS, /* location context and takes no arguments*/
      ngx_http_hello_world_commands_set_method, /* configuration setup function */
      NGX_RS_HTTP_LOC_CONF_OFFSET,
      0, /* No offset when storing the module configuration on the struct. */
      NULL
    },
      ...
    ngx_null_command // termination
};
```
{% endtab %}

{% tab title="Rust" %}
```rust
#[no_mangle]
static mut ngx_http_hello_world_commands: [ngx_command_t; 2] = [
    ngx_command_t {
        name: ngx_string!("hello_world"),
        type_: (NGX_HTTP_LOC_CONF| NGX_CONF_NOARGS) as ngx_uint_t,
        set: Some(ngx_http_hello_world_commands_set_method),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_null_command!(), // termination
];
```
{% endtab %}
{% endtabs %}

In my opinion, Rust wins in readability however I'll leave it for the reader to decide. &#x20;

### Context

The module context will house references to functions that create configurations for the module e.g. pre and post-configuration and merge them.&#x20;

{% tabs %}
{% tab title="C" %}
```c
static ngx_http_module_t ngx_http_hello_world_module_ctx = {
    NULL, /* preconfiguration */
    NULL, /* postconfiguration */
    NULL, /* create main configuration */
    NULL, /* init main configuration */
    NULL, /* create server configuration */
    NULL, /* merge server configuration */
    NULL, /* create location configuration */
    NULL /* merge location configuration */
};
```
{% endtab %}

{% tab title="Rust" %}
```rust
#[no_mangle]
static ngx_http_hello_world_module_ctx: ngx_http_module_t = ngx_http_module_t {
    preconfiguration: Some(Module::preconfiguration),
    postconfiguration: Some(Module::postconfiguration),
    create_main_conf: Some(Module::create_main_conf),
    init_main_conf: Some(Module::init_main_conf),
    create_srv_conf: Some(Module::create_srv_conf),
    merge_srv_conf: Some(Module::merge_srv_conf),
    create_loc_conf: Some(Module::create_loc_conf),
    merge_loc_conf: Some(Module::merge_loc_conf),
};
```
{% endtab %}
{% endtabs %}

### Definition

A module's definition struct glues together the context and directives in one place. Additionally, we can also configure process-level functionality at either the life or death of the thread/process.&#x20;

{% tabs %}
{% tab title="C" %}
```c
ngx_module_t ngx_http_hello_world_module = {
    NGX_MODULE_V1,
    &ngx_http_hello_world_module_ctx, /* module context */
    ngx_http_hello_world_commands, /* module directives */
    NGX_HTTP_MODULE, /* module type */
    NULL, /* init master */
    NULL, /* init module */
    NULL, /* init process */
    NULL, /* init thread */
    NULL, /* exit thread */
    NULL, /* exit process */
    NULL, /* exit master */
    NGX_MODULE_V1_PADDING
};
```
{% endtab %}

{% tab title="Rust" %}
```rust
// Create our module structure and export it with the `ngx_modules!` macro. For this simple
// handler, the ngx_module_t is predominately boilerplate save for setting the above context into
// this structure and setting our custom configuration command (defined below).
ngx_modules!(ngx_http_howto_module);

#[no_mangle]
pub static mut ngx_http_hello_world_module: ngx_module_t = ngx_module_t {
    ctx_index: ngx_uint_t::max_value(),
    index: ngx_uint_t::max_value(),
    name: std::ptr::null_mut(),
    spare0: 0,
    spare1: 0,
    version: nginx_version as ngx_uint_t,
    signature: NGX_RS_MODULE_SIGNATURE.as_ptr() as *const c_char,

    ctx: &ngx_http_hello_world_module_ctx as *const _ as *mut _,
    commands: unsafe { &ngx_http_hello_world_commands[0] as *const _ as *mut _ },
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
```
{% endtab %}
{% endtabs %}

