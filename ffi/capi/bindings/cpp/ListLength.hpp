#ifndef ListLength_HPP
#define ListLength_HPP

#include "ListLength.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XListLength.h"


inline capi::ICU4XListLength ListLength::AsFFI() const {
  return static_cast<capi::ICU4XListLength>(value);
}

inline ListLength ListLength::FromFFI(capi::ICU4XListLength c_enum) {
  switch (c_enum) {
    case capi::ICU4XListLength_Wide:
    case capi::ICU4XListLength_Short:
    case capi::ICU4XListLength_Narrow:
      return static_cast<ListLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ListLength_HPP
