#ifndef ICU4XLocale_HPP
#define ICU4XLocale_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XLocale.h"

class ICU4XLocale;
#include "ICU4XError.hpp"
#include "ICU4XOrdering.hpp"

/**
 * A destruction policy for using ICU4XLocale with std::unique_ptr.
 */
struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};

/**
 * An ICU4X Locale, capable of representing strings like `"en-US"`.
 * 
 * See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html) for more information.
 */
class ICU4XLocale {
 public:

  /**
   * Construct an [`ICU4XLocale`] from an locale identifier.
   * 
   * This will run the complete locale parsing algorithm. If code size and
   * performance are critical and the locale is of a known shape (such as
   * `aa-BB`) use `create_und`, `set_language`, `set_script`, and `set_region`.
   * 
   * See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_bytes) for more information.
   */
  static diplomat::result<ICU4XLocale, ICU4XError> create_from_string(const std::string_view name);

  /**
   * Construct a default undefined [`ICU4XLocale`] "und".
   * 
   * See the [Rust documentation for `UND`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#associatedconstant.UND) for more information.
   */
  static ICU4XLocale create_und();

  /**
   * Clones the [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html) for more information.
   */
  ICU4XLocale clone() const;

  /**
   * Returns a string representation of the `LanguageIdentifier` part of
   * [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id) for more information.
   */
  template<typename W> void basename_to_write(W& write) const;

  /**
   * Returns a string representation of the `LanguageIdentifier` part of
   * [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id) for more information.
   */
  std::string basename() const;

  /**
   * Returns a string representation of the unicode extension.
   * 
   * See the [Rust documentation for `extensions`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.extensions) for more information.
   */
  template<typename W> std::optional<std::monostate> get_unicode_extension_to_write(const std::string_view bytes, W& write) const;

  /**
   * Returns a string representation of the unicode extension.
   * 
   * See the [Rust documentation for `extensions`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.extensions) for more information.
   */
  std::optional<std::string> get_unicode_extension(const std::string_view bytes) const;

  /**
   * Returns a string representation of [`ICU4XLocale`] language.
   * 
   * See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id) for more information.
   */
  template<typename W> void language_to_write(W& write) const;

  /**
   * Returns a string representation of [`ICU4XLocale`] language.
   * 
   * See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id) for more information.
   */
  std::string language() const;

  /**
   * Set the language part of the [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_bytes) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> set_language(const std::string_view bytes);

  /**
   * Returns a string representation of [`ICU4XLocale`] region.
   * 
   * See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id) for more information.
   */
  template<typename W> std::optional<std::monostate> region_to_write(W& write) const;

  /**
   * Returns a string representation of [`ICU4XLocale`] region.
   * 
   * See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id) for more information.
   */
  std::optional<std::string> region() const;

  /**
   * Set the region part of the [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_bytes) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> set_region(const std::string_view bytes);

  /**
   * Returns a string representation of [`ICU4XLocale`] script.
   * 
   * See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id) for more information.
   */
  template<typename W> std::optional<std::monostate> script_to_write(W& write) const;

  /**
   * Returns a string representation of [`ICU4XLocale`] script.
   * 
   * See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id) for more information.
   */
  std::optional<std::string> script() const;

  /**
   * Set the script part of the [`ICU4XLocale`]. Pass an empty string to remove the script.
   * 
   * See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_bytes) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> set_script(const std::string_view bytes);

  /**
   * Best effort locale canonicalizer that doesn't need any data
   * 
   * Use ICU4XLocaleCanonicalizer for better control and functionality
   * 
   * See the [Rust documentation for `canonicalize`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.canonicalize) for more information.
   */
  template<typename W> static diplomat::result<std::monostate, ICU4XError> canonicalize_to_write(const std::string_view bytes, W& write);

  /**
   * Best effort locale canonicalizer that doesn't need any data
   * 
   * Use ICU4XLocaleCanonicalizer for better control and functionality
   * 
   * See the [Rust documentation for `canonicalize`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.canonicalize) for more information.
   */
  static diplomat::result<std::string, ICU4XError> canonicalize(const std::string_view bytes);

  /**
   * Returns a string representation of [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `write_to`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.write_to) for more information.
   */
  template<typename W> void to_string_to_write(W& write) const;

  /**
   * Returns a string representation of [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `write_to`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.write_to) for more information.
   */
  std::string to_string() const;

  /**
   * See the [Rust documentation for `normalizing_eq`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.normalizing_eq) for more information.
   */
  bool normalizing_eq(const std::string_view other) const;

  /**
   * See the [Rust documentation for `strict_cmp`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.strict_cmp) for more information.
   */
  ICU4XOrdering strict_cmp(const std::string_view other) const;

  /**
   * See the [Rust documentation for `total_cmp`](https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.total_cmp) for more information.
   */
  ICU4XOrdering total_cmp(const ICU4XLocale& other) const;

  /**
   * Deprecated
   * 
   * Use `create_from_string("en").
   */
  static ICU4XLocale create_en();

