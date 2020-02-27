use std::path::PathBuf;

fn main() {
    let sdk_dir = std::env::var("LOGITECH_LED_SDK").expect("LOGITECH_LED_SDK");
    let include_dir = format!("{}//Include", sdk_dir);
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir);

    if std::env::var("CARGO_CFG_WINDOWS").is_err() {
        panic!("This library will only work on Windows");
    }

    match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => {
            println!("cargo:rustc-link-search={}/Lib/x64", sdk_dir);
        }
        "x86" => {
            println!("cargo:rustc-link-search={}/Lib/x86", sdk_dir);
        }
        arch => {
            panic!("Arch '{}' is not supported", arch);
        }
    };

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=LOGITECH_LED_SDK");
    println!("cargo:rustc-link-lib=LogitechLEDLib");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-xc++")
        .clang_arg(&format!("-I{}", include_dir))
        .whitelist_type("LogiLed::.*")
        .whitelist_function("Logi.*")
        .whitelist_var(".*")
        .rustified_enum("LogiLed::.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
