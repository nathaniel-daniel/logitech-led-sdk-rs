# logitech-led-sdk-rs
A Rust Binding for the Logitech LED Library. This was built and tested with v9.00. It will most likely work with LGS and definitely works with LG HUB. 
Due to a lack of documentation, the SDK is assumed to be single-threaded. The library will use internal synchronization to ensure accesses to the library are safe. The goal of this binding is to provide a wrapper for the SDK, not to provide a method to interact with LEDs on Logitech devices. 

## Documentation
See <https://nathaniel-daniel.github.io/logitech-led-sdk-rs/logitech_led_sdk/>

## Building
You need to seperately download the LG SDK. You can find it publicly [here](https://www.logitechg.com/en-us/innovation/developer-lab.html).

Set the environment variable `LOGITECH_LED_SDK` to the `LED` folder from the downloaded zip sdk.

Example: `LOGITECH_LED_SDK = C:\Users\[username]\Documents\code\LED_SDK_9.00\LED`.

## Example
```rust
use logitech_led_sdk::Sdk;
use logitech_led_sdk::Color;
use logitech_led_sdk::Target;

fn main() {
    let sdk = Sdk::new_with_name("Test").unwrap();
    
    let version = sdk.get_version().unwrap();
    println!("Version: {:#?}", version);
    
    assert!(sdk.set_target(Target::All));
    assert!(sdk.set_lighting(Color::new(255, 255, 255)));
}
```

## License
This crate is dual-licensed under [Apache](./LICENSE-APACHE) and [MIT](LICENSE-MIT).
