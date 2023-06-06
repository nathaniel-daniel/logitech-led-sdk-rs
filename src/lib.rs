mod color_percent;
mod sdk;
mod target_device;

pub use self::color_percent::ColorPercent;
pub use self::sdk::Sdk;
pub use self::target_device::TargetDevice;
pub use logitech_led_sdk_sys as sys;
use std::sync::Mutex;
pub use sys::LogiLed_DeviceType as DeviceType;
pub use sys::LogiLed_KeyName as KeyName;

/// The lock that syncs accesses to the SDK.
///
/// If you use raw sdk api functions anywhere, you MUST use this lock to wrap accesses to the sdk in order to prevent data races.
/// This library does all this for you, this is exposed only for users who want to use raw sdk functions safely.
pub static SDK_LOCK: Mutex<()> = Mutex::new(());

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    // A simple lock to ensure that only one test runs as a time.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn sanity_check() {
        let _test_lock = TEST_LOCK.lock().expect("test lock poisoned");

        // 1st init
        let sdk = Sdk::new().expect("failed to init LG SDK");
        std::thread::sleep(Duration::from_secs(5));
        drop(sdk);
        std::thread::sleep(Duration::from_secs(5));

        // 2nd init
        let sdk = Sdk::new_with_name("Test").expect("failed to init LG SDK");

        // 3rd init fails, we already opened the 2nd.
        assert!(Sdk::new().is_none());

        std::thread::sleep(Duration::from_secs(5));
        let _version = sdk.get_version().expect("failed to get LG SDK version");
        assert!(sdk.set_target(TargetDevice::All));
        assert!(sdk.set_lighting(ColorPercent::new_rgb(255, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::L, ColorPercent::new_rgb(0, 255, 255)));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Mouse,
            1,
            ColorPercent::new_rgb(255, 0, 0)
        ));
        assert!(sdk.flash_lighting(
            ColorPercent::new_rgb(255, 0, 0),
            Some(Duration::from_millis(10_000)),
            Duration::from_millis(100)
        ));
        assert!(sdk.stop_effects());
        assert!(sdk.pulse_lighting(
            ColorPercent::new_rgb(255, 0, 0),
            Some(Duration::from_millis(10_000)),
            Duration::from_millis(100)
        ));
        assert!(sdk.stop_effects());
        assert!(sdk.set_lighting_for_key_with_scan_code(16, ColorPercent::new_rgb(255, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_hid_code(26, ColorPercent::new_rgb(255, 255, 255)));
        assert!(sdk.save_lighting_for_key(KeyName::L));
        assert!(sdk.restore_lighting_for_key(KeyName::L));
        assert!(sdk.flash_single_key(
            KeyName::L,
            ColorPercent::new_rgb(255, 0, 0),
            Some(Duration::from_millis(10_000)),
            Duration::from_millis(100)
        ));
        assert!(sdk.pulse_single_key(
            KeyName::L,
            ColorPercent::new_rgb(255, 0, 0),
            ColorPercent::new_rgb(255, 255, 0),
            Duration::from_millis(10_000),
            true
        ));
        assert!(sdk.stop_effects_on_key(KeyName::L));
        drop(sdk);
        std::thread::sleep(Duration::from_secs(5));
    }

    #[test]
    fn logi_set_target_zone_sample() {
        let _test_lock = TEST_LOCK.lock().expect("test lock poisoned");

        let sdk = Sdk::new_with_name("Test").expect("failed to init LG SDK");
        std::thread::sleep(Duration::from_secs(5));
        assert!(sdk.set_target(TargetDevice::All));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::L, ColorPercent::new_rgb(0, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::O, ColorPercent::new_rgb(0, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::G, ColorPercent::new_rgb(0, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::I, ColorPercent::new_rgb(0, 255, 255)));

        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Mouse,
            1,
            ColorPercent::new_rgb(255, 0, 0)
        ));

        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            1,
            ColorPercent::new_rgb(255, 0, 0)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            2,
            ColorPercent::new_rgb(255, 255, 0)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            3,
            ColorPercent::new_rgb(0, 255, 0)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            4,
            ColorPercent::new_rgb(0, 255, 255)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            5,
            ColorPercent::new_rgb(0, 0, 255)
        ));

        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Headset,
            0,
            ColorPercent::new_rgb(255, 255, 255)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Headset,
            1,
            ColorPercent::new_rgb(255, 0, 255)
        ));
        drop(sdk);
        std::thread::sleep(Duration::from_secs(5));
    }
}
