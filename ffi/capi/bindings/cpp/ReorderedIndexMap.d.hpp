#ifndef ReorderedIndexMap_D_HPP
#define ReorderedIndexMap_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XReorderedIndexMap.d.h"


class ReorderedIndexMap {
public:

  inline diplomat::span<const size_t> as_slice() const;

  inline size_t len() const;

  inline bool is_empty() const;

  inline size_t get(size_t index) const;

  inline const capi::ICU4XReorderedIndexMap* AsFFI() const;
  inline capi::ICU4XReorderedIndexMap* AsFFI();
  inline static const ReorderedIndexMap* FromFFI(const capi::ICU4XReorderedIndexMap* ptr);
  inline static ReorderedIndexMap* FromFFI(capi::ICU4XReorderedIndexMap* ptr);
  inline static void operator delete(void* ptr);
private:
  ReorderedIndexMap() = delete;
  ReorderedIndexMap(const ReorderedIndexMap&) = delete;
  ReorderedIndexMap(ReorderedIndexMap&&) noexcept = delete;
  ReorderedIndexMap operator=(const ReorderedIndexMap&) = delete;
  ReorderedIndexMap operator=(ReorderedIndexMap&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ReorderedIndexMap_D_HPP
