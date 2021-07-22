use crate::{
    bindings::{fpgaPropertiesGetObjectType, fpga_objtype},
    Accelerator, Device, Properties, ResourceProperties, Result, Token,
};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug)]
pub enum Resource {
    Device(Device),
    Accelerator(Accelerator),
}

impl Resource {
    pub fn from_token(token: Token) -> Result<Self> {
        let properties = Properties::from_token(&token)?;

        let mut obj_type = fpga_objtype::FPGA_DEVICE;
        Result::from(unsafe { fpgaPropertiesGetObjectType(*properties, &mut obj_type) })?;

        Ok(match obj_type {
            fpga_objtype::FPGA_DEVICE => Self::Device(Device::new(token, properties)),
            fpga_objtype::FPGA_ACCELERATOR => {
                Self::Accelerator(Accelerator::new(token, properties))
            }
        })
    }

    pub fn is_device(&self) -> bool {
        matches!(self, Self::Device(_))
    }

    pub fn unwrap_device(self) -> Device {
        match self {
            Self::Device(device) => device,
            _ => panic!("no device"),
        }
    }
}

pub struct ResourceInfo {
    device_id: Option<u16>,
    guid: Option<Uuid>,
    object_id: Option<u64>,
    pci_bus_nr: Option<u8>,
    pci_device_nr: Option<u8>,
    pci_function_nr: Option<u8>,
    pci_segment_nr: Option<u16>,
    socket_id: Option<u8>,
}

impl Debug for ResourceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("");

        if let Some(device_id) = self.device_id {
            debug.field("device_id", &device_id);
        }
        if let Some(guid) = self.guid {
            debug.field("guid", &guid);
        }
        if let Some(object_id) = self.object_id {
            debug.field("object_id", &object_id);
        }
        if let Some(pci_bus_nr) = self.pci_bus_nr {
            debug.field("pci_bus_nr", &pci_bus_nr);
        }
        if let Some(pci_device_nr) = self.pci_device_nr {
            debug.field("pci_device_nr", &pci_device_nr);
        }
        if let Some(pci_function_nr) = self.pci_function_nr {
            debug.field("pci_function_nr", &pci_function_nr);
        }
        if let Some(pci_segment_nr) = self.pci_segment_nr {
            debug.field("pci_segment_nr", &pci_segment_nr);
        }
        if let Some(socket_id) = self.socket_id {
            debug.field("socket_id", &socket_id);
        }

        debug.finish_non_exhaustive()
    }
}

impl<T> From<T> for ResourceInfo
where
    T: ResourceProperties,
{
    fn from(resource: T) -> Self {
        ResourceInfo {
            device_id: resource.device_id().ok(),
            guid: resource.guid().ok(),
            object_id: resource.object_id().ok(),
            pci_bus_nr: resource.pci_bus_nr().ok(),
            pci_device_nr: resource.pci_device_nr().ok(),
            pci_function_nr: resource.pci_function_nr().ok(),
            pci_segment_nr: resource.pci_segment_nr().ok(),
            socket_id: resource.socket_id().ok(),
        }
    }
}
