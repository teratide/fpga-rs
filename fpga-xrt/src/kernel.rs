use crate::ffi;
use cxx::UniquePtr;

pub struct Kernel {
    pub(crate) kernel: UniquePtr<ffi::kernel>,
}
