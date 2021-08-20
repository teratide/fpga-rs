use std::pin::Pin;

use cxx::{Exception, UniquePtr};
use fpga_core::{Platform, PlatformType, Power, Program, Thermal};
use uuid::Uuid;

pub(crate) mod ffi;

mod info;
pub use info::*;

mod xclbin;
pub use xclbin::*;

mod kernel;
pub use kernel::*;

type Result<T> = std::result::Result<T, Exception>;

pub struct Xrt {
    device: UniquePtr<ffi::Device>,
}

impl Xrt {
    pub fn kernel(&self, uuid: Uuid, name: &str) -> Kernel {
        Kernel {
            kernel: ffi::new_kernel(
                &self.device,
                *uuid.as_bytes(),
                name.to_string(),
                ffi::kernel_cu_access_mode::exclusive,
            )
            .unwrap(),
        }
    }
}

pub enum Configuration {
    DeviceIndex(usize),
    Bdf(String),
}

impl Platform for Xrt {
    type Configuration = Configuration;
    type Error = Exception;

    fn from_configuration(configuration: Self::Configuration) -> Result<Self> {
        match configuration {
            Configuration::DeviceIndex(index) => Self::from_device_index(index),
            Configuration::Bdf(_) => todo!(),
        }
    }

    fn platform(&self) -> PlatformType {
        PlatformType::XRT
    }
}

impl Program for Xrt {
    type Source = Xclbin;
    type Output = Uuid;

    fn program(&mut self, source: Self::Source) -> Result<Self::Output> {
        Ok(Uuid::from_bytes(
            self.device.pin_mut().load_xclbin(&source.xclbin),
        ))
    }
}

impl Power for Xrt {
    fn power(&self) -> f32 {
        self.electrical().power_consumption_watts
    }
}

impl Thermal for Xrt {
    fn temperature(&self) -> f32 {
        self.thermal()
            .into_iter()
            .find(|thermal| thermal.location_id == "fpga0")
            .map(|thermal| thermal.temp_c)
            .unwrap_or_default() as f32
    }
}

impl std::fmt::Debug for Xrt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Xrt")
            .field("bdf", &self.bdf())
            .field("interface_uuid", &self.interface_uuid())
            .field("kdma", &self.kdma())
            .field("max_clock_frequency_mhz", &self.max_clock_frequency_mhz())
            .field("m2m", &self.m2m())
            .field("name", &self.name())
            .field("nodma", &self.nodma())
            .field("offline", &self.offline())
            .field("electrical", &self.electrical())
            .field("thermal", &self.thermal())
            .field("mechanical", &self.mechanical())
            .field("memory", &self.memory())
            .field("platform", &self.platform())
            .field("pcie_info", &self.pcie_info())
            .field("host", &self.host())
            .field("dynamic_regions", &self.dynamic_regions())
            .finish()
    }
}

impl Xrt {
    pub fn new() -> Result<Self> {
        Self::from_device_index(0)
    }

    pub fn from_device_index(index: usize) -> Result<Self> {
        Ok(Self {
            device: ffi::new_device(index as u32)?,
        })
    }

    pub fn uuid(&self) -> Uuid {
        Uuid::from_bytes(self.device.xclbin_uuid())
    }
}
