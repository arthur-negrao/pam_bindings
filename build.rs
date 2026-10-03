fn main() {
    println!("cargo:rerun-if-changed=pam_wrapper.h");

    let library = pkg_config::Config::new()
        .atleast_version("1.3")
        .probe("pam")
        .expect("Fail to find the a `libpam` with pkg-config. Verify if the PAM development package is already installed.");

    let mut builder = bindgen::Builder::default()
        .header("pam_wrapper.h")
        .use_core()
        .allowlist_function("pam_start")
        .allowlist_function("pam_authenticate")
        .allowlist_function("pam_acct_mgmt")
        .allowlist_function("pam_open_session")
        .allowlist_function("pam_close_session")
        .allowlist_function("pam_end")
        .allowlist_var("PAM_.*")
        .opaque_type("pam_handle_t")
        .allowlist_function("pam_getenv")
        .allowlist_function("pam_getenvlist")
        .allowlist_function("pam_putenv");

    for include_path in library.include_paths {
        builder = builder.clang_arg(format!("-I{}", include_path.display()));
    }

    let bindings = builder
        .generate()
        .expect("Fail to generate the PAM bindings.");

    let out_path = std::path::PathBuf::from(
        std::env::var("OUT_DIR").expect("Fail to get the environment variable `OUT_DIR`."),
    );

    bindings
        .write_to_file(out_path.join("pam_bindings.rs"))
        .expect("Fail to save generated bindings.");
}
