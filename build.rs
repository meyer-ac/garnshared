use std::{env, fs, io};
use std::path::{Path, PathBuf};
use std::process::Command;

fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".git" {
            continue;
        }
        let (from, to) = (entry.path(), dst.join(&name));
        if entry.file_type()?.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn run(cmd: &mut Command) {
    let status = cmd.status().expect("failed to spawn command");
    assert!(status.success(), "command failed: {:?}", cmd);
}

fn main() {
    let /*mut*/ cc_build = cc::Build::new();

    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    let src = manifest.join("vendor/ck");
    let build = out.join("ck-build");
    let install = out.join("ck-install");
    let include = install.join("include");

    let ck_configure = src.join("configure");
    if !ck_configure.exists() {
        panic!("{} does not exists. Run `git submodule update --init`.", ck_configure.display());
    }

    let _ = fs::remove_dir_all(&build);
    copy_dir(&src, &build).expect("copy ck sources");

    run(Command::new("./configure")
        .current_dir(&build)
        .env_remove("PROFILE")
        .env("CC", cc_build.get_compiler().path())
        .arg(format!("--prefix={}", install.display())));
    run(Command::new("make")
        .current_dir(&build));
    run(Command::new("make")
        .current_dir(&build)
        .arg("install"));

    println!("cargo:rustc-link-search=native={}", install.join("lib").display());
    println!("cargo:rustc-link-lib=static=ck");
    println!("cargo:include={}", include.display());

    let bindings = bindgen::Builder::default()
        .header(include.join("ck_ring.h").to_str().unwrap())
        .clang_arg(format!("-I{}", include.display()))
        .allowlist_function("ck_.*")
        .allowlist_type("ck_.*")
        .allowlist_var("CK_.*")
        .wrap_static_fns(true)
        //.wrap_static_fns_path("extern")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("generate ck bindings");

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("write ck bindings");

    //cc_build
        //.file(out.join("extern.c"))
        //.include(".")
        //.include(include)
        //.compile("ck_extern");
}