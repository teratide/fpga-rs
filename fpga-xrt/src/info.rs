use crate::Xrt;
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Electrical {
    #[serde(deserialize_with = "parse")]
    pub power_consumption_max_watts: f32,
    #[serde(deserialize_with = "parse")]
    pub power_consumption_warning: bool,
    #[serde(deserialize_with = "parse")]
    pub power_consumption_watts: f32,
    pub power_rails: Vec<Power>,
}

fn parse<'de, T: FromStr, D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<T, D::Error>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    Ok(match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::String(s) => s.parse::<T>().map_err(serde::de::Error::custom)?,
        _ => return Err(serde::de::Error::custom("unexpected value")),
    })
}

#[derive(Debug, Deserialize)]
pub struct Power {
    pub id: String,
    pub description: String,
    pub current: Current,
    pub voltage: Voltage,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    #[serde(deserialize_with = "parse")]
    pub amps: f32,
    #[serde(deserialize_with = "parse")]
    pub is_present: bool,
    error_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Voltage {
    #[serde(deserialize_with = "parse")]
    pub volts: f32,
    #[serde(deserialize_with = "parse")]
    pub is_present: bool,
    pub error_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Thermals {
    pub thermals: Vec<Thermal>,
}

#[derive(Debug, Deserialize)]
pub struct Thermal {
    pub location_id: String,
    pub description: String,
    #[serde(rename(deserialize = "temp_C"), deserialize_with = "parse")]
    pub temp_c: u8,
    #[serde(deserialize_with = "parse")]
    pub is_present: bool,
}

#[derive(Debug, Deserialize)]
pub struct Mechanical {
    pub fans: Vec<Fan>,
}

#[derive(Debug, Deserialize)]
pub struct Fan {
    pub location_id: String,
    pub description: String,
    #[serde(deserialize_with = "parse")]
    pub speed_rpm: u16,
    #[serde(
        rename(deserialize = "critical_trigger_temp_C"),
        deserialize_with = "parse"
    )]
    pub critical_trigger_temp_c: u8,
    #[serde(deserialize_with = "parse")]
    pub is_present: bool,
}

