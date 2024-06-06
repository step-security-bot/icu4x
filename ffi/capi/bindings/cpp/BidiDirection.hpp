#ifndef BidiDirection_HPP
#define BidiDirection_HPP

#include "BidiDirection.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBidiDirection.h"


inline capi::ICU4XBidiDirection BidiDirection::AsFFI() const {
  return static_cast<capi::ICU4XBidiDirection>(value);
}

inline BidiDirection BidiDirection::FromFFI(capi::ICU4XBidiDirection c_enum) {
  switch (c_enum) {
    case capi::ICU4XBidiDirection_Ltr:
    case capi::ICU4XBidiDirection_Rtl:
    case capi::ICU4XBidiDirection_Mixed:
      return static_cast<BidiDirection::Value>(c_enum);
    default:
      abort();
  }
}
#endif // BidiDirection_HPP
