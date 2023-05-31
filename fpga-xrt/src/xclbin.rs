use crate::ffi::{self, new_xclbin};
use cxx::UniquePtr;
use std::{fmt::Debug, fs, path::Path};
use uuid::Uuid;

pub struct Xclbin {
    pub(crate) xclbin: UniquePtr<ffi::Xclbin>,
}

impl Debug for Xclbin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Xclbin")
            .field("xsa_name", &self.xsa_name())
            .field("uuid", &self.uuid())
            .finish()
    }
}

impl Xclbin {
    pub fn xsa_name(&self) -> &str {
        self.xclbin.xsa_name()
    }

    pub fn kernels(&self) {
        dbg!(self.xclbin.kernels().len());
        // self.xclbin.kernels().iter().for_each(|kernel| {
        //     println!("{}", kernel.name());
        // });
    }

    pub fn ips(&self) {
        dbg!(self.xclbin.ips().len());
        // .iter().for_each(|ip| {
        //     println!("{}", ip.name());
        // });
    }

    pub fn uuid(&self) -> Uuid {
        Uuid::from_bytes(self.xclbin.uuid())
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
