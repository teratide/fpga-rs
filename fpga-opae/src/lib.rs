use fpga_core::Platform;
use std::fmt::Debug;

pub mod bindings;
use bindings::{fpga_objtype, fpga_result};

mod device;
pub use device::*;

mod resource;
pub use resource::*;

mod accelerator;
pub use accelerator::*;

mod filter;
pub use filter::*;

mod properties;
pub use properties::*;

mod token;
pub use token::*;

mod errors;
pub use errors::*;

pub struct Opae {
    device: Option<Device>,
    accelerator: Accelerator,
}

impl Opae {
    /// Attempt to get the first accelerator object returned by enumeration of the system.
    pub fn new() -> Result<Self> {
        Self::from_filter(Filter::new().with_accelerator_object())
    }

    /// Returns a new Opae. The filter must select an accelerator object.
    pub fn from_filter(filter: Filter) -> Result<Self> {
        match filter.obj_type {
            Some(fpga_objtype::FPGA_DEVICE) => {
                // To construct this platform the filter should select an accelerator.
                Err(fpga_result::FPGA_INVALID_PARAM.into())
            }
            Some(fpga_objtype::FPGA_ACCELERATOR) => filter
                .into_iter()
                .next()
                .ok_or_else(|| fpga_result::FPGA_NOT_FOUND.into())
                .map(|resource| match resource {
                    Resource::Accelerator(accelerator) => Self {
                        device: accelerator.device(),
                        accelerator,
                    },
                    // Safety:
                    // - Filter is selecting accelerators as checked above.
                    Resource::Device(_) => unreachable!(),
                }),
            None => {
                // Add accelerator object type to filter.
                Self::from_filter(filter.with_accelerator_object())
            }
        }
    }

    pub fn accelerator_info(&self) -> AcceleratorInfo {
        self.accelerator.info()
    }

    pub fn device_info(&self) -> Option<DeviceInfo> {
        self.device.as_ref().map(|device| device.info())
    }
}

impl Debug for Opae {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Opae");

        if let Some(ref device) = self.device {
            debug.field("device", &device.info());
        }

        debug
            .field("accelerator", &self.accelerator.info())
            .finish()
    }
}

impl Platform for Opae {
    const NAME: &'static str = "OPAE";

    type Configuration = Filter;
    type Error = Error;

    fn from_configuration(configuration: Self::Configuration) -> Result<Self> {
        Self::from_filter(configuration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_filter() {
        assert!(Opae::from_filter(Filter::new().with_device_object()).is_err());
    }
}
