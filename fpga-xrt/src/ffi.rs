#[cxx::bridge]
mod ffi {

    // xrt_device.h
    #[namespace = "xrt"]
    unsafe extern "C++" {
        include!("src/ffi.h");

        type Device;
        fn new_device(index: u32) -> Result<UniquePtr<Device>>;
        fn new_device_bdf(bdf: &str) -> Result<UniquePtr<Device>>;

        // Modified members
        fn xclbin_uuid(self: &Device) -> [u8; 16];
        fn load(self: Pin<&mut Device>, xclbin: &Xclbin) -> [u8; 16];

        // Custom members
        fn bdf(self: &Device) -> &str;
        fn interface_uuid(self: &Device) -> [u8; 16];
        fn kdma(self: &Device) -> u32;
        fn max_clock_frequency_mhz(self: &Device) -> u64;
        fn m2m(self: &Device) -> bool;
        fn name(self: &Device) -> &str;
        fn nodma(self: &Device) -> bool;
        fn offline(self: &Device) -> bool;
        // fn electrical(self: &Device) -> String;
        // fn thermal(self: &Device) -> String;
        // fn mechanical(self: &Device) -> String;
        // fn memory(self: &Device) -> String;
        // fn platform(self: &Device) -> String;
        // fn pcie_info(self: &Device) -> String;
        // fn host(self: &Device) -> String;
        // fn dynamic_regions(self: &Device) -> String;
    }

    // xrt_xclbin.h
    #[namespace = "xrt"]
    unsafe extern "C++" {
        include!("src/ffi.h");

        type XclbinArg;
        fn name(self: &XclbinArg) -> &str;
        fn mems(self: &XclbinArg) -> UniquePtr<CxxVector<XclbinMem>>;
        
        type XclbinIp;
        fn name(self: &XclbinIp) -> &str;
        #[rust_name = "num_args"]
        fn get_num_args(self: &XclbinIp) -> usize;
        fn args(self: &XclbinIp) -> UniquePtr<CxxVector<XclbinArg>>;
        fn arg(self: &XclbinIp, index: i32) -> UniquePtr<XclbinArg>;
        #[rust_name = "base_address"]
        fn get_base_address(self: &XclbinIp) -> u64;

        type XclbinKernel;
        fn name(self: &XclbinKernel) -> &str;
        
        type XclbinMem;

        type Xclbin;
        fn new_xclbin(bytes: &[i8]) -> Result<UniquePtr<Xclbin>>;
        fn kernels(self: &Xclbin) -> UniquePtr<CxxVector<XclbinKernel>>;
        fn kernel(self: &Xclbin, name: &str) -> UniquePtr<XclbinKernel>;
        fn ips(self: &Xclbin) -> UniquePtr<CxxVector<XclbinIp>>;
        fn ip(self: &Xclbin, name: &str) -> UniquePtr<XclbinIp>;
        fn xsa_name(self: &Xclbin) -> &str;
        fn uuid(self: &Xclbin) -> [u8; 16];
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
        
        type kernel_cu_access_mode;

        type kernel;
        fn new_kernel(
            device: &Device,
            xclbin_id: [u8; 16],
            name: &str,
            cu_access_mode: kernel_cu_access_mode,
        ) -> Result<UniquePtr<kernel>>;
        fn group_id(self: &kernel, argno: i32) -> i32;
        fn offset(self: &kernel, argno: i32) -> u32;
        fn read_register(self: &kernel, offset: u32) -> u32;
        fn write_register(self: Pin<&mut kernel>, offset: u32, data: u32) -> Result<()>;
    }

    // xrt_ip.h
    #[namespace = "xrt"]
    unsafe extern "C++" {
        include!("src/ffi.h");
        
        type ip;
        fn new_ip(
            device: &Device,
            xclbin_id: [u8; 16],
            name: &str
        ) -> Result<UniquePtr<ip>>;
        fn read_register(self: &ip, offset: u32) -> u32;
        fn write_register(self: Pin<&mut ip>, offset: u32, data: u32) -> Result<()>;
    }

    // xrt_ini.h
    #[namespace = "xrt"]
    unsafe extern "C++" {
        include!("src/ffi.h");

        fn set_ini(key: &str, value: &str) -> Result<()>;
    }
}

pub use ffi::*;
