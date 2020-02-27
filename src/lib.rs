use logitech_led_sdk_sys as sys;
use std::convert::TryInto;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::time::Duration;
pub use sys::LogiLed_DeviceType as DeviceType;
pub use sys::LogiLed_KeyName as KeyName;

/// RGB Color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Creates a new color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    fn percentage(self) -> (u32, u32, u32) {
        (
            u32::from(self.r) * 100 / 255,
            u32::from(self.g) * 100 / 255,
            u32::from(self.b) * 100 / 255,
        )
    }
}

/// Color Targets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    All,
    Monochrome,
    PerKeyRgb,
    Rgb,
}

impl Target {
    fn as_num(self) -> u32 {
        match self {
            Self::All => sys::LOGI_DEVICETYPE_ALL,
            Self::Monochrome => sys::LOGI_DEVICETYPE_MONOCHROME,
            Self::PerKeyRgb => sys::LOGI_DEVICETYPE_PERKEY_RGB,
            Self::Rgb => sys::LOGI_DEVICETYPE_RGB,
        }
    }
}

/// Entry to Api
pub struct Sdk(PhantomData<*const u8>);

impl Sdk {
    /// New instance with no name
    pub fn new() -> Option<Self> {
        let init = unsafe { sys::LogiLedInit() };
        if !init {
            return None;
        }

        Some(Sdk(PhantomData))
    }

    /// New instance with name. Also returns none if passed &str has no valid cstr rep
    pub fn new_with_name(name: &str) -> Option<Self> {
        let name = CString::new(name).ok()?;
        let init = unsafe { sys::LogiLedInitWithName(name.as_ptr()) };

        if !init {
            return None;
        }

        Some(Sdk(PhantomData))
    }

    /// Returns sdk version. Returns None on failure.
    pub fn get_version(&self) -> Option<(u32, u32, u32)> {
        let mut major = 0;
        let mut minor = 0;
        let mut build = 0;

        let valid = unsafe { sys::LogiLedGetSdkVersion(&mut major, &mut minor, &mut build) };

        if !valid {
            return None;
        }

        Some((
            major.try_into().unwrap(),
            minor.try_into().unwrap(),
            build.try_into().unwrap(),
        ))
    }

    /// Selects the target channels. Returns true if successful
    pub fn set_target(&self, target: Target) -> bool {
        unsafe { sys::LogiLedSetTargetDevice(target.as_num() as _) }
    }

    /// Sets the lighting. Returns true if successful.
    pub fn set_lighting(&self, color: Color) -> bool {
        let p = color.percentage();
        unsafe { sys::LogiLedSetLighting(p.0 as _, p.1 as _, p.2 as _) }
    }

    /// Sets the lighting for a keyboard key by name
    pub fn set_lighting_for_key_with_name(&self, key: KeyName, color: Color) -> bool {
        let p = color.percentage();
        unsafe { sys::LogiLedSetLightingForKeyWithKeyName(key as _, p.0 as _, p.1 as _, p.2 as _) }
    }

    /// Sets the lighting for a keyboard key by scan code
    pub fn set_lighting_for_key_with_scan_code(&self, code: u32, color: Color) -> bool {
        let p = color.percentage();
        unsafe {
            sys::LogiLedSetLightingForKeyWithScanCode(code as _, p.0 as _, p.1 as _, p.2 as _)
        }
    }

    /// Sets the lighting for a keyboard key by HID code
    pub fn set_lighting_for_key_with_hid_code(&self, code: u32, color: Color) -> bool {
        let p = color.percentage();
        unsafe { sys::LogiLedSetLightingForKeyWithHidCode(code as _, p.0 as _, p.1 as _, p.2 as _) }
    }

    /// Sets the lighting for a specific device's target zone
    pub fn set_lighting_for_target_zone(
        &self,
        device: DeviceType,
        zone: usize,
        color: Color,
    ) -> bool {
        let p = color.percentage();
        unsafe {
            sys::LogiLedSetLightingForTargetZone(
                device as _,
                zone as _,
                p.0 as _,
                p.1 as _,
                p.2 as _,
            )
        }
    }

    /// Returns None if the call fails or any of the time values are too large. Duration how long the flashes occur overall. The interval is the time between flashes.
    pub fn flash_lighting(&self, color: Color, duration: Duration, interval: Duration) -> bool {
        let p = color.percentage();
        let duration: c_int = match duration.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        let interval: c_int = match interval.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        unsafe { sys::LogiLedFlashLighting(p.0 as _, p.1 as _, p.2 as _, duration, interval) }
    }

    /// Returns None if the call fails or any of the time values are too large. Duration how long the flashes occur overall. The interval is the time between flashes.
    pub fn flash_single_key(
        &self,
        key: KeyName,
        color: Color,
        duration: Duration,
        interval: Duration,
    ) -> bool {
        let p = color.percentage();
        let duration: c_int = match duration.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        let interval: c_int = match interval.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        unsafe {
            sys::LogiLedFlashSingleKey(key as _, p.0 as _, p.1 as _, p.2 as _, duration, interval)
        }
    }

