use crate::ffi;
use cxx::UniquePtr;

pub struct Ip {
    pub(crate) ip: UniquePtr<ffi::ip>,
}