  /**
   * Deprecated
   * 
   * Use `create_from_string("bn").
   */
  static ICU4XLocale create_bn();
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocale* AsFFIMut() { return this->inner.get(); }
  inline explicit ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
  ICU4XLocale() = default;
  ICU4XLocale(ICU4XLocale&&) noexcept = default;
  ICU4XLocale& operator=(ICU4XLocale&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};


inline diplomat::result<ICU4XLocale, ICU4XError> ICU4XLocale::create_from_string(const std::string_view name) {
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_create_from_string(name.data(), name.size());
  diplomat::result<ICU4XLocale, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XLocale>(ICU4XLocale(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline ICU4XLocale ICU4XLocale::create_und() {
  return ICU4XLocale(capi::ICU4XLocale_create_und());
}
inline ICU4XLocale ICU4XLocale::clone() const {
  return ICU4XLocale(capi::ICU4XLocale_clone(this->inner.get()));
}
template<typename W> inline void ICU4XLocale::basename_to_write(W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  capi::ICU4XLocale_basename(this->inner.get(), &write_writer);
}
inline std::string ICU4XLocale::basename() const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  capi::ICU4XLocale_basename(this->inner.get(), &diplomat_write_out);
  return diplomat_write_string;
}
template<typename W> inline std::optional<std::monostate> ICU4XLocale::get_unicode_extension_to_write(const std::string_view bytes, W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_get_unicode_extension(this->inner.get(), bytes.data(), bytes.size(), &write_writer);
  std::optional<std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = std::optional<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = std::nullopt;
  }
  return diplomat_result_out_value;
}
inline std::optional<std::string> ICU4XLocale::get_unicode_extension(const std::string_view bytes) const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_get_unicode_extension(this->inner.get(), bytes.data(), bytes.size(), &diplomat_write_out);
  std::optional<std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = std::optional<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = std::nullopt;
  }
  return diplomat_result_out_value.has_value() ? std::optional<std::string>{std::move(diplomat_write_string)} : std::nullopt;
}
template<typename W> inline void ICU4XLocale::language_to_write(W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  capi::ICU4XLocale_language(this->inner.get(), &write_writer);
}
inline std::string ICU4XLocale::language() const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  capi::ICU4XLocale_language(this->inner.get(), &diplomat_write_out);
  return diplomat_write_string;
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::set_language(const std::string_view bytes) {
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_set_language(this->inner.get(), bytes.data(), bytes.size());
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
template<typename W> inline std::optional<std::monostate> ICU4XLocale::region_to_write(W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_region(this->inner.get(), &write_writer);
  std::optional<std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = std::optional<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = std::nullopt;
  }
  return diplomat_result_out_value;
}
inline std::optional<std::string> ICU4XLocale::region() const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_region(this->inner.get(), &diplomat_write_out);
  std::optional<std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = std::optional<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = std::nullopt;
  }
  return diplomat_result_out_value.has_value() ? std::optional<std::string>{std::move(diplomat_write_string)} : std::nullopt;
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::set_region(const std::string_view bytes) {
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_set_region(this->inner.get(), bytes.data(), bytes.size());
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
template<typename W> inline std::optional<std::monostate> ICU4XLocale::script_to_write(W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_script(this->inner.get(), &write_writer);
  std::optional<std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = std::optional<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = std::nullopt;
  }
  return diplomat_result_out_value;
}
inline std::optional<std::string> ICU4XLocale::script() const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_script(this->inner.get(), &diplomat_write_out);
  std::optional<std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = std::optional<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = std::nullopt;
  }
  return diplomat_result_out_value.has_value() ? std::optional<std::string>{std::move(diplomat_write_string)} : std::nullopt;
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::set_script(const std::string_view bytes) {
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_set_script(this->inner.get(), bytes.data(), bytes.size());
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::canonicalize_to_write(const std::string_view bytes, W& write) {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_canonicalize(bytes.data(), bytes.size(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XLocale::canonicalize(const std::string_view bytes) {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_canonicalize(bytes.data(), bytes.size(), &diplomat_write_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_write_string));
}
template<typename W> inline void ICU4XLocale::to_string_to_write(W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  capi::ICU4XLocale_to_string(this->inner.get(), &write_writer);
}
inline std::string ICU4XLocale::to_string() const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  capi::ICU4XLocale_to_string(this->inner.get(), &diplomat_write_out);
  return diplomat_write_string;
}
inline bool ICU4XLocale::normalizing_eq(const std::string_view other) const {
  return capi::ICU4XLocale_normalizing_eq(this->inner.get(), other.data(), other.size());
}
inline ICU4XOrdering ICU4XLocale::strict_cmp(const std::string_view other) const {
  return static_cast<ICU4XOrdering>(capi::ICU4XLocale_strict_cmp(this->inner.get(), other.data(), other.size()));
}
inline ICU4XOrdering ICU4XLocale::total_cmp(const ICU4XLocale& other) const {
  return static_cast<ICU4XOrdering>(capi::ICU4XLocale_total_cmp(this->inner.get(), other.AsFFI()));
}
inline ICU4XLocale ICU4XLocale::create_en() {
  return ICU4XLocale(capi::ICU4XLocale_create_en());
}
inline ICU4XLocale ICU4XLocale::create_bn() {
  return ICU4XLocale(capi::ICU4XLocale_create_bn());
}
#endif
