#ifndef ReorderedIndexMap_HPP
#define ReorderedIndexMap_HPP

#include "ReorderedIndexMap.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XReorderedIndexMap.h"


inline diplomat::span<const size_t> ReorderedIndexMap::as_slice() const {
  auto result = capi::ICU4XReorderedIndexMap_as_slice(this->AsFFI());
  return diplomat::span<const size_t>(result.data, result.len);
}

inline size_t ReorderedIndexMap::len() const {
  auto result = capi::ICU4XReorderedIndexMap_len(this->AsFFI());
  return result;
}

inline bool ReorderedIndexMap::is_empty() const {
  auto result = capi::ICU4XReorderedIndexMap_is_empty(this->AsFFI());
  return result;
}

inline size_t ReorderedIndexMap::get(size_t index) const {
  auto result = capi::ICU4XReorderedIndexMap_get(this->AsFFI(),
    index);
  return result;
}

inline const capi::ICU4XReorderedIndexMap* ReorderedIndexMap::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XReorderedIndexMap*>(this);
}

inline capi::ICU4XReorderedIndexMap* ReorderedIndexMap::AsFFI() {
  return reinterpret_cast<capi::ICU4XReorderedIndexMap*>(this);
}

inline const ReorderedIndexMap* ReorderedIndexMap::FromFFI(const capi::ICU4XReorderedIndexMap* ptr) {
  return reinterpret_cast<const ReorderedIndexMap*>(ptr);
}

inline ReorderedIndexMap* ReorderedIndexMap::FromFFI(capi::ICU4XReorderedIndexMap* ptr) {
  return reinterpret_cast<ReorderedIndexMap*>(ptr);
}

inline void ReorderedIndexMap::operator delete(void* ptr) {
  capi::ICU4XReorderedIndexMap_destroy(reinterpret_cast<capi::ICU4XReorderedIndexMap*>(ptr));
}


#endif // ReorderedIndexMap_HPP
