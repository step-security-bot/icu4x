import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ICU4XError_js_to_rust = {
  "UnknownError": 0,
  "OutOfBoundsError": 2,
  "DataMissingDataKeyError": 256,
  "DataMissingVariantError": 257,
  "DataMissingLocaleError": 258,
  "DataNeedsVariantError": 259,
  "DataNeedsLocaleError": 260,
  "DataExtraneousLocaleError": 261,
  "DataFilteredResourceError": 262,
  "DataMismatchedTypeError": 263,
  "DataMissingPayloadError": 264,
  "DataInvalidStateError": 265,
  "DataCustomError": 266,
  "DataIoError": 267,
  "DataUnavailableBufferFormatError": 268,
  "DataMismatchedAnyBufferError": 269,
  "LocaleUndefinedSubtagError": 512,
  "LocaleParserLanguageError": 513,
  "LocaleParserSubtagError": 514,
  "LocaleParserExtensionError": 515,
  "DataStructValidityError": 768,
  "PropertyUnexpectedPropertyNameError": 1026,
  "FixedDecimalLimitError": 1280,
  "FixedDecimalSyntaxError": 1281,
  "PluralsParseError": 1536,
  "CalendarOutOfRangeError": 1795,
  "CalendarUnknownEraError": 1796,
  "CalendarUnknownMonthCodeError": 1797,
  "DateTimePatternError": 2048,
  "DateTimeMissingInputFieldError": 2049,
  "DateTimeSkeletonError": 2050,
  "DateTimeUnsupportedFieldError": 2051,
  "DateTimeUnsupportedOptionsError": 2052,
  "DateTimeMissingWeekdaySymbolError": 2053,
  "DateTimeMissingMonthSymbolError": 2054,
  "DateTimeFixedDecimalError": 2055,
  "DateTimeMismatchedCalendarError": 2056,
  "TimeZoneInvalidOffsetError": 2561,
  "TimeZoneMissingInputError": 2562,
  "TimeZoneInvalidIdError": 2563,
  "InvalidCldrUnitIdentifierError": 3072,
};

export const ICU4XError_rust_to_js = {
  [0]: "UnknownError",
  [2]: "OutOfBoundsError",
  [256]: "DataMissingDataKeyError",
  [257]: "DataMissingVariantError",
  [258]: "DataMissingLocaleError",
  [259]: "DataNeedsVariantError",
  [260]: "DataNeedsLocaleError",
  [261]: "DataExtraneousLocaleError",
  [262]: "DataFilteredResourceError",
  [263]: "DataMismatchedTypeError",
  [264]: "DataMissingPayloadError",
  [265]: "DataInvalidStateError",
  [266]: "DataCustomError",
  [267]: "DataIoError",
  [268]: "DataUnavailableBufferFormatError",
  [269]: "DataMismatchedAnyBufferError",
  [512]: "LocaleUndefinedSubtagError",
  [513]: "LocaleParserLanguageError",
  [514]: "LocaleParserSubtagError",
  [515]: "LocaleParserExtensionError",
  [768]: "DataStructValidityError",
  [1026]: "PropertyUnexpectedPropertyNameError",
  [1280]: "FixedDecimalLimitError",
  [1281]: "FixedDecimalSyntaxError",
  [1536]: "PluralsParseError",
  [1795]: "CalendarOutOfRangeError",
  [1796]: "CalendarUnknownEraError",
  [1797]: "CalendarUnknownMonthCodeError",
  [2048]: "DateTimePatternError",
  [2049]: "DateTimeMissingInputFieldError",
  [2050]: "DateTimeSkeletonError",
  [2051]: "DateTimeUnsupportedFieldError",
  [2052]: "DateTimeUnsupportedOptionsError",
  [2053]: "DateTimeMissingWeekdaySymbolError",
  [2054]: "DateTimeMissingMonthSymbolError",
  [2055]: "DateTimeFixedDecimalError",
  [2056]: "DateTimeMismatchedCalendarError",
  [2561]: "TimeZoneInvalidOffsetError",
  [2562]: "TimeZoneMissingInputError",
  [2563]: "TimeZoneInvalidIdError",
  [3072]: "InvalidCldrUnitIdentifierError",
};

export const ICU4XError = {
  "UnknownError": "UnknownError",
  "OutOfBoundsError": "OutOfBoundsError",
  "DataMissingDataKeyError": "DataMissingDataKeyError",
  "DataMissingVariantError": "DataMissingVariantError",
  "DataMissingLocaleError": "DataMissingLocaleError",
  "DataNeedsVariantError": "DataNeedsVariantError",
  "DataNeedsLocaleError": "DataNeedsLocaleError",
  "DataExtraneousLocaleError": "DataExtraneousLocaleError",
  "DataFilteredResourceError": "DataFilteredResourceError",
  "DataMismatchedTypeError": "DataMismatchedTypeError",
  "DataMissingPayloadError": "DataMissingPayloadError",
  "DataInvalidStateError": "DataInvalidStateError",
  "DataCustomError": "DataCustomError",
  "DataIoError": "DataIoError",
  "DataUnavailableBufferFormatError": "DataUnavailableBufferFormatError",
  "DataMismatchedAnyBufferError": "DataMismatchedAnyBufferError",
  "LocaleUndefinedSubtagError": "LocaleUndefinedSubtagError",
  "LocaleParserLanguageError": "LocaleParserLanguageError",
  "LocaleParserSubtagError": "LocaleParserSubtagError",
  "LocaleParserExtensionError": "LocaleParserExtensionError",
  "DataStructValidityError": "DataStructValidityError",
  "PropertyUnexpectedPropertyNameError": "PropertyUnexpectedPropertyNameError",
  "FixedDecimalLimitError": "FixedDecimalLimitError",
  "FixedDecimalSyntaxError": "FixedDecimalSyntaxError",
  "PluralsParseError": "PluralsParseError",
  "CalendarOutOfRangeError": "CalendarOutOfRangeError",
  "CalendarUnknownEraError": "CalendarUnknownEraError",
  "CalendarUnknownMonthCodeError": "CalendarUnknownMonthCodeError",
  "DateTimePatternError": "DateTimePatternError",
  "DateTimeMissingInputFieldError": "DateTimeMissingInputFieldError",
  "DateTimeSkeletonError": "DateTimeSkeletonError",
  "DateTimeUnsupportedFieldError": "DateTimeUnsupportedFieldError",
  "DateTimeUnsupportedOptionsError": "DateTimeUnsupportedOptionsError",
  "DateTimeMissingWeekdaySymbolError": "DateTimeMissingWeekdaySymbolError",
  "DateTimeMissingMonthSymbolError": "DateTimeMissingMonthSymbolError",
  "DateTimeFixedDecimalError": "DateTimeFixedDecimalError",
  "DateTimeMismatchedCalendarError": "DateTimeMismatchedCalendarError",
  "TimeZoneInvalidOffsetError": "TimeZoneInvalidOffsetError",
  "TimeZoneMissingInputError": "TimeZoneMissingInputError",
  "TimeZoneInvalidIdError": "TimeZoneInvalidIdError",
  "InvalidCldrUnitIdentifierError": "InvalidCldrUnitIdentifierError",
};