#[derive(Debug, Deserialize)]
pub struct Memory {
    pub board: Option<Board>,
    pub error_msg: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Board {
    pub direct_memory_accesses: DirectMemoryAccesses,
    pub memory: InnerMemory,
}
#[derive(Debug, Deserialize)]
pub struct DirectMemoryAccesses {
    pub metrics: Vec<Metric>,
    #[serde(rename(deserialize = "type"))]
    pub ty: String,
}
#[derive(Debug, Deserialize)]
pub struct Metric {
    #[serde(deserialize_with = "parse")]
    pub channel_id: u8,
    pub card_to_host_bytes: String, // todo
    pub host_to_card_bytes: String, // todo
}
#[derive(Debug, Deserialize)]
pub struct InnerMemory {
    pub data_streams: Vec<DataStream>,
    pub memories: Vec<Mem>,
}
#[derive(Debug, Deserialize)]
pub struct Mem {
    pub base_address: String, // todo
    #[serde(deserialize_with = "parse")]
    pub enabled: bool,
    pub error_msg: Option<String>,
    pub extended_info: ExtendedInfo,
    pub range_bytes: String, // todo
    pub tag: String,
    #[serde(rename(deserialize = "type"))]
    pub ty: String,
}
#[derive(Debug, Deserialize)]
pub struct ExtendedInfo {
    pub usage: Usage,
}
#[derive(Debug, Deserialize)]
pub struct Usage {
    #[serde(deserialize_with = "parse")]
    pub allocated_bytes: u64,
    #[serde(deserialize_with = "parse")]
    pub buffer_objects_count: u64,
}
#[derive(Debug, Deserialize)]
pub struct DataStream {
    pub tag: String,
}

#[derive(Debug, Deserialize)]
pub struct Platform {
    pub controller: Controller,
    pub macs: Vec<Mac>,
    pub off_chip_board_info: OffChipBoardInfo,
    pub static_region: StaticRegion,
    pub status: Status,
}

#[derive(Deserialize)]
struct Wrapper<T> {
    #[serde(rename(deserialize = ""))]
    inner: T,
}

#[derive(Debug, Deserialize)]
pub struct Controller {
    pub card_mgmt_controller: CardMgmtController,
    pub satellite_controller: SatelliteController,
}
#[derive(Debug, Deserialize)]
pub struct CardMgmtController {
    pub oem_id: String,
    pub serial_number: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct SatelliteController {
    pub expected_version: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Mac {
    pub address: String, // todo [u8; 6]
}

#[derive(Debug, Deserialize)]
pub struct OffChipBoardInfo {
    #[serde(deserialize_with = "parse")]
    pub ddr_count: u8,
    #[serde(deserialize_with = "parse")]
    pub ddr_size_bytes: u64,
}

#[derive(Debug, Deserialize)]
pub struct StaticRegion {
    pub fpga_name: String,
    pub interface_uuid: String,
    pub jtag_idcode: String,
    pub vbnv: String,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    #[serde(deserialize_with = "parse")]
    pub mig_calibrated: bool,
    pub p2p_status: String,
}

#[derive(Debug, Deserialize)]
pub struct PCIeInfo {
    pub cpu_affinity: String,
    pub device: String,
    #[serde(deserialize_with = "parse")]
    pub dma_thread_count: u8,
    #[serde(deserialize_with = "parse")]
    pub express_lane_width_count: u8,
    #[serde(deserialize_with = "parse")]
    pub link_speed_gbit_sec: u16,
    #[serde(deserialize_with = "parse")]
    pub max_shared_host_mem_aperture_bytes: u64,
    pub sub_device: String,
    pub sub_vendor: String,
    pub vendor: String,
}

#[derive(Debug, Deserialize)]
pub struct Host {
    pub branch: String,
    pub build_date: String,
    pub hash: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct DynamicRegions {
    pub xclbin_uuid: Uuid,
}

impl Xrt {
    pub fn bdf(&self) -> &str {
        self.device.bdf()
    }
    pub fn interface_uuid(&self) -> Uuid {
        Uuid::from_bytes(self.device.interface_uuid())
    }
    pub fn kdma(&self) -> u32 {
        self.device.kdma()
    }
    pub fn max_clock_frequency_mhz(&self) -> u64 {
        self.device.max_clock_frequency_mhz()
    }
    pub fn m2m(&self) -> bool {
        self.device.m2m()
    }
    pub fn name(&self) -> &str {
        self.device.name()
    }
    pub fn nodma(&self) -> bool {
        self.device.nodma()
    }
    pub fn offline(&self) -> bool {
        self.device.offline()
    }
    // pub fn electrical(&self) -> Electrical {
    //     let mut electrical: Electrical = serde_json::from_str(&self.device.electrical()).unwrap();
    //     electrical.power_rails = electrical
    //         .power_rails
    //         .into_iter()
    //         .filter(|power| power.current.is_present || power.voltage.is_present)
    //         .collect();
    //     electrical
    // }
    // pub fn thermal(&self) -> Vec<Thermal> {
    //     let thermals: Thermals = serde_json::from_str(&self.device.thermal()).unwrap();
    //     thermals
    //         .thermals
    //         .into_iter()
    //         .filter(|thermal| thermal.is_present)
    //         .collect()
    // }
    // pub fn mechanical(&self) -> Mechanical {
    //     serde_json::from_str(&self.device.mechanical()).unwrap()
    // }
    // pub fn memory(&self) -> Memory {
    //     serde_json::from_str(&self.device.memory()).unwrap()
    // }
    // pub fn platform(&self) -> Platform {
    //     let wrapper: Wrapper<Platform> = serde_json::from_str(&self.device.platform()).unwrap();
    //     wrapper.inner
    // }
    // pub fn pcie_info(&self) -> PCIeInfo {
    //     serde_json::from_str(&self.device.pcie_info()).unwrap()
    // }
    // pub fn host(&self) -> Host {
    //     serde_json::from_str(&self.device.host()).unwrap()
    // }
    // pub fn dynamic_regions(&self) -> DynamicRegions {
    //     serde_json::from_str(&self.device.dynamic_regions()).unwrap()
    // }
}
