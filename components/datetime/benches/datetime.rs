// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::Write;

use icu::calendar::{DateTime, Gregorian};
#[cfg(feature = "experimental")]
use icu::datetime::neo::TypedNeoDateTimeFormatter;
use icu::datetime::TypedDateTimeFormatter;
use icu::datetime::{time_zone::TimeZoneFormatterOptions, TypedZonedDateTimeFormatter};
use icu::timezone::CustomTimeZone;
use writeable::TryWriteable;

#[path = "../tests/mock.rs"]
mod mock;

fn datetime_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime");

    let mut bench_datetime_with_fixture = |name, file| {
        let fxs = serde_json::from_str::<fixtures::Fixture>(file).unwrap();
        group.bench_function(&format!("datetime_{name}"), |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<DateTime<Gregorian>> = fx
                        .values
                        .iter()
                        .map(|value| {
                            mock::parse_gregorian_from_str(value).expect("Failed to parse value.")
                        })
                        .collect();
                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        #[cfg(feature = "experimental")]
                        let dtf = {
                            TypedDateTimeFormatter::<Gregorian>::try_new_experimental(
                                &setup.locale.parse().expect("Failed to parse locale."), options,
                            )
                            .expect("Failed to create TypedDateTimeFormatter.")
                        };
                        #[cfg(not(feature = "experimental"))]
                        let dtf = {
                            TypedDateTimeFormatter::<Gregorian>::try_new(&locale.into(), options)
                                .expect("Failed to create TypedDateTimeFormatter.")
                        };

                        let mut result = String::new();

                        for dt in &datetimes {
                            let fdt = dtf.format(dt);
                            write!(result, "{fdt}").expect("Failed to write to date time format.");
                            result.clear();
                        }
                    }
                }
            })
        });
    };

    bench_datetime_with_fixture("lengths", include_str!("fixtures/tests/lengths.json"));

    #[cfg(feature = "experimental")]
    bench_datetime_with_fixture("components", include_str!("fixtures/tests/components.json"));

    #[cfg(feature = "experimental")]
    let mut bench_neo_datetime_with_fixture = |name, file| {
        let fxs = serde_json::from_str::<fixtures::Fixture>(file).unwrap();
        group.bench_function(&format!("neo/datetime_{name}"), |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<DateTime<Gregorian>> = fx
                        .values
                        .iter()
                        .map(|value| {
                            mock::parse_gregorian_from_str(value).expect("Failed to parse value.")
                        })
                        .collect();
                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf = {
                            TypedNeoDateTimeFormatter::<Gregorian>::try_new(&setup.locale.parse().expect("Failed to parse locale."), options)
                                .expect("Failed to create TypedNeoDateTimeFormatter.")
                        };

                        let mut result = String::new();

                        for dt in &datetimes {
                            let fdt = dtf.format(dt);
                            fdt.try_write_to(&mut result)
                                .unwrap()
                                .expect("Failed to write to date time format.");
                            result.clear();
                        }
                    }
                }
            })
        });
    };

    #[cfg(feature = "experimental")]
    bench_neo_datetime_with_fixture("lengths", include_str!("fixtures/tests/lengths.json"));

    let fxs = serde_json::from_str::<fixtures::Fixture>(include_str!(
        "fixtures/tests/lengths_with_zones.json"
    ))
    .unwrap();
    group.bench_function("zoned_datetime_overview", |b| {
        b.iter(|| {
            for fx in &fxs.0 {
                let datetimes: Vec<(DateTime<Gregorian>, CustomTimeZone)> = fx
                    .values
                    .iter()
                    .map(|value| mock::parse_zoned_gregorian_from_str(value).unwrap())
                    .collect();
                for setup in &fx.setups {
                    let options = fixtures::get_options(&setup.options).unwrap();
                    let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
                        &setup.locale.parse().unwrap(),
                        options,
                        TimeZoneFormatterOptions::default(),
                    )
                    .unwrap();

                    let mut result = String::new();

                    for dt in &datetimes {
                        let fdt = dtf.format(&dt.0, &dt.1);
                        write!(result, "{fdt}").unwrap();
                        result.clear();
                    }
                }
            }
        })
    });

    group.finish();

    #[cfg(feature = "bench")]
    {
        use writeable::Writeable;

        let mut group = c.benchmark_group("datetime");

        let fxs =
            serde_json::from_str::<fixtures::Fixture>(include_str!("fixtures/tests/lengths.json"))
                .unwrap();
        group.bench_function("TypedDateTimeFormatter/format_to_write", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<DateTime<Gregorian>> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let locale = setup.locale.parse().unwrap();
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf =
                            TypedDateTimeFormatter::<Gregorian>::try_new(&locale, options)
                                .unwrap();

                        let mut scratch = String::new();

                        for dt in &datetimes {
                            let _ = dtf.format(dt).write_to(&mut scratch);
                            scratch.clear();
                        }
                    }
                }
            })
        });

        group.bench_function("TypedDateTimeFormatter/format_to_string", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<DateTime<Gregorian>> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf =
                            TypedDateTimeFormatter::<Gregorian>::try_new(&setup.locale.parse().unwrap(), options)
                                .unwrap();

                        for dt in &datetimes {
                            let _ = dtf.format_to_string(dt);
                        }
                    }
                }
            })
        });

        group.bench_function("FormattedDateTime/format", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<DateTime<Gregorian>> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf =
                            TypedDateTimeFormatter::<Gregorian>::try_new(&setup.locale.parse().unwrap(), options)
                                .unwrap();

                        let mut result = String::new();

                        for dt in &datetimes {
                            let fdt = dtf.format(dt);
                            write!(result, "{fdt}").unwrap();
                            result.clear();
                        }
                    }
                }
            })
        });

        group.bench_function("FormattedDateTime/to_string", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<DateTime<Gregorian>> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf =
                            TypedDateTimeFormatter::<Gregorian>::try_new(&setup.locale.parse().unwrap(), options)
                                .unwrap();

                        for dt in &datetimes {
                            let fdt = dtf.format(dt);
                            let _ = fdt.to_string();
                        }
                    }
                }
            })
        });

        group.bench_function("FormattedDateTime/write_to_string", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<DateTime<Gregorian>> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf =
                            TypedDateTimeFormatter::<Gregorian>::try_new(&setup.locale.parse().unwrap(), options)
                                .unwrap();

                        for dt in &datetimes {
                            let fdt = dtf.format(dt);
                            let _ = fdt.write_to_string();
                        }
                    }
                }
            })
        });

        let fxs = serde_json::from_str::<fixtures::Fixture>(include_str!(
            "fixtures/tests/lengths_with_zones.json"
        ))
        .unwrap();
        group.bench_function("TypedZonedDateTimeFormatter/format_to_write", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<(DateTime<Gregorian>, CustomTimeZone)> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_zoned_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
                            &setup.locale.parse().unwrap(),
                            options,
                            TimeZoneFormatterOptions::default(),
                        )
                        .unwrap();

                        let mut scratch = String::new();

                        for dt in &datetimes {
                            let _ = dtf.format(&dt.0, &dt.1).write_to(&mut scratch);
                            scratch.clear();
                        }
                    }
                }
            })
        });

        group.bench_function("TypedZonedDateTimeFormatter/format_to_string", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<(DateTime<Gregorian>, CustomTimeZone)> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_zoned_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
                            &setup.locale.parse().unwrap(),
                            options,
                            TimeZoneFormatterOptions::default(),
                        )
                        .unwrap();

                        for dt in &datetimes {
                            let _ = dtf.format_to_string(&dt.0, &dt.1);
                        }
                    }
                }
            })
        });

        group.bench_function("FormattedZonedDateTime/format", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<(DateTime<Gregorian>, CustomTimeZone)> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_zoned_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
                            &setup.locale.parse().unwrap(),
                            options,
                            TimeZoneFormatterOptions::default(),
                        )
                        .unwrap();

                        let mut result = String::new();

                        for dt in &datetimes {
                            let fdt = dtf.format(&dt.0, &dt.1);
                            write!(result, "{fdt}").unwrap();
                            result.clear();
                        }
                    }
                }
            })
        });

        group.bench_function("FormattedZonedDateTime/to_string", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<(DateTime<Gregorian>, CustomTimeZone)> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_zoned_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
                            &setup.locale.parse().unwrap(),
                            options,
                            TimeZoneFormatterOptions::default(),
                        )
                        .unwrap();

                        for dt in &datetimes {
                            let fdt = dtf.format(&dt.0, &dt.1);
                            let _ = fdt.to_string();
                        }
                    }
                }
            })
        });

        group.bench_function("FormattedZonedDateTime/write_to_string", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<(DateTime<Gregorian>, CustomTimeZone)> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_zoned_gregorian_from_str(value).unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let options = fixtures::get_options(&setup.options).unwrap();
                        let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
                            &setup.locale.parse().unwrap(),
                            options,
                            TimeZoneFormatterOptions::default(),
                        )
                        .unwrap();

                        for dt in &datetimes {
                            let fdt = dtf.format(&dt.0, &dt.1);
                            let _ = fdt.write_to_string();
                        }
                    }
                }
            })
        });

        group.finish();
    }
}

criterion_group!(benches, datetime_benches);
criterion_main!(benches);
