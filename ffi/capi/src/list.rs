// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename("ICU4X{0}")]
pub mod ffi {
    use crate::{
        errors::ffi::ICU4XDataError, locale_core::ffi::ICU4XLocale,
        provider::ffi::ICU4XDataProvider,
    };
    use alloc::boxed::Box;

    use writeable::Writeable;

    #[diplomat::rust_link(icu::list::ListLength, Enum)]
    #[diplomat::enum_convert(icu_list::ListLength, needs_wildcard)]
    pub enum ListLength {
        Wide,
        Short,
        Narrow,
    }
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::list::ListFormatter, Struct)]
    pub struct ListFormatter(pub icu_list::ListFormatter);

    impl ListFormatter {
        /// Construct a new ICU4XListFormatter instance for And patterns
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_and_with_length, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "and_with_length")]
        pub fn create_and_with_length(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, ICU4XDataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ListFormatter(call_constructor!(
                icu_list::ListFormatter::try_new_and_with_length,
                icu_list::ListFormatter::try_new_and_with_length_with_any_provider,
                icu_list::ListFormatter::try_new_and_with_length_with_buffer_provider,
                provider,
                &locale,
                length.into()
            )?)))
        }
        /// Construct a new ICU4XListFormatter instance for And patterns
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_or_with_length, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "or_with_length")]
        pub fn create_or_with_length(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, ICU4XDataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ListFormatter(call_constructor!(
                icu_list::ListFormatter::try_new_or_with_length,
                icu_list::ListFormatter::try_new_or_with_length_with_any_provider,
                icu_list::ListFormatter::try_new_or_with_length_with_buffer_provider,
                provider,
                &locale,
                length.into()
            )?)))
        }
        /// Construct a new ICU4XListFormatter instance for And patterns
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_unit_with_length, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "unit_with_length")]
        pub fn create_unit_with_length(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, ICU4XDataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ListFormatter(call_constructor!(
                icu_list::ListFormatter::try_new_unit_with_length,
                icu_list::ListFormatter::try_new_unit_with_length_with_any_provider,
                icu_list::ListFormatter::try_new_unit_with_length_with_buffer_provider,
                provider,
                &locale,
                length.into()
            )?)))
        }

        #[diplomat::rust_link(icu::list::ListFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::list::ListFormatter::format_to_string, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::list::FormattedList, Struct, hidden)]
        #[diplomat::attr(dart, disable)]
        #[diplomat::skip_if_ast]
        pub fn format_valid_utf8(&self, list: &[&str], write: &mut DiplomatWrite) {
            let _infallible = self.0.format(list.iter()).write_to(write);
        }

        #[diplomat::rust_link(icu::list::ListFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::list::ListFormatter::format_to_string, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::list::FormattedList, Struct, hidden)]
        #[diplomat::attr(dart, disable)]
        #[diplomat::skip_if_ast]
        pub fn format(&self, list: &[&DiplomatStr], write: &mut DiplomatWrite) {
            let _infallible = self
                .0
                .format(
                    list.iter()
                        .copied()
                        .map(crate::utf::PotentiallyInvalidUtf8)
                        .map(crate::utf::LossyWrap),
                )
                .write_to(write);
        }

        #[diplomat::rust_link(icu::list::ListFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::list::ListFormatter::format_to_string, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::list::FormattedList, Struct, hidden)]
        #[diplomat::attr(dart, rename = "format")]
        #[diplomat::skip_if_ast]
        pub fn format_utf16(&self, list: &[&DiplomatStr16], write: &mut DiplomatWrite) {
            let _infallible = self
                .0
                .format(
                    list.iter()
                        .copied()
                        .map(crate::utf::PotentiallyInvalidUtf16)
                        .map(crate::utf::LossyWrap),
                )
                .write_to(write);
        }
    }
}
