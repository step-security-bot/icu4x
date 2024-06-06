#ifndef ListFormatter_D_HPP
#define ListFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XListFormatter.d.h"
#include "ListLength.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XDataError;
class ListLength;


class ListFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError> create_and_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ListLength length);

  inline static diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError> create_or_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ListLength length);

  inline static diplomat::result<std::unique_ptr<ListFormatter>, ICU4XDataError> create_unit_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ListLength length);

  inline std::string format_valid_utf8(diplomat::span<const std::string_view> list) const;

  inline std::string format(diplomat::span<const std::string_view> list) const;

  inline std::string format_utf16(diplomat::span<const std::u16string_view> list) const;

  inline const capi::ICU4XListFormatter* AsFFI() const;
  inline capi::ICU4XListFormatter* AsFFI();
  inline static const ListFormatter* FromFFI(const capi::ICU4XListFormatter* ptr);
  inline static ListFormatter* FromFFI(capi::ICU4XListFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ListFormatter() = delete;
  ListFormatter(const ListFormatter&) = delete;
  ListFormatter(ListFormatter&&) noexcept = delete;
  ListFormatter operator=(const ListFormatter&) = delete;
  ListFormatter operator=(ListFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ListFormatter_D_HPP
