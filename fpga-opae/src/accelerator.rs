use crate::{
    AcceleratorProperties, Device, Filter, Properties, Resource, ResourceInfo, ResourceProperties,
    Token,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Accelerator {
    token: Token,
    properties: Properties,
}

impl Accelerator {
    pub fn new(token: Token, properties: Properties) -> Self {
        Accelerator { token, properties }
    }

    pub fn device(&self) -> Option<Device> {
        self.device_id().ok().and_then(|device_id| {
            Filter::new()
                .with_device_object()
                .with_device_id(device_id)
                .into_iter()
                .next()
                // Safety:
                // - Used device_object on filter, so resource can't be accelerator.
                .map(Resource::unwrap_device)
        })
    }

    pub fn info(&self) -> AcceleratorInfo {
        AcceleratorInfo {
            resource: self.into(),
            assigned: self.is_assigned().ok(),
            num_error_registers: self.num_error_registers().ok(),
            num_interrupts: self.num_interrupts().ok(),
            num_mmio_spaces: self.num_mmio_spaces().ok(),
        }
    }
}

impl ResourceProperties for Accelerator {
    fn properties(&self) -> &Properties {
        &self.properties
    }
}

impl ResourceProperties for &Accelerator {
    fn properties(&self) -> &Properties {
        &self.properties
    }
}

impl AcceleratorProperties for Accelerator {}

pub struct AcceleratorInfo {
    resource: ResourceInfo,
    assigned: Option<bool>,
    num_error_registers: Option<u32>,
    num_interrupts: Option<u32>,
    num_mmio_spaces: Option<u32>,
}

impl Debug for AcceleratorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("AcceleratorInfo");

        debug.field("resource", &self.resource);
        if let Some(assigned) = self.assigned {
            debug.field("assigned", &assigned);
        }
        if let Some(num_error_registers) = self.num_error_registers {
            debug.field("num_error_registers", &num_error_registers);
        }
        if let Some(num_interrupts) = self.num_interrupts {
            debug.field("num_interrupts", &num_interrupts);
        }
        if let Some(num_mmio_spaces) = self.num_mmio_spaces {
            debug.field("num_mmio_spaces", &num_mmio_spaces);
        }

        debug.finish()
    }
}
