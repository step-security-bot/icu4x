// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::transform::cldr::cldr_serde;
use crate::provider::DatagenProvider;
use crate::provider::IterableDataProviderCached;
use icu_experimental::compactdecimal::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::convert::TryFrom;

impl DataProvider<ShortCompactDecimalFormatDataV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<ShortCompactDecimalFormatDataV1Marker>, DataError> {
        self.check_req::<ShortCompactDecimalFormatDataV1Marker>(req)?;
        let langid = req.locale.get_langid();

        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        let nsname = req.key_attributes.single().unwrap_or(numbers.default_numbering_system);

        let result = CompactDecimalPatternDataV1::try_from(
            &numbers
                .numsys_data
                .formats
                .get(&nsname)
                .ok_or_else(|| {
                    DataError::custom("Could not find formats for numbering system")
                        .with_display_context(&nsname)
                })?
                .short
                .decimal_format,
        )
        .map_err(|s| {
            DataError::custom("Could not create compact decimal patterns")
                .with_display_context(&s)
                .with_display_context(&nsname)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl DataProvider<LongCompactDecimalFormatDataV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LongCompactDecimalFormatDataV1Marker>, DataError> {
        self.check_req::<LongCompactDecimalFormatDataV1Marker>(req)?;
        let langid = req.locale.get_langid();

        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        let nsname = req.key_attributes.single().unwrap_or(numbers.default_numbering_system);

        let result = CompactDecimalPatternDataV1::try_from(
            &numbers
                .numsys_data
                .formats
                .get(&nsname)
                .ok_or_else(|| {
                    DataError::custom("Could not find formats for numbering system")
                        .with_display_context(&nsname)
                })?
                .long
                .decimal_format,
        )
        .map_err(|s| {
            DataError::custom("Could not create compact decimal patterns")
                .with_display_context(&s)
                .with_display_context(&nsname)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProviderCached<ShortCompactDecimalFormatDataV1Marker> for DatagenProvider {
    fn supported_locales_cached(
        &self,
    ) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
        self.supported_locales_for_numbers()
    }
}

impl IterableDataProviderCached<LongCompactDecimalFormatDataV1Marker> for DatagenProvider {
    fn supported_locales_cached(
        &self,
    ) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
        self.supported_locales_for_numbers()
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use icu_locale_core::langid;
    use std::borrow::Cow;
    use zerofrom::ZeroFrom;
    use zerovec::ule::AsULE;

    #[test]

    fn test_compact_long() {
        let provider = DatagenProvider::new_testing();

        let fr_compact_long: DataPayload<LongCompactDecimalFormatDataV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("en").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();

        let nonzero_copy: Box<[(i8, Count, Pattern)]> = fr_compact_long
            .get()
            .patterns
            .iter0()
            .flat_map(|kkv| {
                let key0 = *kkv.key0();
                kkv.into_iter1()
                    .map(move |(k, v)| (key0, Count::from_unaligned(*k), Pattern::zero_from(v)))
            })
            .collect();
        assert_eq!(
            nonzero_copy.as_ref(),
            [
                (
                    3,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 3,
                        literal_text: Cow::Borrowed(" thousand")
                    }
                ),
                (
                    6,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 6,
                        literal_text: Cow::Borrowed(" million")
                    }
                ),
                (
                    9,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 9,
                        literal_text: Cow::Borrowed(" billion")
                    }
                ),
                (
                    12,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 12,
                        literal_text: Cow::Borrowed(" trillion")
                    }
                ),
            ]
        );
    }

    #[test]
    fn test_compact_short() {
        let provider = DatagenProvider::new_testing();

        let ja_compact_short: DataPayload<ShortCompactDecimalFormatDataV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("ja").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();

        let nonzero_copy: Box<[(i8, Count, Pattern)]> = ja_compact_short
            .get()
            .patterns
            .iter0()
            .flat_map(|kkv| {
                let key0 = *kkv.key0();
                kkv.into_iter1()
                    .map(move |(k, v)| (key0, Count::from_unaligned(*k), Pattern::zero_from(v)))
            })
            .collect();
        assert_eq!(
            nonzero_copy.as_ref(),
            [
                (
                    4,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 4,
                        literal_text: Cow::Borrowed("万")
                    }
                ),
                (
                    8,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 8,
                        literal_text: Cow::Borrowed("億")
                    }
                ),
                (
                    12,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 12,
                        literal_text: Cow::Borrowed("兆")
                    }
                ),
                (
                    16,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 16,
                        literal_text: Cow::Borrowed("京")
                    }
                )
            ]
        );
    }
}
