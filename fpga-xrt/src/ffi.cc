#include "src/ffi.h"
// #include "fpga-xrt/src/ffi.rs.h"

#include <iostream>

// Xclbin
namespace xrt
{
  rust::Str XclbinArg::name() const
  {
    return get_name();
  }

  std::unique_ptr<std::vector<XclbinMem>> XclbinArg::mems() const
  {
    std::vector<xclbin::mem> input = get_mems();
    std::vector<XclbinMem> output(input.begin(), input.end());
    return std::make_unique<std::vector<XclbinMem>>(output);
  }

  rust::Str XclbinKernel::name() const
  {
    return get_name();
  }

  rust::Str XclbinIp::name() const
  {
    return get_name();
  }

  std::unique_ptr<std::vector<XclbinArg>> XclbinIp::args() const
  {
    std::vector<xclbin::arg> input = get_args();
    std::vector<XclbinArg> output(input.begin(), input.end());
    return std::make_unique<std::vector<XclbinArg>>(output);
  }

  std::unique_ptr<XclbinArg> XclbinIp::arg(int32_t index) const
  {
    xclbin::arg arg = get_arg(index);
    return std::make_unique<XclbinArg>(arg);
  }

  std::unique_ptr<Xclbin> new_xclbin(const rust::Slice<const int8_t> data)
  {
    std::vector<char> input(data.begin(), data.end());
    return std::make_unique<Xclbin>(input);
  }

  std::unique_ptr<std::vector<XclbinKernel>> Xclbin::kernels() const
  {
    std::vector<xclbin::kernel> input = get_kernels();
    std::vector<XclbinKernel> output(input.begin(), input.end());
    return std::make_unique<std::vector<XclbinKernel>>(output);
  }

  std::unique_ptr<XclbinKernel> Xclbin::kernel(const rust::Str name) const
  {
    return std::make_unique<XclbinKernel>(get_kernel(std::string(name)));
  }

  std::unique_ptr<std::vector<XclbinIp>> Xclbin::ips() const
  {
    std::vector<xclbin::ip> input = get_ips();
    std::vector<XclbinIp> output(input.begin(), input.end());
    return std::make_unique<std::vector<XclbinIp>>(output);
  }

  std::unique_ptr<XclbinIp> Xclbin::ip(const rust::Str name) const
  {
    return std::make_unique<XclbinIp>(get_ip(std::string(name)));
  }

  rust::Str Xclbin::xsa_name() const
  {
    return get_xsa_name();
  }

  std::array<unsigned char, 16> Xclbin::uuid() const
  {
    std::array<unsigned char, 16> array;
    memcpy(array.data(), xclbin::get_uuid().get(), sizeof(unsigned char) * 16);
    return array;
  }

}

// Device
namespace xrt
{
  std::unique_ptr<Device> new_device(unsigned int index)
  {
    return std::make_unique<Device>(index);
  }

  std::unique_ptr<Device> new_device_bdf(const rust::Str bdf)
  {
    return std::make_unique<Device>(std::string(bdf));
  }

  rust::Str Device::bdf() const
  {
    return get_info<xrt::info::device::bdf>();
  }

  std::array<unsigned char, 16> Device::interface_uuid() const
  {
    std::array<unsigned char, 16> array;
    memcpy(array.data(), get_info<xrt::info::device::interface_uuid>().get(), sizeof(unsigned char) * 16);
    return array;
  }

  uint32_t Device::kdma() const
  {
    return get_info<xrt::info::device::kdma>();
  }

  unsigned long Device::max_clock_frequency_mhz() const
  {
    return get_info<xrt::info::device::max_clock_frequency_mhz>();
  }

  bool Device::m2m() const
  {
    return get_info<xrt::info::device::m2m>();
  }

  rust::Str Device::name() const
  {
    return get_info<xrt::info::device::name>();
  }

  bool Device::nodma() const
  {
    return get_info<xrt::info::device::nodma>();
  }

  bool Device::offline() const
  {
    return get_info<xrt::info::device::offline>();
  }

  // rust::String Device::electrical() const
  // {
  //   return get_info<xrt::info::device::electrical>();
  // }

  // rust::String Device::thermal() const
  // {
  //   return get_info<xrt::info::device::thermal>();
  // }

  // rust::String Device::mechanical() const
  // {
  //   return get_info<xrt::info::device::mechanical>();
  // }

  // rust::String Device::memory() const
  // {
  //   return get_info<xrt::info::device::memory>();
  // }

  // rust::String Device::platform() const
  // {
  //   return get_info<xrt::info::device::platform>();
  // }

  // rust::String Device::pcie_info() const
  // {
  //   return get_info<xrt::info::device::pcie_info>();
  // }

  // rust::String Device::host() const
  // {
  //   return get_info<xrt::info::device::host>();
  // }

  // rust::String Device::dynamic_regions() const
  // {
  //   return get_info<xrt::info::device::dynamic_regions>();
  // }

  std::array<unsigned char, 16> Device::xclbin_uuid() const
  {
    std::array<unsigned char, 16> array;
    memcpy(array.data(), get_xclbin_uuid().get(), sizeof(unsigned char) * 16);
    return array;
  }

  std::array<unsigned char, 16> Device::load(const Xclbin &xclbin)
  {
    auto uuid = load_xclbin(xclbin).get();
    std::array<unsigned char, 16> array;
    memcpy(array.data(), uuid, sizeof(unsigned char) * 16);
    return array;
  }
}

// Kernel
namespace xrt
{
  std::unique_ptr<kernel> new_kernel(const Device &device,
                                     std::array<unsigned char, 16> xclbin_id,
                                     const rust::Str name,
                                     kernel_cu_access_mode mode)
  {
    xrt::uuid uuid(xclbin_id.data());
    // // workaround that registers the alxf
    // std::shared_ptr<xrt_core::device> handle = device.get_handle();
    // handle->register_axlf(xclbin.get_axlf());
    // xrt::xclbin bin(xclbin);
    return std::make_unique<kernel>(device, uuid, std::string(name), mode);
  }
}

// IP
namespace xrt
{
  std::unique_ptr<ip> new_ip(const Device &device,
                             std::array<unsigned char, 16> xclbin_id,
                             const rust::Str name)
  {
    xrt::uuid uuid(xclbin_id.data());
    return std::make_unique<ip>(device, uuid, std::string(name));
  }
}

// Ini
namespace xrt
{
  void set_ini(rust::Str key, rust::Str value)
  {
    xrt::ini::set(std::string(key), std::string(value));
  }
}