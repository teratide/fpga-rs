use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlatformType {
    XRT,
    OPAE,
}

impl Display for PlatformType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::XRT => "xrt",
                Self::OPAE => "opae",
            }
        )
    }
}

pub trait Platform: Sized {
    /// Platform specific configuration. Required to construct this platform.
    type Configuration;

    /// Platform specific error type.
    type Error: Error;

    /// Returns specific [PlatformType]. Can be used when performing platform
    /// tasks e.g. selecting a bit stream compatible with this platform. Can
    /// also be used to downcast a trait object to a concrete type.
    fn platform(&self) -> PlatformType;

    /// Constructs and initializes this platform using the provided platform configuration.
    fn from_configuration(configuration: Self::Configuration) -> Result<Self, Self::Error>;
}

pub trait MMIO: Platform {
    fn read_mmio<const N: usize>(&self, offset: usize, len: usize) -> Result<[u8; N], Self::Error>;
    fn write_mmio<T>(&mut self, offset: usize, data: T) -> Result<(), Self::Error>
    where
        T: AsRef<[u8]>;
}

pub trait Power: Platform {
    /// Current power usage in Watts.
    fn power(&self) -> f32;
}

pub trait Thermal: Platform {
    /// Current temperature of device in degrees C.
    fn temperature(&self) -> f32;
    /// Current temperature of device in degrees F.
    fn temperature_f(&self) -> f32 {
        (self.temperature() * 9. / 5.) + 32.
    }
}

pub trait Program: Platform {
    type Source;
    type Output;
    fn program(&mut self, source: Self::Source) -> Result<Self::Output, Self::Error>;
}
