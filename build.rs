extern crate bindgen;
extern crate git2;

use git2::Repository;
use std::env;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

const LIBINJECTION_URL: &'static str = "https://github.com/libinjection/libinjection";
const BUILD_DIR_NAME: &'static str = "libinjection";

fn clone_repository(repo_url: &str, local_path: &Path) -> Result<Repository, git2::Error> {
    let repo = Repository::clone(repo_url, local_path)?;
    println!("Repository cloned to: {:?}", repo.path());
    Ok(repo)
}

fn run(cmd: &str, args: &[&str], cwd: &Path) -> bool {
    let output = Command::new(cmd)
        .args(args)
        .env("OUT_DIR", env::var("OUT_DIR").unwrap())
        .current_dir(cwd)
        .output()
        .unwrap();

    if output.status.success() {
        true
    } else {
        panic!(
            "failed to run {}: {}",
            cmd,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut build_parent_dir: PathBuf = out_path.join(BUILD_DIR_NAME);

    let _ = remove_dir_all(build_parent_dir.as_path());

    clone_repository(LIBINJECTION_URL, build_parent_dir.as_path())?;

    if !run("bash", &["autogen.sh"], build_parent_dir.as_path()) {
        panic!("error: unable to run autogen.sh");
    }

    if !run("bash", &["configure"], build_parent_dir.as_path()) {
        panic!("error: unable to run configure");
    }

    if !run("make", &["all"], build_parent_dir.as_path()) {
        panic!("error: unable to make libinjection");
    }

    if !run(
        "ar",
        &[
            "-crs",
            "libinjection.a",
            "libinjection_sqli.o",
            "libinjection_html5.o",
            "libinjection_xss.o",
        ],
        build_parent_dir.join("src").as_path(),
    ) {
        panic!("unable to build static library");
    }

    println!("cargo:rustc-link-lib=static=injection");
    println!(
        "cargo:rustc-link-search={}",
        build_parent_dir.join("src").display()
    );

    build_parent_dir.push("src");

    let h_path = build_parent_dir.join("libinjection.h");
    let bindings = bindgen::Builder::default()
        .header(h_path.to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings");

    Ok(())
}
