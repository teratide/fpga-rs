#include "src/ffi.h"
// #include "fpga-xrt/src/ffi.rs.h"

// todo(mb): remove
#include <iostream>

// Xclbin
namespace xrt
{
  rust::String xclbin_kernel_get_name(const xclbin_kernel &kernel)
  {
    return kernel.get_name();
  }

  rust::String xclbin_ip_get_name(const xclbin_ip &ip)
  {
    return ip.get_name();
  }

  std::unique_ptr<Xclbin> new_xclbin(const rust::Slice<const int8_t> data)
  {
    std::vector<char> input(data.begin(), data.end());
    return std::make_unique<Xclbin>(input);
  }

  std::unique_ptr<std::vector<xclbin::kernel>> Xclbin::get_kernels() const
  {
    return std::make_unique<std::vector<xclbin::kernel>>(xclbin::get_kernels());
  }

  std::unique_ptr<std::vector<xclbin::ip>> Xclbin::get_ips() const
  {
    return std::make_unique<std::vector<xclbin::ip>>(xclbin::get_ips());
  }

  rust::String Xclbin::get_xsa_name() const
  {
    return xclbin::get_xsa_name();
  }

  std::array<unsigned char, 16> Xclbin::get_uuid() const
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

  rust::String Device::bdf() const
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

  rust::String Device::name() const
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

  rust::String Device::electrical() const
  {
    return get_info<xrt::info::device::electrical>();
  }

  rust::String Device::thermal() const
  {
    return get_info<xrt::info::device::thermal>();
  }

  rust::String Device::mechanical() const
  {
    return get_info<xrt::info::device::mechanical>();
  }

  rust::String Device::memory() const
  {
    return get_info<xrt::info::device::memory>();
  }

  rust::String Device::platform() const
  {
    return get_info<xrt::info::device::platform>();
  }

  rust::String Device::pcie_info() const
  {
    return get_info<xrt::info::device::pcie_info>();
  }

  rust::String Device::host() const
  {
    return get_info<xrt::info::device::host>();
  }

  rust::String Device::dynamic_regions() const
  {
    return get_info<xrt::info::device::dynamic_regions>();
  }

  std::array<unsigned char, 16> Device::xclbin_uuid() const
  {
    std::array<unsigned char, 16> array;
    memcpy(array.data(), get_xclbin_uuid().get(), sizeof(unsigned char) * 16);
    return array;
  }

  std::array<unsigned char, 16> Device::load_xclbin(const Xclbin &xclbin)
  {
    std::array<unsigned char, 16> array;
    std::cout << "load_xclbin" << std::endl;
    xrt::xclbin input = xclbin;
    auto uuid = xrt::device::load_xclbin(input).get();
    std::cout << "load_xclbin done" << std::endl;
    memcpy(array.data(), uuid, sizeof(unsigned char) * 16);
    return array;
  }
}

// Kernel
namespace xrt
{
  std::unique_ptr<kernel> new_kernel(const Device &device,
                                     std::array<unsigned char, 16> xclbin_id,
                                     const rust::String name,
                                     kernel_cu_access_mode mode)
  {
    unsigned char uuid[16];
    memcpy(uuid, xclbin_id.data(), sizeof(unsigned char) * 16);
    xrt::uuid xclbin_id_(uuid);
    std::string name_(name);
    xrt::device device_(device);
    return std::make_unique<kernel>(device_, xclbin_id_, name_, mode);
  }
}
