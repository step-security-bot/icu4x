#ifndef BidiParagraph_D_HPP
#define BidiParagraph_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BidiDirection.d.hpp"
#include "ICU4XBidiParagraph.d.h"

class BidiDirection;


class BidiParagraph {
public:

  inline bool set_paragraph_in_text(size_t n);

  inline BidiDirection direction() const;

  inline size_t size() const;

  inline size_t range_start() const;

  inline size_t range_end() const;

  inline std::optional<std::string> reorder_line(size_t range_start, size_t range_end) const;

  inline uint8_t level_at(size_t pos) const;

  inline const capi::ICU4XBidiParagraph* AsFFI() const;
  inline capi::ICU4XBidiParagraph* AsFFI();
  inline static const BidiParagraph* FromFFI(const capi::ICU4XBidiParagraph* ptr);
  inline static BidiParagraph* FromFFI(capi::ICU4XBidiParagraph* ptr);
  inline static void operator delete(void* ptr);
private:
  BidiParagraph() = delete;
  BidiParagraph(const BidiParagraph&) = delete;
  BidiParagraph(BidiParagraph&&) noexcept = delete;
  BidiParagraph operator=(const BidiParagraph&) = delete;
  BidiParagraph operator=(BidiParagraph&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // BidiParagraph_D_HPP
