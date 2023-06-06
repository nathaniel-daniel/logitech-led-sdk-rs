use crate::sys;

bitflags::bitflags! {
    /// Select target devices by color types.
    ///
    /// This is a bitflag, so feel free to select multiple at once.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TargetDevice: u32 {
        const Monochrome = sys::LOGI_DEVICETYPE_MONOCHROME;
        const PerKeyRgb = sys::LOGI_DEVICETYPE_PERKEY_RGB;
        const Rgb = sys::LOGI_DEVICETYPE_RGB;
        const All = sys::LOGI_DEVICETYPE_ALL;
    }
}
