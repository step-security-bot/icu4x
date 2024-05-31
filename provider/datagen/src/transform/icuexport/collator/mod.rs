// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::provider::DatagenProvider;
use crate::provider::IterableDataProviderCached;
use icu_collator::provider::*;
use icu_collections::codepointtrie::CodePointTrie;
use icu_locale::provider::CollationFallbackSupplementV1Marker;
use icu_locale::provider::LocaleFallbackSupplementV1;
use icu_locale_core::extensions::unicode::key;
use icu_locale_core::subtags::language;
use icu_locale_core::subtags::Language;
use icu_locale_core::subtags::Region;
use icu_locale_core::subtags::Script;
use icu_locale_core::LanguageIdentifier;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::convert::TryFrom;
use tinystr::tinystr;
use writeable::Writeable;
use zerovec::ule::UnvalidatedStr;
use zerovec::ZeroVec;

mod collator_serde;

impl DataProvider<CollationFallbackSupplementV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CollationFallbackSupplementV1Marker>, DataError> {
        self.check_req::<CollationFallbackSupplementV1Marker>(req)?;

        let parent_locales = &self
            .cldr()?
            .core()
            .read_and_parse::<crate::provider::transform::cldr::cldr_serde::parent_locales::Resource>(
                "supplemental/parentLocales.json",
            )?.supplemental.parent_locales;

        let additional = if parent_locales
            .rules
            .collations
            .as_ref()
            .map(|c| &c.non_likely_scripts)
            != Some(&String::from("root"))
        {
            let collation_locales = self
                .icuexport()?
                .list(&format!("collation/{}", self.collation_han_database()))?
                .filter_map(|s| {
                    Some(
                        file_name_to_req(
                            s.rsplit_once('_')?.0,
                            self.has_legacy_swedish_variants(),
                        )?
                        .0
                        .language(),
                    )
                })
                .collect::<HashSet<_>>();

            parent_locales
                .parent_locale
                .iter()
                .filter(|(k, _)| collation_locales.contains(&k.language))
                .filter(|(from, to)| {
                    // Script gets removed while language changes. For collation we want to insert the script-removal as its
                    // own step.
                    from.script.is_some() && to.script.is_none() && from.language != to.language
                })
                .map(|(from, _)| (from.to_string(), (from.language, None, from.region)))
                .collect()
        } else {
            HashSet::new()
        };

        let parents = additional
            .iter()
            .map(|(k, v)| (UnvalidatedStr::from_str(k), *v))
            .chain(parent_locales.collations.iter().map(|(from, to)| {
                (
                    <&UnvalidatedStr>::from(from.as_str()),
                    <(Language, Option<Script>, Option<Region>)>::from(to),
                )
            }))
            .collect();