    /// Stops all current LED effects
    pub fn stop_effects(&self) -> bool {
        unsafe { sys::LogiLedStopEffects() }
    }

    /// Stops all LED effects on one key
    pub fn stop_effects_on_key(&self, key: KeyName) -> bool {
        unsafe { sys::LogiLedStopEffectsOnKey(key as _) }
    }

    /// Returns None if the call fails or any of the time values are too large. Duration how long the pulses occur overall. The interval is the time between pulses.
    pub fn pulse_lighting(&self, color: Color, duration: Duration, interval: Duration) -> bool {
        let p = color.percentage();
        let duration: c_int = match duration.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        let interval: c_int = match interval.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        unsafe { sys::LogiLedPulseLighting(p.0 as _, p.1 as _, p.2 as _, duration, interval) }
    }

    /// Returns None if the call fails or any of the time values are too large. Duration how long the pulses occur overall.
    pub fn pulse_single_key(
        &self,
        key: KeyName,
        start_color: Color,
        finish_color: Color,
        duration: Duration,
        is_infinite: bool,
    ) -> bool {
        let p = start_color.percentage();
        let p1 = finish_color.percentage();
        let duration: c_int = match duration.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        unsafe {
            sys::LogiLedPulseSingleKey(
                key as _,
                p.0 as _,
                p.1 as _,
                p.2 as _,
                p1.0 as _,
                p1.1 as _,
                p1.2 as _,
                duration,
                is_infinite,
            )
        }
    }

    /// Saves the current lighting config for the given key
    pub fn save_lighting_for_key(&self, key: KeyName) -> bool {
        unsafe { sys::LogiLedSaveLightingForKey(key as _) }
    }

    /// Saves the current lighting config for the given key
    pub fn restore_lighting_for_key(&self, key: KeyName) -> bool {
        unsafe { sys::LogiLedRestoreLightingForKey(key as _) }
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        unsafe {
            sys::LogiLedShutdown();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn sanity_check() {
        let sdk = Sdk::new().unwrap();
        drop(sdk);
        let sdk = Sdk::new_with_name("Test").unwrap();
        let _version = sdk.get_version().unwrap();
        assert!(sdk.set_target(Target::All));
        assert!(sdk.set_lighting(Color::new(255, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::L, Color::new(0, 255, 255)));
        assert!(sdk.set_lighting_for_target_zone(DeviceType::Mouse, 1, Color::new(255, 0, 0)));
        assert!(sdk.flash_lighting(
            Color::new(255, 0, 0),
            Duration::from_millis(10_000),
            Duration::from_millis(100)
        ));
        assert!(sdk.stop_effects());
        assert!(sdk.pulse_lighting(
            Color::new(255, 0, 0),
            Duration::from_millis(10_000),
            Duration::from_millis(100)
        ));
        assert!(sdk.stop_effects());
        assert!(sdk.set_lighting_for_key_with_scan_code(16, Color::new(255, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_hid_code(26, Color::new(255, 255, 255)));
        assert!(sdk.save_lighting_for_key(KeyName::L));
        assert!(sdk.restore_lighting_for_key(KeyName::L));
        assert!(sdk.flash_single_key(
            KeyName::L,
            Color::new(255, 0, 0),
            Duration::from_millis(10_000),
            Duration::from_millis(100)
        ));
        assert!(sdk.pulse_single_key(
            KeyName::L,
            Color::new(255, 0, 0),
            Color::new(255, 255, 0),
            Duration::from_millis(10_000),
            true
        ));
        assert!(sdk.stop_effects_on_key(KeyName::L,));
    }

    #[test]
    #[serial]
    fn logi_set_target_zone_sample() {
        let sdk = Sdk::new_with_name("Test").unwrap();
        assert!(sdk.set_target(Target::All));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::L, Color::new(0, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::O, Color::new(0, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::G, Color::new(0, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::I, Color::new(0, 255, 255)));

        assert!(sdk.set_lighting_for_target_zone(DeviceType::Mouse, 1, Color::new(255, 0, 0)));

        assert!(sdk.set_lighting_for_target_zone(DeviceType::Keyboard, 1, Color::new(255, 0, 0)));
        assert!(sdk.set_lighting_for_target_zone(DeviceType::Keyboard, 2, Color::new(255, 255, 0)));
        assert!(sdk.set_lighting_for_target_zone(DeviceType::Keyboard, 3, Color::new(0, 255, 0)));
        assert!(sdk.set_lighting_for_target_zone(DeviceType::Keyboard, 4, Color::new(0, 255, 255)));
        assert!(sdk.set_lighting_for_target_zone(DeviceType::Keyboard, 5, Color::new(0, 0, 255)));

        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Headset,
            0,
            Color::new(255, 255, 255)
        ));
        assert!(sdk.set_lighting_for_target_zone(DeviceType::Headset, 1, Color::new(255, 0, 255)));
    }
}
