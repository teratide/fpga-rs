use std::error::Error;

pub trait Platform: Sized {
    const NAME: &'static str;

    /// Platform specific configuration. Required to construct this platform.
    type Configuration;

    /// Platform specific error type.
    type Error: Error;

    /// Constructs and initializes this platform using the provided platform configuration.
    fn from_configuration(configuration: Self::Configuration) -> Result<Self, Self::Error>;
}
