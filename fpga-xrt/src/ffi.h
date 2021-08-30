#pragma once
#include "rust/cxx.h"

#include "fpga-xrt/xrt/src/runtime_src/core/include/xrt/xrt_device.h"
#include "fpga-xrt/xrt/src/runtime_src/core/include/xrt/xrt_kernel.h"

#include "fpga-xrt/xrt/src/runtime_src/core/include/experimental/xrt_ini.h"
#include "fpga-xrt/xrt/src/runtime_src/core/include/experimental/xrt_ip.h"
#include "fpga-xrt/xrt/src/runtime_src/core/include/experimental/xrt_xclbin.h"

namespace xrt
{
  class XclbinMem : public xclbin::mem
  {
    using xclbin::mem::mem;

  public:
    XclbinMem(const xclbin::mem &mem) : xclbin::mem(mem){};
  };

  class XclbinArg : public xclbin::arg
  {
    using xclbin::arg::arg;

  public:
    XclbinArg(const xclbin::arg &arg) : xclbin::arg(arg){};
    rust::Str name() const;
    std::unique_ptr<std::vector<XclbinMem>> mems() const;
  };

  class XclbinKernel : public xclbin::kernel
  {
    using xclbin::kernel::kernel;

  public:
    XclbinKernel(const xclbin::kernel &kernel) : xclbin::kernel(kernel){};
    rust::Str name() const;
  };

  class XclbinIp : public xclbin::ip
  {
    using xclbin::ip::ip;

  public:
    XclbinIp(const xclbin::ip &ip) : xclbin::ip(ip){};
    rust::Str name() const;
    std::unique_ptr<std::vector<XclbinArg>> args() const;
    std::unique_ptr<XclbinArg> arg(int32_t index) const;
  };

  class Xclbin : public xrt::xclbin
  {
    using xrt::xclbin::xclbin;

  public:
    rust::Str xsa_name() const;
    std::unique_ptr<std::vector<XclbinKernel>> kernels() const;
    std::unique_ptr<XclbinKernel> kernel(const rust::Str name) const;
    std::unique_ptr<std::vector<XclbinIp>> ips() const;
    std::unique_ptr<XclbinIp> ip(const rust::Str name) const;
    std::array<unsigned char, 16> uuid() const;
  };

  std::unique_ptr<Xclbin>
  new_xclbin(const rust::Slice<const int8_t> data);
}

namespace xrt
{
  class Device : public xrt::device
  {
    using xrt::device::device;

  public:
    // get_info methods.
    rust::Str bdf() const;
    std::array<unsigned char, 16> interface_uuid() const;
    uint32_t kdma() const;
    unsigned long max_clock_frequency_mhz() const;
    bool m2m() const;
    rust::Str name() const;
    bool nodma() const;
    bool offline() const;
    // rust::String electrical() const;
    // rust::String thermal() const;
    // rust::String mechanical() const;
    // rust::String memory() const;
    // rust::String platform() const;
    // rust::String pcie_info() const;
    // rust::String host() const;
    // rust::String dynamic_regions() const;

    // Modified member functions.
    std::array<unsigned char, 16> xclbin_uuid() const;
    std::array<unsigned char, 16> load(const Xclbin &xclbin);
  };

  std::unique_ptr<Device> new_device(unsigned int index);
  std::unique_ptr<Device> new_device_bdf(const rust::Str bdf);
}

// Kernel
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
      const rust::Str name,
      kernel_cu_access_mode mode);
}

// IP
namespace xrt
{
  // FFI constructor.
  std::unique_ptr<ip> new_ip(
      const Device &device,
      std::array<unsigned char, 16> xclbin_id,
      const rust::Str name);
}

// Ini
namespace xrt
{
  void set_ini(rust::Str key, rust::Str value);
}
