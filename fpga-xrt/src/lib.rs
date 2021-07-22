use fpga_core::Platform;
use log::{error, info};
use std::{
    ffi::{c_void, CStr, CString},
    io::Error,
    ops::Not,
    os::raw::c_int,
    ptr::{self, NonNull},
    sync::Once,
};
use uuid::Uuid;

// todo(mb): visibility
pub mod bindings;
use bindings::{
    xclDeviceHandle, xclDeviceInfo2, xclGetDeviceInfo2, xclProbe, xrtDeviceClose,
    xrtDeviceGetXclbinUUID, xrtDeviceHandle, xrtDeviceOpen, xrtDeviceOpenByBDF,
    xrtDeviceToXclDevice, xrtIniStringSet,
};

pub type Result<T> = std::result::Result<T, Error>;

/// One-time global initialization for Xrt.
static INIT: Once = Once::new();

/// Number of XRT devices in the system. Populated during initialization.
static mut XRT_DEVICES: Result<usize> = Ok(0); // this should default to Err(Error::from_raw_os_error(libc::ENODEV));

// Key-value pair to disable runtime logging by XRT via xrt.ini.
static RUNTIME_LOG_KEY: &[u8] = b"Runtime.runtime_log\0";
static RUNTIME_LOG_VALUE: &[u8] = b"null\0";

#[derive(Debug)]
pub struct Xrt {
    /// Handle to XRT device. Always non-null.
    handle: NonNull<c_void>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Configuration {
    FirstDevice,
    DeviceIndex(usize),
    BusDeviceFunction(String),
}

impl Platform for Xrt {
    const NAME: &'static str = "XRT";

    type Configuration = Configuration;
    type Error = Error;

    fn from_configuration(configuration: Self::Configuration) -> Result<Self> {
        match configuration {
            Configuration::FirstDevice => Self::new(),
            Configuration::DeviceIndex(device_index) => Self::from_device_index(device_index),
            Configuration::BusDeviceFunction(ref bdf) => Self::from_bus_device_function(bdf),
        }
    }
}

impl Drop for Xrt {
    fn drop(&mut self) {
        // Safety
        // - Handle is always non-null
        let result = unsafe { xrtDeviceClose(self.handle.as_ptr()) };

        // Drop should not panic so we ignore failed closing of the device.
        // Instead an error message is written.
        if result != 0 {
            error!("Failed to close xrt device");
        }
    }
}

fn init() {
    INIT.call_once(|| {
        if unsafe {
            // Disable runtime logging by XRT
            xrtIniStringSet(
                CStr::from_bytes_with_nul_unchecked(RUNTIME_LOG_KEY).as_ptr(),
                CStr::from_bytes_with_nul_unchecked(RUNTIME_LOG_VALUE).as_ptr(),
            )
        } != 0
        {
            panic!("Failed to update xrt.ini configuration to disable logging")
        }
        // Get number of XRT devices.
        unsafe {
            let n = xclProbe();
            if n != 0 {
                XRT_DEVICES = Ok(n as usize);
            }
        }
    });
}

fn errno() -> Error {
    Error::from_raw_os_error({
        let errno = unsafe { libc::__errno_location() };
        if errno.is_null() {
            // Pointer returned by __errno_location is null.
            libc::EFAULT
        } else {
            // Safety
            // - Checked for null pointer above.
            unsafe { ptr::read(errno) }
        }
    })
}

fn check(result: c_int) -> Result<()> {
    (result == 0).then(|| ()).ok_or_else(errno)
}

/// Checks if given handle is non-null. Returns None when handle is null.
fn check_device_handle(handle: xrtDeviceHandle) -> Result<Xrt> {
    handle
        .is_null()
        .not()
        .then(||
            // Safety:
            // - Checked that pointer is non-null.
            Xrt {
                handle: unsafe { NonNull::new_unchecked(handle) }
            })
        .ok_or_else(|| Error::from_raw_os_error(libc::ENODEV))
}

impl Xrt {
    /// Returns the number of XRT devices in the system.
    pub fn num_devices() -> usize {
        init();
        // Safety
        // - Value set by init above.
        unsafe { XRT_DEVICES.unwrap_or(0) }
    }

    pub fn new() -> Result<Self> {
        init();
        // Init checks that there is at least one device.
        Self::from_device_index(0)
    }

    pub fn from_device_index(device_index: usize) -> Result<Self> {
        #[cold]
        #[inline(never)]
        fn assert_failed(device_index: usize) -> ! {
            panic!(
                "device_index (is {}) should be < number of XRT devices (is {})",
                device_index,
                Xrt::num_devices()
            );
        }

        // Safety
        // - Value set by init above.
        if device_index >= Self::num_devices() {
            assert_failed(device_index);
        }

        check_device_handle(unsafe { xrtDeviceOpen(device_index as u32) })
    }

    pub fn from_bus_device_function<T>(bus_device_function: T) -> Result<Self>
    where
        T: AsRef<str>,
    {
        init();
        let bdf = CString::new(bus_device_function.as_ref())?;
        check_device_handle(unsafe { xrtDeviceOpenByBDF(bdf.as_ptr()) })
    }

    fn xcl_handle(&self) -> Result<xclDeviceHandle> {
        let handle = unsafe { xrtDeviceToXclDevice(self.handle.as_ptr()) };
        handle.is_null().not().then(|| handle).ok_or_else(errno)
    }

    pub fn xclbin_uuid(&self) -> Option<Uuid> {
        // Initialize some bytes on the stack to store the Uuid.
        let mut uuid_bytes: uuid::Bytes = [0; 16];

        // Safety
        // - Handle is always non-null.
        // - Uuid bytes initialized above.
        check(unsafe { xrtDeviceGetXclbinUUID(self.handle.as_ptr(), uuid_bytes.as_mut_ptr()) })
            .ok()?;

        // 0 indicates no xclbin is loaded on the device.
        (Uuid::from_bytes(uuid_bytes) == Uuid::nil())
            .not()
            .then(|| Uuid::from_bytes(uuid_bytes))
    }

    pub fn info(&self) -> Result<()> {
        let mut info = xclDeviceInfo2::default();
        check(unsafe { xclGetDeviceInfo2(self.xcl_handle()?, &mut info) })?;
        info!("{:#?}", &info);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // todo(mb): cfg_if based on num of devices
    #[test]
    fn device() {
        let device = Xrt::from_configuration(Configuration::FirstDevice);
    }
}
