use crate::{
    bindings::{
        fpgaCloneProperties, fpgaDestroyProperties, fpgaEnumerate, fpgaGetProperties,
        fpgaPropertiesGetAcceleratorState, fpgaPropertiesGetBBSID, fpgaPropertiesGetBBSVersion,
        fpgaPropertiesGetBus, fpgaPropertiesGetCapabilities, fpgaPropertiesGetDevice,
        fpgaPropertiesGetDeviceID, fpgaPropertiesGetFunction, fpgaPropertiesGetGUID,
        fpgaPropertiesGetLocalMemorySize, fpgaPropertiesGetModel, fpgaPropertiesGetNumErrors,
        fpgaPropertiesGetNumInterrupts, fpgaPropertiesGetNumMMIO, fpgaPropertiesGetNumSlots,
        fpgaPropertiesGetObjectID, fpgaPropertiesGetSegment, fpgaPropertiesGetSocketID,
        fpgaPropertiesGetVendorID, fpgaPropertiesSetAcceleratorState, fpgaPropertiesSetBBSID,
        fpgaPropertiesSetBBSVersion, fpgaPropertiesSetBus, fpgaPropertiesSetCapabilities,
        fpgaPropertiesSetDevice, fpgaPropertiesSetDeviceID, fpgaPropertiesSetFunction,
        fpgaPropertiesSetGUID, fpgaPropertiesSetLocalMemorySize, fpgaPropertiesSetModel,
        fpgaPropertiesSetNumErrors, fpgaPropertiesSetNumInterrupts, fpgaPropertiesSetNumMMIO,
        fpgaPropertiesSetNumSlots, fpgaPropertiesSetObjectID, fpgaPropertiesSetObjectType,
        fpgaPropertiesSetSegment, fpgaPropertiesSetSocketID, fpgaPropertiesSetVendorID,
        fpga_accelerator_state, fpga_properties, fpga_version,
    },
    Filter, Result, Token,
};
use log::{error, trace};
use std::{
    ffi::CString,
    ops::{Deref, DerefMut, Not},
    ptr,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Properties(fpga_properties);

impl Properties {
    pub fn from_raw(properties: fpga_properties) -> Self {
        Properties(properties)
    }

    pub fn from_token(token: &Token) -> Result<Self> {
        let mut properties = ptr::null_mut();
        Result::from(unsafe { fpgaGetProperties(**token, &mut properties) })?;
        Ok(Self::from_raw(properties))
    }

    pub fn num_matches(&mut self) -> usize {
        let mut num_matches = 0;
        Result::from(unsafe { fpgaEnumerate(&**self, 1, ptr::null_mut(), 0, &mut num_matches) })
            .map(|_| num_matches as usize)
            .unwrap_or(0)
    }

    pub fn from_filter(filter: Filter) -> Result<Self> {
        let mut properties: fpga_properties = ptr::null_mut();

        Result::from(unsafe { fpgaGetProperties(ptr::null_mut(), &mut properties) })?;

        if let Some(pci_bus_nr) = filter.pci_bus_nr {
            Result::from(unsafe { fpgaPropertiesSetBus(properties, pci_bus_nr) })?;
        }
        if let Some(guid) = filter.guid {
            Result::from(unsafe {
                fpgaPropertiesSetGUID(properties, guid.as_bytes().to_owned().as_mut_ptr())
            })?;
        }
        if let Some(bbs_id) = filter.bbs_id {
            Result::from(unsafe { fpgaPropertiesSetBBSID(properties, bbs_id) })?;
        }
        if let Some(model) = filter.model {
            Result::from(unsafe { fpgaPropertiesSetModel(properties, model.into_raw()) })?;
        }
        if let Some(pci_device_nr) = filter.pci_device_nr {
            Result::from(unsafe { fpgaPropertiesSetDevice(properties, pci_device_nr) })?;
        }
        if let Some(num_mmio_spaces) = filter.num_mmio_spaces {
            Result::from(unsafe { fpgaPropertiesSetNumMMIO(properties, num_mmio_spaces) })?;
        }
        if let Some(pci_segment_nr) = filter.pci_segment_nr {
            Result::from(unsafe { fpgaPropertiesSetSegment(properties, pci_segment_nr) })?;
        }
        if let Some(device_id) = filter.device_id {
            Result::from(unsafe { fpgaPropertiesSetDeviceID(properties, device_id) })?;
        }
        if let Some(pci_function_nr) = filter.pci_function_nr {
            Result::from(unsafe { fpgaPropertiesSetFunction(properties, pci_function_nr) })?;
        }
        if let Some(num_slots) = filter.num_slots {
            Result::from(unsafe { fpgaPropertiesSetNumSlots(properties, num_slots) })?;
        }
        if let Some(object_id) = filter.object_id {
            Result::from(unsafe { fpgaPropertiesSetObjectID(properties, object_id) })?;
        }
        if let Some(socket_id) = filter.socket_id {
            Result::from(unsafe { fpgaPropertiesSetSocketID(properties, socket_id) })?;
        }
        if let Some(vendor_id) = filter.vendor_id {
            Result::from(unsafe { fpgaPropertiesSetVendorID(properties, vendor_id) })?;
        }
        if let Some(num_error_registers) = filter.num_error_registers {
            Result::from(unsafe { fpgaPropertiesSetNumErrors(properties, num_error_registers) })?;
        }
        if let Some(bbs_version) = filter.bbs_version {
            Result::from(unsafe { fpgaPropertiesSetBBSVersion(properties, bbs_version) })?;
        }
        if let Some(obj_type) = filter.obj_type {
            Result::from(unsafe { fpgaPropertiesSetObjectType(properties, obj_type) })?;
        }
        if let Some(capabilities) = filter.capabilities {
            Result::from(unsafe { fpgaPropertiesSetCapabilities(properties, capabilities) })?;
        }
        if let Some(num_interrupts) = filter.num_interrupts {
            Result::from(unsafe { fpgaPropertiesSetNumInterrupts(properties, num_interrupts) })?;
        }
        if let Some(local_memory_size) = filter.local_memory_size {
            Result::from(unsafe {
                fpgaPropertiesSetLocalMemorySize(properties, local_memory_size)
            })?;
        }
        if let Some(accelerator_state) = filter.accelerator_state {
            Result::from(unsafe {
                fpgaPropertiesSetAcceleratorState(properties, accelerator_state)
            })?;
        }

        Ok(Self(properties))
    }
}

impl Clone for Properties {
    fn clone(&self) -> Self {
        let mut properties = ptr::null_mut();
        Result::from(unsafe { fpgaCloneProperties(self.0, &mut properties) }).unwrap();
        Self(properties)
    }
}

impl Deref for Properties {
    type Target = fpga_properties;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Properties {
    fn drop(&mut self) {
        trace!("Dropping Properties: {:p}", self.0);
        if Result::from(unsafe { fpgaDestroyProperties(&mut self.0) }).is_err() {
            error!("Failed to destroy properties object");
        }
    }
}

pub trait ResourceProperties {
    fn properties(&self) -> &Properties;

    fn device_id(&self) -> Result<u16> {
        let mut device_id = Default::default();
        Result::from(unsafe { fpgaPropertiesGetDeviceID(**self.properties(), &mut device_id) })
            .map(|_| device_id)
    }

    fn guid(&self) -> Result<Uuid> {
        let mut guid = [Default::default(); 16];
        Result::from(unsafe { fpgaPropertiesGetGUID(**self.properties(), &mut guid) })
            .map(|_| Uuid::from_bytes(guid))
    }

    fn object_id(&self) -> Result<u64> {
        let mut object_id = Default::default();
        Result::from(unsafe { fpgaPropertiesGetObjectID(**self.properties(), &mut object_id) })
            .map(|_| object_id)
    }

    fn pci_bus_nr(&self) -> Result<u8> {
        let mut pci_bus_nr = Default::default();
        Result::from(unsafe { fpgaPropertiesGetBus(**self.properties(), &mut pci_bus_nr) })
            .map(|_| pci_bus_nr)
    }

    fn pci_device_nr(&self) -> Result<u8> {
        let mut pci_device_nr = Default::default();
        Result::from(unsafe { fpgaPropertiesGetDevice(**self.properties(), &mut pci_device_nr) })
            .map(|_| pci_device_nr)
    }

    fn pci_function_nr(&self) -> Result<u8> {
        let mut pci_function_nr = Default::default();
        Result::from(unsafe {
            fpgaPropertiesGetFunction(**self.properties(), &mut pci_function_nr)
        })
        .map(|_| pci_function_nr)
    }

    fn pci_segment_nr(&self) -> Result<u16> {
        let mut pci_segment_nr = Default::default();
        Result::from(unsafe { fpgaPropertiesGetSegment(**self.properties(), &mut pci_segment_nr) })
            .map(|_| pci_segment_nr)
    }

    fn socket_id(&self) -> Result<u8> {
        let mut socket_id = Default::default();
        Result::from(unsafe { fpgaPropertiesGetSocketID(**self.properties(), &mut socket_id) })
            .map(|_| socket_id)
    }
}

pub trait DeviceProperties: ResourceProperties {
    fn bbs_id(&self) -> Result<u64> {
        let mut bbs_id = Default::default();
        Result::from(unsafe { fpgaPropertiesGetBBSID(**self.properties(), &mut bbs_id) })
            .map(|_| bbs_id)
    }

    fn bbs_version(&self) -> Result<fpga_version> {
        let mut bbs_version = Default::default();
        Result::from(unsafe { fpgaPropertiesGetBBSVersion(**self.properties(), &mut bbs_version) })
            .map(|_| bbs_version)
    }

    fn capabilities(&self) -> Result<u64> {
        let mut capabilities = Default::default();
        Result::from(unsafe {
            fpgaPropertiesGetCapabilities(**self.properties(), &mut capabilities)
        })
        .map(|_| capabilities)
    }

    fn local_memory_size(&self) -> Result<u64> {
        let mut local_memory_size = Default::default();
        Result::from(unsafe {
            fpgaPropertiesGetLocalMemorySize(**self.properties(), &mut local_memory_size)
        })
        .map(|_| local_memory_size)
    }

    fn model(&self) -> Result<String> {
        let mut model = Vec::with_capacity(u8::MAX as usize); // FPGA_MODEL_LENGTH not defined;
        Result::from(unsafe { fpgaPropertiesGetModel(**self.properties(), model.as_mut_ptr()) })
            .map(|_| {
                unsafe { CString::from_raw(model.as_mut_ptr()) }
                    .into_string()
                    .unwrap_or_else(|_| String::from("?"))
            })
    }

    fn num_slots(&self) -> Result<u32> {
        let mut num_slots = Default::default();
        Result::from(unsafe { fpgaPropertiesGetNumSlots(**self.properties(), &mut num_slots) })
            .map(|_| num_slots)
    }

    fn vendor_id(&self) -> Result<u16> {
        let mut vendor_id = Default::default();
        Result::from(unsafe { fpgaPropertiesGetVendorID(**self.properties(), &mut vendor_id) })
            .map(|_| vendor_id)
    }
}

pub trait AcceleratorProperties: ResourceProperties {
    fn is_assigned(&self) -> Result<bool> {
        let mut state = fpga_accelerator_state::FPGA_ACCELERATOR_UNASSIGNED;
        Result::from(unsafe { fpgaPropertiesGetAcceleratorState(**self.properties(), &mut state) })
            .map(|_| matches!(state, fpga_accelerator_state::FPGA_ACCELERATOR_ASSIGNED))
    }

    fn is_unassigned(&self) -> Result<bool> {
        self.is_assigned().map(Not::not)
    }

    fn num_error_registers(&self) -> Result<u32> {
        let mut num_error_registers = Default::default();
        Result::from(unsafe {
            fpgaPropertiesGetNumErrors(**self.properties(), &mut num_error_registers)
        })
        .map(|_| num_error_registers)
    }

    fn num_interrupts(&self) -> Result<u32> {
        let mut num_interrupts = Default::default();
        Result::from(unsafe {
            fpgaPropertiesGetNumInterrupts(**self.properties(), &mut num_interrupts)
        })
        .map(|_| num_interrupts)
    }

    fn num_mmio_spaces(&self) -> Result<u32> {
        let mut num_mmio_spaces = Default::default();
        Result::from(unsafe { fpgaPropertiesGetNumMMIO(**self.properties(), &mut num_mmio_spaces) })
            .map(|_| num_mmio_spaces)
    }
}
