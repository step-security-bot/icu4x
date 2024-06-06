#ifndef BidiInfo_HPP
#define BidiInfo_HPP

#include "BidiInfo.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BidiParagraph.hpp"
#include "ICU4XBidiInfo.h"


inline size_t BidiInfo::paragraph_count() const {
  auto result = capi::ICU4XBidiInfo_paragraph_count(this->AsFFI());
  return result;
}

inline std::unique_ptr<BidiParagraph> BidiInfo::paragraph_at(size_t n) const {
  auto result = capi::ICU4XBidiInfo_paragraph_at(this->AsFFI(),
    n);
  return std::unique_ptr<BidiParagraph>(BidiParagraph::FromFFI(result));
}

inline size_t BidiInfo::size() const {
  auto result = capi::ICU4XBidiInfo_size(this->AsFFI());
  return result;
}

inline uint8_t BidiInfo::level_at(size_t pos) const {
  auto result = capi::ICU4XBidiInfo_level_at(this->AsFFI(),
    pos);
  return result;
}

inline const capi::ICU4XBidiInfo* BidiInfo::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XBidiInfo*>(this);
}

inline capi::ICU4XBidiInfo* BidiInfo::AsFFI() {
  return reinterpret_cast<capi::ICU4XBidiInfo*>(this);
}

inline const BidiInfo* BidiInfo::FromFFI(const capi::ICU4XBidiInfo* ptr) {
  return reinterpret_cast<const BidiInfo*>(ptr);
}

inline BidiInfo* BidiInfo::FromFFI(capi::ICU4XBidiInfo* ptr) {
  return reinterpret_cast<BidiInfo*>(ptr);
}

inline void BidiInfo::operator delete(void* ptr) {
  capi::ICU4XBidiInfo_destroy(reinterpret_cast<capi::ICU4XBidiInfo*>(ptr));
}


#endif // BidiInfo_HPP
