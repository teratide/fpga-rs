use crate::{
    bindings::fpga_version, DeviceProperties, Properties, ResourceInfo, ResourceProperties, Token,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Device {
    token: Token,
    properties: Properties,
}

impl Device {
    pub fn new(token: Token, properties: Properties) -> Self {
        Device { token, properties }
    }

    pub fn info(&self) -> DeviceInfo {
        DeviceInfo {
            resource: self.into(),
            bbs_id: self.bbs_id().ok(),
            bbs_version: self.bbs_version().ok(),
            capabilities: self.capabilities().ok(),
            local_memory_size: self.local_memory_size().ok(),
            model: self.model().ok(),
            num_slots: self.num_slots().ok(),
            vendor_id: self.vendor_id().ok(),
        }
    }
}

impl ResourceProperties for Device {
    fn properties(&self) -> &Properties {
        &self.properties
    }
}

impl ResourceProperties for &Device {
    fn properties(&self) -> &Properties {
        &self.properties
    }
}

impl DeviceProperties for Device {}

pub struct DeviceInfo {
    resource: ResourceInfo,
    bbs_id: Option<u64>,
    bbs_version: Option<fpga_version>,
    capabilities: Option<u64>,
    local_memory_size: Option<u64>,
    model: Option<String>,
    num_slots: Option<u32>,
    vendor_id: Option<u16>,
}

impl Debug for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("AcceleratorInfo");

        debug.field("resource", &self.resource);

        if let Some(bbs_id) = self.bbs_id {
            debug.field("bbs_id", &bbs_id);
        }
        if let Some(bbs_version) = self.bbs_version {
            debug.field("bbs_version", &bbs_version);
        }
        if let Some(capabilities) = self.capabilities {
            debug.field("capabilities", &capabilities);
        }
        if let Some(local_memory_size) = self.local_memory_size {
            debug.field("local_memory_size", &local_memory_size);
        }
        if let Some(ref model) = self.model {
            debug.field("model", &model);
        }
        if let Some(num_slots) = self.num_slots {
            debug.field("num_slots", &num_slots);
        }
        if let Some(vendor_id) = self.vendor_id {
            debug.field("vendor_id", &vendor_id);
        }

        debug.finish()
    }
}
