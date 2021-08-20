#pragma once
#include "rust/cxx.h"

#include "fpga-xrt/xrt/src/runtime_src/core/include/xrt/xrt_device.h"
#include "fpga-xrt/xrt/src/runtime_src/core/include/xrt/xrt_kernel.h"

#include "fpga-xrt/xrt/src/runtime_src/core/include/experimental/xrt_xclbin.h"

// #include <memory>

namespace xrt
{
  // export nested class.
  using xclbin_kernel = xclbin::kernel;

  rust::String xclbin_kernel_get_name(const xclbin_kernel &kernel);

  // export nested class.
  using xclbin_ip = xclbin::ip;

  rust::String xclbin_ip_get_name(const xclbin_ip &ip);

  class Xclbin : public xrt::xclbin
  {
    // Constructor.
    using xrt::xclbin::xclbin;

  public:
    rust::String get_xsa_name() const;
    std::unique_ptr<std::vector<xclbin::kernel>> get_kernels() const;
    std::unique_ptr<std::vector<xclbin::ip>> get_ips() const;
    std::array<unsigned char, 16> get_uuid() const;
  };

  // FFI constructor.
  std::unique_ptr<Xclbin>
  new_xclbin(const rust::Slice<const int8_t> data);
}

namespace xrt
{
  class Device : public xrt::device
  {
    // Constructor.
    using xrt::device::device;

  public:
    // get_info methods.
    rust::String bdf() const;
    std::array<unsigned char, 16> interface_uuid() const;
    uint32_t kdma() const;
    unsigned long max_clock_frequency_mhz() const;
    bool m2m() const;
    rust::String name() const;
    bool nodma() const;
    bool offline() const;
    rust::String electrical() const;
    rust::String thermal() const;
    rust::String mechanical() const;
    rust::String memory() const;
    rust::String platform() const;
    rust::String pcie_info() const;
    rust::String host() const;
    rust::String dynamic_regions() const;

    // Modified member functions.
    std::array<unsigned char, 16> xclbin_uuid() const;
    std::array<unsigned char, 16> load_xclbin(const Xclbin &xclbin);
  };

  // FFI constructor.
  std::unique_ptr<Device> new_device(unsigned int index);

}

namespace xrt
{
  // export nested enum class.
  using kernel_cu_access_mode = xrt::kernel::cu_access_mode;

  // class Kernel : public xrt::kernel
  // {
  //   using xrt::kernel::kernel;

  // public:
  // };

  // FFI constructor.
  std::unique_ptr<kernel> new_kernel(
      const Device &device,
      std::array<unsigned char, 16> xclbin_id,
      const rust::String name,
      kernel_cu_access_mode mode);
}
