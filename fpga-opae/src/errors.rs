use crate::bindings::{fpgaErrStr, fpga_result};
use std::ffi::CStr;

/// Error wrapper for `bindings::fpga_result`.
#[derive(Copy, Clone, Debug)]
pub struct Error(fpga_result);

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            unsafe { CStr::from_ptr(fpgaErrStr(self.0)) }.to_string_lossy()
        )
    }
}

/// Result type with `Error` error type.
pub type Result<T> = std::result::Result<T, Error>;

impl From<fpga_result> for Result<()> {
    fn from(result: fpga_result) -> Self {
        match result {
            fpga_result::FPGA_OK => Ok(()),
            _ => Err(Error(result)),
        }
    }
}

impl From<fpga_result> for Error {
    fn from(fpga_result: fpga_result) -> Self {
        Self(fpga_result)
    }
}
