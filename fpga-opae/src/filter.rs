use crate::{
    bindings::{fpgaEnumerate, fpga_accelerator_state, fpga_objtype, fpga_token, fpga_version},
    Properties, Resource, Result, Token,
};
use std::{ffi::CString, fmt::Debug, iter::FilterMap, vec};
use uuid::Uuid;

#[derive(Clone, Default, PartialEq)]
pub struct Filter {
    pub accelerator_state: Option<fpga_accelerator_state>,
    pub bbs_id: Option<u64>,
    pub bbs_version: Option<fpga_version>,
    pub capabilities: Option<u64>,
    pub device_id: Option<u16>,
    pub guid: Option<Uuid>,
    pub local_memory_size: Option<u64>,
    pub model: Option<CString>,
    pub num_error_registers: Option<u32>,
    pub num_interrupts: Option<u32>,
    pub num_mmio_spaces: Option<u32>,
    pub num_slots: Option<u32>,
    pub obj_type: Option<fpga_objtype>,
    pub object_id: Option<u64>,
    pub pci_bus_nr: Option<u8>,
    pub pci_device_nr: Option<u8>,
    pub pci_function_nr: Option<u8>,
    pub pci_segment_nr: Option<u16>,
    pub socket_id: Option<u8>,
    pub vendor_id: Option<u16>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_accelerator_assigned(mut self) -> Self {
        self.accelerator_state
            .replace(fpga_accelerator_state::FPGA_ACCELERATOR_ASSIGNED);
        self
    }

    pub fn with_accelerator_unassigned(mut self) -> Self {
        self.accelerator_state
            .replace(fpga_accelerator_state::FPGA_ACCELERATOR_UNASSIGNED);
        self
    }

    pub fn with_bbs_id(mut self, bbs_id: u64) -> Self {
        self.bbs_id.replace(bbs_id);
        self
    }

    pub fn with_bbs_version(
        mut self,
        bbs_major_version: u8,
        bbs_minor_version: u8,
        bbs_patch_version: u16,
    ) -> Self {
        self.bbs_version.replace(fpga_version {
            major: bbs_major_version,
            minor: bbs_minor_version,
            patch: bbs_patch_version,
        });
        self
    }

    pub fn with_capabilities(mut self, capabilities: u64) -> Self {
        self.capabilities.replace(capabilities);
        self
    }

    pub fn with_device_id(mut self, device_id: u16) -> Self {
        self.device_id.replace(device_id);
        self
    }

    pub fn with_guid(mut self, guid: Uuid) -> Self {
        self.guid.replace(guid);
        self
    }

    pub fn with_local_memory_size(mut self, local_memory_size: u64) -> Self {
        self.local_memory_size.replace(local_memory_size);
        self
    }

    pub fn with_model<T>(mut self, model: T) -> Self
    where
        T: AsRef<str>,
    {
        self.model.replace(CString::new(model.as_ref()).unwrap());
        self
    }

    pub fn with_num_error_registers(mut self, num_error_registers: u32) -> Self {
        self.num_error_registers.replace(num_error_registers);
        self
    }

    pub fn with_num_interrupts(mut self, num_interrupts: u32) -> Self {
        self.num_interrupts.replace(num_interrupts);
        self
    }

    pub fn with_num_mmio_spaces(mut self, num_mmio_spaces: u32) -> Self {
        self.num_mmio_spaces.replace(num_mmio_spaces);
        self
    }

    pub fn with_num_slots(mut self, num_slots: u32) -> Self {
        self.num_slots.replace(num_slots);
        self
    }

    pub fn with_accelerator_object(mut self) -> Self {
        self.obj_type.replace(fpga_objtype::FPGA_ACCELERATOR);
        self
    }

    pub fn with_device_object(mut self) -> Self {
        self.obj_type.replace(fpga_objtype::FPGA_DEVICE);
        self
    }

    pub fn with_object_id(mut self, object_id: u64) -> Self {
        self.object_id.replace(object_id);
        self
    }

    pub fn with_pci_bus_nr(mut self, pci_bus_nr: u8) -> Self {
        self.pci_bus_nr.replace(pci_bus_nr);
        self
    }

    pub fn with_pci_device_nr(mut self, pci_device_nr: u8) -> Self {
        self.pci_device_nr.replace(pci_device_nr);
        self
    }

    pub fn with_pci_function_nr(mut self, pci_function_nr: u8) -> Self {
        self.pci_function_nr.replace(pci_function_nr);
        self
    }

    pub fn with_pci_segment_nr(mut self, pci_segment_nr: u16) -> Self {
        self.pci_segment_nr.replace(pci_segment_nr);
        self
    }

    pub fn with_socket_id(mut self, socket_id: u8) -> Self {
        self.socket_id.replace(socket_id);
        self
    }

    pub fn with_vendor_id(mut self, vendor_id: u16) -> Self {
        self.vendor_id.replace(vendor_id);
        self
    }
}

impl Debug for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Filter");

        if let Some(accelerator_state) = self.accelerator_state {
            debug.field("accelerator_state", &accelerator_state);
        }
        if let Some(bbs_id) = self.bbs_id {
            debug.field("bbs_id", &bbs_id);
        }
        if let Some(bbs_version) = self.bbs_version {
            debug.field("bbs_version", &bbs_version);
        }
        if let Some(capabilities) = self.capabilities {
            debug.field("capabilities", &capabilities);
        }
        if let Some(device_id) = self.device_id {
            debug.field("device_id", &device_id);
        }
        if let Some(guid) = self.guid {
            debug.field("guid", &guid);
        }
        if let Some(local_memory_size) = self.local_memory_size {
            debug.field("local_memory_size", &local_memory_size);
        }
        if let Some(model) = &self.model {
            debug.field("model", &model);
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
        if let Some(num_slots) = self.num_slots {
            debug.field("num_slots", &num_slots);
        }
        if let Some(obj_type) = self.obj_type {
            debug.field("obj_type", &obj_type);
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
        if let Some(vendor_id) = self.vendor_id {
            debug.field("vendor_id", &vendor_id);
        }

        debug.finish_non_exhaustive()
    }
}

pub type TokenIter = FilterMap<vec::IntoIter<fpga_token>, fn(fpga_token) -> Option<Resource>>;

impl IntoIterator for Filter {
    type Item = Resource;
    type IntoIter = TokenIter;

    fn into_iter(self) -> Self::IntoIter {
        Properties::from_filter(self)
            .and_then(|mut properties| {
                let mut num_matches = 0;
                let capacity = properties.num_matches();
                let mut tokens = Vec::with_capacity(capacity);
                Result::from(unsafe {
                    fpgaEnumerate(
                        &*properties,
                        1,
                        tokens.as_mut_ptr(),
                        capacity as u32,
                        &mut num_matches,
                    )
                })
                .map(|_| {
                    // Safety
                    // - The max nr of tokens is limited by capacity of vec
                    // - The enumerate function returns the number of matches
                    unsafe { tokens.set_len(num_matches as usize) };
                    tokens.into_iter()
                })
            })
            .unwrap_or_else(|_| Vec::default().into_iter())
            .filter_map(|token| Resource::from_token(Token::from_raw(token)).ok())
    }
}
