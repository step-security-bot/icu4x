#ifndef Bidi_HPP
#define Bidi_HPP

#include "Bidi.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BidiInfo.hpp"
#include "ICU4XBidi.h"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ReorderedIndexMap.hpp"


inline diplomat::result<std::unique_ptr<Bidi>, ICU4XDataError> Bidi::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XBidi_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Bidi>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<Bidi>>(std::unique_ptr<Bidi>(Bidi::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Bidi>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::unique_ptr<BidiInfo> Bidi::for_text(std::string_view text, uint8_t default_level) const {
  auto result = capi::ICU4XBidi_for_text(this->AsFFI(),
    text.data(),
    text.size(),
    default_level);
  return std::unique_ptr<BidiInfo>(BidiInfo::FromFFI(result));
}

inline std::unique_ptr<ReorderedIndexMap> Bidi::reorder_visual(diplomat::span<const uint8_t> levels) const {
  auto result = capi::ICU4XBidi_reorder_visual(this->AsFFI(),
    levels.data(),
    levels.size());
  return std::unique_ptr<ReorderedIndexMap>(ReorderedIndexMap::FromFFI(result));
}

inline bool Bidi::level_is_rtl(uint8_t level) {
  auto result = capi::ICU4XBidi_level_is_rtl(level);
  return result;
}

inline bool Bidi::level_is_ltr(uint8_t level) {
  auto result = capi::ICU4XBidi_level_is_ltr(level);
  return result;
}

inline uint8_t Bidi::level_rtl() {
  auto result = capi::ICU4XBidi_level_rtl();
  return result;
}

inline uint8_t Bidi::level_ltr() {
  auto result = capi::ICU4XBidi_level_ltr();
  return result;
}

inline const capi::ICU4XBidi* Bidi::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XBidi*>(this);
}

inline capi::ICU4XBidi* Bidi::AsFFI() {
  return reinterpret_cast<capi::ICU4XBidi*>(this);
}

inline const Bidi* Bidi::FromFFI(const capi::ICU4XBidi* ptr) {
  return reinterpret_cast<const Bidi*>(ptr);
}

inline Bidi* Bidi::FromFFI(capi::ICU4XBidi* ptr) {
  return reinterpret_cast<Bidi*>(ptr);
}

inline void Bidi::operator delete(void* ptr) {
  capi::ICU4XBidi_destroy(reinterpret_cast<capi::ICU4XBidi*>(ptr));
}


#endif // Bidi_HPP
