# logitech-led-sdk-rs
A Rust Binding for the Logitech LED Library. 
This was built and tested with v9.00. 
It will most likely work with LGS and definitely works with LG HUB. 
Due to a lack of documentation, the SDK is assumed to be single-threaded. 
The library will use internal synchronization to ensure accesses to the library are safe. 
The goal of this binding is to provide a wrapper for the SDK, not to provide a method to interact with LEDs on Logitech devices. 

## Documentation
Release: https://docs.rs/logitech-led-sdk/latest/logitech_led_sdk/

Master: https://nathaniel-daniel.github.io/logitech-led-sdk-rs/logitech_led_sdk/

## Building
You need to seperately download the LG SDK. You can find it publicly [here](https://www.logitechg.com/en-us/innovation/developer-lab.html).

Set the environment variable `LOGITECH_LED_SDK` to the `LED` folder from the downloaded zip sdk.

Example: `LOGITECH_LED_SDK = C:\Users\[username]\Documents\code\LED_SDK_9.00\LED`.

## Example
```rust
use logitech_led_sdk::Sdk;
use logitech_led_sdk::ColorPercent;
use logitech_led_sdk::TargetDevice;

fn main() {
    /// Do not use `expect` in a real application.
    /// If this fails, its likely that the user does not have LGS or LG HUB installed.
    let sdk = Sdk::new_with_name("Test").expect("failed to initialize SDK");
    
    let version = sdk.get_version().expect("failed to get version");
    println!("Version: {:#?}", version);
    
    /// Do not use `assert!` in a real application.
    assert!(sdk.set_target(TargetDevice::All));
    assert!(sdk.set_lighting(ColorPercent::new_rgb(255, 255, 255)));
}
```

## Testing
On a PC with either LGS or LG HUB running, run:
```bash
cargo test
```

## License
Licensed under either of
 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.