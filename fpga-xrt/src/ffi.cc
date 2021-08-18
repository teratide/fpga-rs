#include "src/ffi.h"
// #include "fpga-xrt/src/ffi.rs.h"
#include "xrt/xrt_device.h"

namespace xrt {

rust::String Device::name() const {
  return get_info<xrt::info::device::name>();
}

rust::String Device::bdf() const {
  return get_info<xrt::info::device::bdf>();
}

std::array<unsigned char, 16> Device::xclbin_uuid() const {
  std::array<unsigned char, 16> array;
  memcpy(array.data(), get_xclbin_uuid().get(), sizeof(unsigned char) * 16);
  return array;
}

std::unique_ptr<Device> new_device(unsigned int index) {
  return std::make_unique<Device>(index);
}

}