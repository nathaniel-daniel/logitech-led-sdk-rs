use std::path::PathBuf;

fn main() {
    let sdk_dir = std::env::var("LOGITECH_LED_SDK").expect("missing or invalid `LOGITECH_LED_SDK`");
    let include_dir = format!("{sdk_dir}//Include");
    let out_dir = std::env::var_os("OUT_DIR").expect("missing `OUT_DIR`");
    let out_path = PathBuf::from(out_dir);

    if std::env::var_os("CARGO_CFG_WINDOWS").is_none() {
        panic!("This library will only work on Windows");
    }

    match std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("missing or invalid `CARGO_CFG_TARGET_ARCH`")
        .as_str()
    {
        "x86_64" => {
            println!("cargo:rustc-link-search={sdk_dir}/Lib/x64");
        }
        "x86" => {
            println!("cargo:rustc-link-search={sdk_dir}/Lib/x86");
        }
        arch => {
            panic!("Arch \"{arch}\" is not supported");
        }
    };

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=LOGITECH_LED_SDK");
    println!("cargo:rustc-link-lib=LogitechLEDLib");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-xc++")
        .clang_arg(format!("-I{include_dir}"))
        .allowlist_type("LogiLed::.*")
        .allowlist_function("Logi.*")
        .allowlist_var(".*")
        .rustified_enum("LogiLed::.*")
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");
}