        let data = LocaleFallbackSupplementV1 {
            parents,
            unicode_extension_defaults: [
                (
                    key!("co"),
                    <&UnvalidatedStr>::from("zh"),
                    <&UnvalidatedStr>::from("pinyin"),
                ),
                (
                    key!("co"),
                    <&UnvalidatedStr>::from("zh-Hant"),
                    <&UnvalidatedStr>::from("stroke"),
                ),
            ]
            .into_iter()
            .collect(),
        };
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

impl IterableDataProviderCached<CollationFallbackSupplementV1Marker> for DatagenProvider {
    fn supported_locales_cached(
        &self,
    ) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DatagenProvider {
    /// Backward compatibility for https://unicode-org.atlassian.net/browse/CLDR-15603
    fn has_legacy_swedish_variants(&self) -> bool {
        self.icuexport()
            .and_then(|i| {
                i.file_exists(&format!(
                    "collation/{}/sv_reformed_meta.toml",
                    self.collation_han_database(),
                ))
            })
            .unwrap_or(false)
    }
}

fn req_to_file_name(
    locale: &DataLocale,
    key_attributes: &DataKeyAttributes,
    has_legacy_swedish_variants: bool,
) -> String {
    let mut s = if locale.get_langid() == LanguageIdentifier::UND {
        "root".to_owned()
    } else {
        locale
            .get_langid()
            .write_to_string()
            .replace('-', "_")
            .replace("posix", "POSIX")
    };

    if !key_attributes.is_empty() {
        s.push('_');
        s.push_str(match key_attributes as &str {
            "trad" => "traditional",
            "phonebk" => "phonebook",
            "dict" => "dictionary",
            "gb2312" => "gb2312han",
            extension => extension,
        });
    } else if locale.get_langid().language == language!("zh") {
        // "zh" uses "_pinyin" as the default
        s.push_str("_pinyin");
    } else if has_legacy_swedish_variants && locale.get_langid().language == language!("sv") {
        // "sv" used to use "_reformed" as the default
        // TODO(#2856): Remove when dropping pre-42 support in 2.0
        s.push_str("_reformed");
    } else {
        // Everyting else uses "_standard"
        s.push_str("_standard");
    }
    s
}

fn file_name_to_req(
    file_name: &str,
    has_legacy_swedish_variants: bool,
) -> Option<(DataLocale, DataKeyAttributes)> {
    let (language, variant) = file_name.rsplit_once('_').unwrap();
    let locale = if language == "root" {
        DataLocale::default()
    } else {
        language.parse().ok()?
    };

    // See above for the two special cases.
    if language == "zh" {
        if variant == "pinyin" {
            return Some((locale, Default::default()));
        }
    } else if has_legacy_swedish_variants && language == "sv" {
        // TODO(#2856): Remove when dropping pre-42 support in 2.0
        if variant == "reformed" {
            return Some((locale, Default::default()));
        }
    } else if variant == "standard" {
        return Some((locale, Default::default()));
    }

    let key_attributes = match variant {
        "traditional" => DataKeyAttributes::from_tinystr(tinystr!(8, "trad")),
        "phonebook" => DataKeyAttributes::from_tinystr(tinystr!(8, "phonebk")),
        "dictionary" => DataKeyAttributes::from_tinystr(tinystr!(8, "dict")),
        "gb2312han" => DataKeyAttributes::from_tinystr(tinystr!(8, "gb2312")),
        _ => variant.parse().unwrap(),
    };

    Some((locale, key_attributes))
}

macro_rules! collation_provider {
    ($(($marker:ident, $serde_struct:ident, $suffix:literal, $conversion:expr)),+, $toml_data:ident) => {
        $(
            impl DataProvider<$marker> for DatagenProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let $toml_data: &collator_serde::$serde_struct = self
                        .icuexport()?
                        .read_and_parse_toml(&format!(
                            "collation/{}/{}{}.toml",
                            self.collation_han_database(),
                            req_to_file_name(&req.locale, &req.key_attributes, self.has_legacy_swedish_variants()),
                            $suffix
                        ))
                        .map_err(|e| match e.kind {
                            DataErrorKind::Io(std::io::ErrorKind::NotFound) => {
                                DataErrorKind::MissingLocale.with_req($marker::KEY, req)
                            }
                            _ => e,
                        })?;

                    Ok(DataResponse {
                        metadata: DataResponseMetadata::default(),
                        // The struct conversion is macro-based instead of
                        // using a method on the Serde struct, because the
                        // method approach caused lifetime issues that I
                        // didn't know how to solve.
                        payload: Some(DataPayload::from_owned($conversion)),
                    })
                }
            }

            impl IterableDataProviderCached<$marker> for DatagenProvider {
                fn supported_locales_cached(&self) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
                    Ok(self
                        .icuexport()?
                        .list(&format!(
                            "collation/{}",
                            self.collation_han_database()
                        ))?
                        .filter_map(|mut file_name| {
                            file_name.truncate(file_name.len() - ".toml".len());
                            file_name.ends_with($suffix).then(|| {
                                file_name.truncate(file_name.len() - $suffix.len());
                                file_name
                            })
                        })
                        .filter_map(|s| file_name_to_req(&s, self.has_legacy_swedish_variants()))
                        .collect())
                }
            }
        )+
    };
}

collation_provider!(
    (
        CollationDataV1Marker,
        CollationData,
        "_data",
        icu_collator::provider::CollationDataV1 {
            trie: CodePointTrie::<u32>::try_from(&toml_data.trie)
                .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?,
            contexts: ZeroVec::alloc_from_slice(&toml_data.contexts),
            ce32s: ZeroVec::alloc_from_slice(&toml_data.ce32s),
            ces: toml_data.ces.iter().map(|i| *i as u64).collect(),
        }
    ),
    (
        CollationDiacriticsV1Marker,
        CollationDiacritics,
        "_dia",
        icu_collator::provider::CollationDiacriticsV1 {
            secondaries: ZeroVec::alloc_from_slice(&toml_data.secondaries),
        }
    ),
    (
        CollationJamoV1Marker,
        CollationJamo,
        "_jamo",
        icu_collator::provider::CollationJamoV1 {
            ce32s: ZeroVec::alloc_from_slice(&toml_data.ce32s),
        }
    ),
    (
        CollationMetadataV1Marker,
        CollationMetadata,
        "_meta",
        icu_collator::provider::CollationMetadataV1 {
            bits: toml_data.bits,
        }
    ),
    (
        CollationReorderingV1Marker,
        CollationReordering,
        "_reord",
        icu_collator::provider::CollationReorderingV1 {
            min_high_no_reorder: toml_data.min_high_no_reorder,
            reorder_table: ZeroVec::alloc_from_slice(&toml_data.reorder_table),
            reorder_ranges: ZeroVec::alloc_from_slice(&toml_data.reorder_ranges),
        }
    ),
    (
        CollationSpecialPrimariesV1Marker,
        CollationSpecialPrimaries,
        "_prim",
        icu_collator::provider::CollationSpecialPrimariesV1 {
            last_primaries: ZeroVec::alloc_from_slice(&toml_data.last_primaries),
            numeric_primary: toml_data.numeric_primary,
        }
    ),
    toml_data
);
