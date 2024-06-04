// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate icu_datagen;

use icu_datagen::baked_exporter;
use icu_datagen::fs_exporter;
use icu_datagen::fs_exporter::serializers::AbstractSerializer;
use icu_datagen::prelude::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

const REPO_VERSION: &str = env!("CARGO_PKG_VERSION");
const EXPERIMENTAL_VERSION: &str = "0.1.0";
const COMPONENTS: &[(&str, &[DataKey], &str)] = &[
    ("calendar", icu::calendar::provider::KEYS, REPO_VERSION),
    ("casemap", icu::casemap::provider::KEYS, REPO_VERSION),
    ("collator", icu::collator::provider::KEYS, REPO_VERSION),
    ("datetime", icu::datetime::provider::KEYS, REPO_VERSION),
    ("decimal", icu::decimal::provider::KEYS, REPO_VERSION),
    ("list", icu::list::provider::KEYS, REPO_VERSION),
    ("locale", icu::locale::provider::KEYS, REPO_VERSION),
    ("normalizer", icu::normalizer::provider::KEYS, REPO_VERSION),
    ("plurals", icu::plurals::provider::KEYS, REPO_VERSION),
    ("properties", icu::properties::provider::KEYS, REPO_VERSION),
    ("segmenter", icu::segmenter::provider::KEYS, REPO_VERSION),
    ("timezone", icu::timezone::provider::KEYS, REPO_VERSION),
    (
        "experimental",
        icu::experimental::provider::KEYS,
        EXPERIMENTAL_VERSION,
    ),
];

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let components = if args.is_empty() {
        COMPONENTS
            .iter()
            .map(|(krate, keys, version)| (krate.to_string(), *keys, *version))
            .collect::<Vec<_>>()
    } else {
        let map = std::collections::HashMap::<&str, (&'static [DataKey], &'static str)>::from_iter(
            COMPONENTS
                .iter()
                .map(|(krate, keys, version)| (*krate, (*keys, *version))),
        );
        args.into_iter()
            .filter_map(|krate| {
                map.get(krate.as_str())
                    .map(|(keys, version)| (krate, *keys, *version))
            })
            .collect()
    };

    let source = DatagenProvider::new_latest_tested();

    let driver = DatagenDriver::new()
        .with_locales_and_fallback(
            source
                .locales_for_coverage_levels([
                    CoverageLevel::Modern,
                    CoverageLevel::Moderate,
                    CoverageLevel::Basic,
                ])
                .unwrap()
                .into_iter()
                .map(LocaleFamily::with_descendants),
            Default::default(),
        )
        .with_recommended_segmenter_models();

    let mut options = baked_exporter::Options::default();
    options.overwrite = true;
    options.pretty = true;

    for (component, keys, version) in &components {
        let path = Path::new("provider/baked").join(component);

        let _ = std::fs::remove_dir_all(&path);
        for dir in ["", "src", "data"] {
            std::fs::create_dir(&path.join(dir)).unwrap();
        }
        for (file, template) in [
            ("build.rs", include_str!("../template/build.rs.template")),
            (
                "Cargo.toml",
                include_str!("../template/Cargo.toml.template"),
            ),
            ("LICENSE", include_str!("../LICENSE")),
            ("README.md", include_str!("../template/README.md.template")),
            (
                "src/lib.rs",
                include_str!("../template/src/lib.rs.template"),
            ),
        ] {
            std::fs::write(
                path.join(file),
                template
                    .replace("_component_", component)
                    .replace("_version_", version)
                    .replace("_cldr_tag_", DatagenProvider::LATEST_TESTED_CLDR_TAG)
                    .replace(
                        "_icuexport_tag_",
                        DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG,
                    )
                    .replace(
                        "_segmenter_lstm_tag_",
                        DatagenProvider::LATEST_TESTED_SEGMENTER_LSTM_TAG,
                    ),
            )
            .unwrap();
        }

        let baked_exporter =
            baked_exporter::BakedExporter::new(path.join("data"), options).unwrap();
        let fingerprinter = PostcardFingerprintExporter {
            size_hash: Default::default(),
            fingerprints: crlify::BufWriterWithLineEndingFix::new(
                File::create(path.join("fingerprints.csv")).unwrap(),
            ),
        };

        driver
            .clone()
            .with_keys(keys.iter().copied())
            .export(
                &source,
                MultiExporter::new(vec![Box::new(baked_exporter), Box::new(fingerprinter)]),
            )
            .unwrap();

        for file in ["data/any.rs", "data/mod.rs"] {
            std::fs::remove_file(path.join(file)).unwrap();
        }
    }
}

struct PostcardFingerprintExporter<F> {
    size_hash: Mutex<BTreeMap<(DataKey, String), (usize, u64)>>,
    fingerprints: F,
}

impl<F: Write + Send + Sync> DataExporter for PostcardFingerprintExporter<F> {
    fn put_payload(
        &self,
        key: DataKey,
        locale: &DataLocale,
        key_attributes: &DataKeyAttributes,
        payload_before: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let mut serialized = vec![];

        fs_exporter::serializers::Postcard::new(Default::default())
            .serialize(payload_before, &mut serialized)?;

        let size = serialized.len();

        // We're using SipHash, which is deprecated, but we want a stable hasher
        // (we're fine with it not being cryptographically secure since we're just using it to track diffs)
        #[allow(deprecated)]
        use std::hash::{Hash, Hasher, SipHasher};
        #[allow(deprecated)]
        let mut hasher = SipHasher::new();
        serialized.iter().for_each(|b| b.hash(&mut hasher));
        let hash = hasher.finish();

        self.size_hash.lock().expect("poison").insert(
            (
                key,
                DataRequest {
                    locale,
                    key_attributes,
                    ..Default::default()
                }
                .legacy_encode(),
            ),
            (size, hash),
        );

        Ok(())
    }

    fn flush(&self, _key: DataKey) -> Result<(), DataError> {
        Ok(())
    }

    fn flush_with_built_in_fallback(
        &self,
        _key: DataKey,
        _fallback_mode: BuiltInFallbackMode,
    ) -> Result<(), DataError> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        for ((key, req), (size, hash)) in self.size_hash.get_mut().expect("poison") {
            writeln!(&mut self.fingerprints, "{key}, {req}, {size}B, {hash:x}")?;
        }
        Ok(())
    }
    fn supports_built_in_fallback(&self) -> bool {
        true
    }
}
