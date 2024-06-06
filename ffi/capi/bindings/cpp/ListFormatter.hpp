#ifndef ListFormatter_HPP
#define ListFormatter_HPP

#include "ListFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XListFormatter.h"
#include "ICU4XLocale.hpp"
#include "ListLength.hpp"


inline diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError> ListFormatter::create_and_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ListLength length) {
  auto result = capi::ICU4XListFormatter_create_and_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError> ListFormatter::create_or_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ListLength length) {
  auto result = capi::ICU4XListFormatter_create_or_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError> ListFormatter::create_unit_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ListLength length) {
  auto result = capi::ICU4XListFormatter_create_unit_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::string ListFormatter::format_valid_utf8(diplomat::span<const std::string_view> list) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XListFormatter_format_valid_utf8(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline std::string ListFormatter::format(diplomat::span<const std::string_view> list) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XListFormatter_format(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline std::string ListFormatter::format_utf16(diplomat::span<const std::u16string_view> list) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XListFormatter_format_utf16(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline const capi::ICU4XListFormatter* ListFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XListFormatter*>(this);
}

inline capi::ICU4XListFormatter* ListFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XListFormatter*>(this);
}

inline const ListFormatter* ListFormatter::FromFFI(const capi::ICU4XListFormatter* ptr) {
  return reinterpret_cast<const ListFormatter*>(ptr);
}

inline ListFormatter* ListFormatter::FromFFI(capi::ICU4XListFormatter* ptr) {
  return reinterpret_cast<ListFormatter*>(ptr);
}

inline void ListFormatter::operator delete(void* ptr) {
  capi::ICU4XListFormatter_destroy(reinterpret_cast<capi::ICU4XListFormatter*>(ptr));
}


#endif // ListFormatter_HPP
