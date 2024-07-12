#ifndef WeekCalculator_HPP
#define WeekCalculator_HPP

#include "WeekCalculator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "IsoWeekday.hpp"
#include "Locale.hpp"
#include "WeekendContainsDay.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XWeekCalculator_create_result {union {WeekCalculator* ok; DataError err;}; bool is_ok;} ICU4XWeekCalculator_create_result;
    ICU4XWeekCalculator_create_result ICU4XWeekCalculator_create(const DataProvider* provider, const Locale* locale);
    
    WeekCalculator* ICU4XWeekCalculator_create_from_first_day_of_week_and_min_week_days(IsoWeekday first_weekday, uint8_t min_week_days);
    
    IsoWeekday ICU4XWeekCalculator_first_weekday(const WeekCalculator* self);
    
    uint8_t ICU4XWeekCalculator_min_week_days(const WeekCalculator* self);
    
    WeekendContainsDay ICU4XWeekCalculator_weekend(const WeekCalculator* self);
    
    
    void ICU4XWeekCalculator_destroy(WeekCalculator* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<WeekCalculator>, DataError> WeekCalculator::create(const DataProvider& provider, const Locale& locale) {
  auto result = capi::ICU4XWeekCalculator_create(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<WeekCalculator>, DataError>(diplomat::Ok<std::unique_ptr<WeekCalculator>>(std::unique_ptr<WeekCalculator>(WeekCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<WeekCalculator>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<WeekCalculator> WeekCalculator::create_from_first_day_of_week_and_min_week_days(IsoWeekday first_weekday, uint8_t min_week_days) {
  auto result = capi::ICU4XWeekCalculator_create_from_first_day_of_week_and_min_week_days(first_weekday.AsFFI(),
    min_week_days);
  return std::unique_ptr<WeekCalculator>(WeekCalculator::FromFFI(result));
}

inline IsoWeekday WeekCalculator::first_weekday() const {
  auto result = capi::ICU4XWeekCalculator_first_weekday(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint8_t WeekCalculator::min_week_days() const {
  auto result = capi::ICU4XWeekCalculator_min_week_days(this->AsFFI());
  return result;
}

inline WeekendContainsDay WeekCalculator::weekend() const {
  auto result = capi::ICU4XWeekCalculator_weekend(this->AsFFI());
  return WeekendContainsDay::FromFFI(result);
}

inline const capi::WeekCalculator* WeekCalculator::AsFFI() const {
  return reinterpret_cast<const capi::WeekCalculator*>(this);
}

inline capi::WeekCalculator* WeekCalculator::AsFFI() {
  return reinterpret_cast<capi::WeekCalculator*>(this);
}

inline const WeekCalculator* WeekCalculator::FromFFI(const capi::WeekCalculator* ptr) {
  return reinterpret_cast<const WeekCalculator*>(ptr);
}

inline WeekCalculator* WeekCalculator::FromFFI(capi::WeekCalculator* ptr) {
  return reinterpret_cast<WeekCalculator*>(ptr);
}

inline void WeekCalculator::operator delete(void* ptr) {
  capi::ICU4XWeekCalculator_destroy(reinterpret_cast<capi::WeekCalculator*>(ptr));
}


#endif // WeekCalculator_HPP