#[cxx::bridge]
mod ffi {
    // xrt_device.h
    #[namespace = "xrt"]
    unsafe extern "C++" {
        include!("src/ffi.h");

        // Wrapper class for xrt::device. To handle get_info API.
        type Device;
        type uuid;

        // Constructor
        fn new_device(index: u32) -> Result<UniquePtr<Device>>;
        
        // Members
        fn xclbin_uuid(self: &Device) -> [u8; 16];

        // Custom members
        fn name(self: &Device) -> String;
        fn bdf(self: &Device) -> String;

    }

}

pub use ffi::*;
