use crate::ffi::{self, new_xclbin, xclbin_ip_get_name, xclbin_kernel_get_name};
use cxx::UniquePtr;
use std::{fmt::Debug, fs, path::Path};
use uuid::Uuid;

pub struct Xclbin {
    pub(crate) xclbin: UniquePtr<ffi::Xclbin>,
}

impl Debug for Xclbin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Xclbin")
            .field("xsa_name", &self.get_xsa_name())
            .field("uuid", &self.get_uuid())
            .finish()
    }
}

impl Xclbin {
    pub fn get_xsa_name(&self) -> String {
        self.xclbin.get_xsa_name()
    }

    pub fn get_kernels(&self) {
        self.xclbin.get_kernels().iter().for_each(|kernel| {
            println!("{}", xclbin_kernel_get_name(&kernel));
        });
    }

    pub fn get_ips(&self) {
        self.xclbin.get_ips().iter().for_each(|ip| {
            println!("{}", xclbin_ip_get_name(&ip));
        });
    }

    pub fn get_uuid(&self) -> Uuid {
        Uuid::from_bytes(self.xclbin.get_uuid())
    }
}

impl Xclbin {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, std::io::Error> {
        // Attempt to read the file.
        let data = fs::read(path)?;

        // Cast to i8, because that is what the C++ api expects.
        // Prevent destructor.
        let mut data = std::mem::ManuallyDrop::new(data);
        // Cast.
        // todo(mb): cast in c++ (don't know how without a copy)
        let input = unsafe {
            Vec::from_raw_parts(data.as_mut_ptr() as *mut i8, data.len(), data.capacity())
        };
        Ok(Self {
            xclbin: new_xclbin(&input).map_err(|exception| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, exception.what())
            })?,
        })
    }
}
