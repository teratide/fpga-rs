use crate::{
    bindings::{fpgaCloneToken, fpgaDestroyToken, fpga_token},
    Result,
};
use log::{error, trace};
use std::{ops::Deref, ptr};

#[derive(Debug)]
pub struct Token(fpga_token);

impl Token {
    pub fn from_raw(token: fpga_token) -> Self {
        Self(token)
    }
}

impl Deref for Token {
    type Target = fpga_token;

    fn deref(&self) -> &fpga_token {
        &self.0
    }
}

// panics when cloning fails
impl Clone for Token {
    fn clone(&self) -> Self {
        let mut token = ptr::null_mut();
        Result::from(unsafe { fpgaCloneToken(self.0, &mut token) }).unwrap();
        Self(token)
    }
}

impl Drop for Token {
    fn drop(&mut self) {
        trace!("Dropping Token: {:p}", self.0);
        if Result::from(unsafe { fpgaDestroyToken(&mut self.0) }).is_err() {
            error!("Failed to destroy token object");
        }
    }
}
