use cxx::{Exception, UniquePtr};
use uuid::Uuid;

mod ffi;
// pub mod bindings;

type Result<T> = std::result::Result<T, Exception>;

pub struct Xrt {
  device: UniquePtr<ffi::Device>,
}

impl Xrt {
  pub fn new() -> Result<Self> {
      Self::from_device_index(0)
  }

  pub fn from_device_index(index: usize) -> Result<Self> {
    Ok(Self {
        device: ffi::new_device(index as u32)?
    })
  }

  pub fn name(&self) -> String {
    self.device.name()
  }

  pub fn uuid(&self) -> Uuid {
    Uuid::from_bytes(self.device.xclbin_uuid())
  }

}
