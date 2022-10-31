use std::path::PathBuf;

fn main() {
    let sdk_dir = std::env::var("LOGITECH_LED_SDK").expect("missing or invalid `LOGITECH_LED_SDK`");
    let include_dir = format!("{}//Include", sdk_dir);
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
            println!("cargo:rustc-link-search={}/Lib/x64", sdk_dir);
        }
        "x86" => {
            println!("cargo:rustc-link-search={}/Lib/x86", sdk_dir);
        }
        arch => {
            panic!("Arch `{}` is not supported", arch);
        }
    };

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=LOGITECH_LED_SDK");
    println!("cargo:rustc-link-lib=LogitechLEDLib");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-xc++")
        .clang_arg(format!("-I{}", include_dir))
        .allowlist_type("LogiLed::.*")
        .allowlist_function("Logi.*")
        .allowlist_var(".*")
        .rustified_enum("LogiLed::.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
