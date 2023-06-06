fn main() {
    let sdk_dir = std::env::var("LOGITECH_LED_SDK").expect("missing or invalid `LOGITECH_LED_SDK`");

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
    println!("cargo:rustc-link-lib=LogitechLEDLib");
}
