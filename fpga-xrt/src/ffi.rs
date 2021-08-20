#[cxx::bridge]
mod ffi {

    // xrt_device.h
    #[namespace = "xrt"]
    unsafe extern "C++" {
        include!("src/ffi.h");

        type Device;

        // Constructor
        fn new_device(index: u32) -> Result<UniquePtr<Device>>;

        // Members
        fn xclbin_uuid(self: &Device) -> [u8; 16];
        fn load_xclbin(self: Pin<&mut Device>, xclbin: &Xclbin) -> [u8; 16];

        // Custom members
        fn bdf(self: &Device) -> String;
        fn interface_uuid(self: &Device) -> [u8; 16];
        fn kdma(self: &Device) -> u32;
        fn max_clock_frequency_mhz(self: &Device) -> u64;
        fn m2m(self: &Device) -> bool;
        fn name(self: &Device) -> String;
        fn nodma(self: &Device) -> bool;
        fn offline(self: &Device) -> bool;
        fn electrical(self: &Device) -> String;
        fn thermal(self: &Device) -> String;
        fn mechanical(self: &Device) -> String;
        fn memory(self: &Device) -> String;
        fn platform(self: &Device) -> String;
        fn pcie_info(self: &Device) -> String;
        fn host(self: &Device) -> String;
        fn dynamic_regions(self: &Device) -> String;
    }

    // xrt_xclbin.h
    #[namespace = "xrt"]
    unsafe extern "C++" {
        include!("src/ffi.h");

        type xclbin_kernel;

        // todo(mb): extend
        fn xclbin_kernel_get_name(kernel: &xclbin_kernel) -> String;

        type xclbin_ip;

        // todo(mb): extend
        fn xclbin_ip_get_name(ip: &xclbin_ip) -> String;

        type Xclbin;

        // Constructor
        fn new_xclbin(bytes: &[i8]) -> Result<UniquePtr<Xclbin>>;

        // Members
        fn get_kernels(self: &Xclbin) -> UniquePtr<CxxVector<xclbin_kernel>>;
        fn get_ips(self: &Xclbin) -> UniquePtr<CxxVector<xclbin_ip>>;
        fn get_xsa_name(self: &Xclbin) -> String;
        fn get_uuid(self: &Xclbin) -> [u8; 16];
    }

    #[namespace = "xrt"]
    enum kernel_cu_access_mode {
        exclusive,
        shared,
        none,
    }

    // xrt_kernel.h
    #[namespace = "xrt"]
    unsafe extern "C++" {
        include!("src/ffi.h");

        type kernel;
        type kernel_cu_access_mode;

        // Constructor.
        fn new_kernel(
            device: &Device,
            xclbin_id: [u8; 16],
            name: String,
            cu_access_mode: kernel_cu_access_mode,
        ) -> Result<UniquePtr<kernel>>;

        // Methods.
        fn read_register(self: &kernel, offset: u32) -> u32;
    }
}

pub use ffi::*;
