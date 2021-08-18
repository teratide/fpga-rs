#pragma once
#include "rust/cxx.h"

#include "fpga-xrt/xrt/src/runtime_src/core/include/xrt/xrt_device.h"
#include "fpga-xrt/xrt/src/runtime_src/core/include/xrt/xrt_uuid.h"

#include <memory>

namespace xrt {

class Device: public xrt::device {
  using xrt::device::device;
public:
  rust::String name() const;
  rust::String bdf() const;

  std::array<unsigned char, 16> xclbin_uuid() const;
};

std::unique_ptr<Device> new_device(unsigned int index);

}