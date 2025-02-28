// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::time_zone;
use serde::{Deserialize, Serialize};
use tinystr::TinyAsciiStr;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneConfig {
    pub time_zone_id: Option<TinyAsciiStr<8>>,
    pub metazone_id: Option<TinyAsciiStr<4>>,
    pub time_variant: Option<TinyAsciiStr<2>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTests(pub Vec<TimeZoneTest>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTest {
    pub locale: String,
    pub config: TimeZoneConfig,
    pub datetime: String,
    pub expectations: Vec<TimeZoneExpectation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneExpectation {
    pub patterns: Vec<String>,
    pub configs: Vec<TimeZoneFormatterConfig>,
    pub fallback_formats: Vec<FallbackFormat>,
    pub expected: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FallbackFormat {
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds),
    LocalizedGmt,
}

impl From<FallbackFormat> for time_zone::TimeZoneFormatterOptions {
    fn from(other: FallbackFormat) -> time_zone::TimeZoneFormatterOptions {
        match other {
            FallbackFormat::Iso8601(iso_format, iso_minutes, iso_seconds) => {
                time_zone::TimeZoneFormatterOptions {
                    fallback_format: time_zone::FallbackFormat::Iso8601(
                        iso_format.into(),
                        iso_minutes.into(),
                        iso_seconds.into(),
                    ),
                }
            }
            FallbackFormat::LocalizedGmt => time_zone::TimeZoneFormatterOptions {
                fallback_format: time_zone::FallbackFormat::LocalizedGmt,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IsoSeconds {
    Optional,
    Never,
}

impl From<IsoSeconds> for time_zone::IsoSeconds {
    fn from(other: IsoSeconds) -> time_zone::IsoSeconds {
        match other {
            IsoSeconds::Optional => time_zone::IsoSeconds::Optional,
            IsoSeconds::Never => time_zone::IsoSeconds::Never,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IsoMinutes {
    Required,
    Optional,
}

impl From<IsoMinutes> for time_zone::IsoMinutes {
    fn from(other: IsoMinutes) -> time_zone::IsoMinutes {
        match other {
            IsoMinutes::Required => time_zone::IsoMinutes::Required,
            IsoMinutes::Optional => time_zone::IsoMinutes::Optional,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ZeroPadding {
    On,
    Off,
}

impl From<ZeroPadding> for time_zone::ZeroPadding {
    fn from(other: ZeroPadding) -> time_zone::ZeroPadding {
        match other {
            ZeroPadding::On => time_zone::ZeroPadding::On,
            ZeroPadding::Off => time_zone::ZeroPadding::Off,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IsoFormat {
    Basic,
    Extended,
    UtcBasic,
    UtcExtended,
}

impl From<IsoFormat> for time_zone::IsoFormat {
    fn from(other: IsoFormat) -> time_zone::IsoFormat {
        match other {
            IsoFormat::Basic => time_zone::IsoFormat::Basic,
            IsoFormat::Extended => time_zone::IsoFormat::Extended,
            IsoFormat::UtcBasic => time_zone::IsoFormat::UtcBasic,
            IsoFormat::UtcExtended => time_zone::IsoFormat::UtcExtended,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimeZoneFormatterConfig {
    GenericNonLocationLong,
    GenericNonLocationShort,
    GenericLocation,
    SpecificNonLocationLong,
    SpecificNonLocationShort,
    LocalizedGMT,
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds),
}

impl From<TimeZoneFormatterConfig> for time_zone::TimeZoneFormatterConfig {
    fn from(other: TimeZoneFormatterConfig) -> time_zone::TimeZoneFormatterConfig {
        match other {
            TimeZoneFormatterConfig::GenericNonLocationLong => {
                time_zone::TimeZoneFormatterConfig::GenericNonLocationLong
            }
            TimeZoneFormatterConfig::GenericNonLocationShort => {
                time_zone::TimeZoneFormatterConfig::GenericNonLocationShort
            }
            TimeZoneFormatterConfig::GenericLocation => {
                time_zone::TimeZoneFormatterConfig::GenericLocation
            }
            TimeZoneFormatterConfig::SpecificNonLocationLong => {
                time_zone::TimeZoneFormatterConfig::SpecificNonLocationLong
            }
            TimeZoneFormatterConfig::SpecificNonLocationShort => {
                time_zone::TimeZoneFormatterConfig::SpecificNonLocationShort
            }
            TimeZoneFormatterConfig::LocalizedGMT => {
                time_zone::TimeZoneFormatterConfig::LocalizedGMT
            }
            TimeZoneFormatterConfig::Iso8601(iso_format, iso_minutes, iso_seconds) => {
                time_zone::TimeZoneFormatterConfig::Iso8601(
                    iso_format.into(),
                    iso_minutes.into(),
                    iso_seconds.into(),
                )
            }
        }
    }
}
