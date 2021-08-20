pub use fpga_core::*;

#[cfg(feature = "xrt")]
pub use fpga_xrt as xrt;

#[cfg(feature = "opae")]
pub use fpga_opae as opae;

pub fn discover() -> Option<Box<dyn Platform>> {
    #[cfg(feature = "xrt")]
    if let Some(platform) = xrt::Xrt::new().ok() {
        return Some(Box::new(platform) as Box<dyn Platform>);
    }

    #[cfg(feature = "opae")]
    if let Some(platform) = opae::Opae::new().ok() {
        return Some(Box::new(platform) as Box<dyn Platform>);
    }

    None
}
