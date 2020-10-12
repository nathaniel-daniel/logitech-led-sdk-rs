# logitech-led-sdk-rs
A Rust Binding for the Logitech LED Library. Tested with v9.00.

# Building
You need to seperately download the LG SDK. You can find it publicly [here](https://www.logitechg.com/en-us/innovation/developer-lab.html).

Set the environment variable `LOGITECH_LED_SDK` to the `LED` folder from the downloaded zip sdk.

Example: `LOGITECH_LED_SDK = C:\Users\[username]\Documents\code\LED_SDK_9.00\LED`.

# Example
```rust
use logitech_led_sdk::Sdk;

fn main() {
    let sdk = Sdk::new_with_name("Test").unwrap();
    
    let version = sdk.get_version().unwrap();
    println!("Version: {:#?}", version);
    
    assert!(sdk.set_target(Target::All));
    assert!(sdk.set_lighting(Color::new(255, 255, 255)));
}
```
