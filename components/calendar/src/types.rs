// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains various types used by `icu_calendar` and `icu_datetime`

use crate::error::DateTimeError;
use core::convert::TryFrom;
use core::convert::TryInto;
use core::fmt;
use core::str::FromStr;
use tinystr::{TinyStr16, TinyStr4};
use zerovec::maps::ZeroMapKV;
use zerovec::ule::AsULE;

#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(clippy::exhaustive_structs)] // this is a newtype
pub struct Era(pub TinyStr16);

impl From<TinyStr16> for Era {
    fn from(x: TinyStr16) -> Self {
        Self(x)
    }
}

impl FromStr for Era {
    type Err = <TinyStr16 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

/// Representation of a formattable year.
///
/// More fields may be added in the future, for things like
/// the cyclic or extended year
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct FormattableYear {
    /// The era containing the year.
    pub era: Era,

    /// The year number in the current era (usually 1-based).
    pub number: i32,

    /// The related ISO year. This is normally the ISO (proleptic Gregorian) year having the greatest
    /// overlap with the calendar year. It is used in certain date formatting patterns.
    ///
    /// Can be None if the calendar does not typically use related_iso (and CLDR does not contain patterns
    /// using it)
    pub related_iso: Option<i32>,
}

impl FormattableYear {
    /// Construct a new Year given an era and number
    ///
    /// Other fields can be set mutably after construction
    /// as needed
    pub fn new(era: Era, number: i32) -> Self {
        Self {
            era,
            number,
            related_iso: None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::exhaustive_structs)] // this is a newtype
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_calendar::types),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct MonthCode(pub TinyStr4);

impl AsULE for MonthCode {
    type ULE = TinyStr4;
    fn to_unaligned(self) -> TinyStr4 {
        self.0
    }
    fn from_unaligned(u: TinyStr4) -> Self {
        Self(u)
    }
}

impl<'a> ZeroMapKV<'a> for MonthCode {
    type Container = zerovec::ZeroVec<'a, MonthCode>;
    type Slice = zerovec::ZeroSlice<MonthCode>;
    type GetType = <MonthCode as AsULE>::ULE;
    type OwnedType = MonthCode;
}

impl fmt::Display for MonthCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<TinyStr4> for MonthCode {
    fn from(x: TinyStr4) -> Self {
        Self(x)
    }
}
impl FromStr for MonthCode {
    type Err = <TinyStr4 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

/// Representation of a formattable month.
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct FormattableMonth {
    /// The month number in this given year. For calendars with leap months, all months after
    /// the leap month will end up with an incremented number.
    ///
    /// In general, prefer using the month code in generic code.
    pub ordinal: u32,

    /// The month code, used to distinguish months during leap years.
    pub code: MonthCode,
}

/// A struct containing various details about the position of the day within a year. It is returned
// by the [`day_of_year_info()`](trait.DateInput.html#tymethod.day_of_year_info) method of the
// [`DateInput`] trait.
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct DayOfYearInfo {
    /// The current day of the year, 1-based.
    pub day_of_year: u32,
    /// The number of days in a year.
    pub days_in_year: u32,
    /// The previous year.
    pub prev_year: FormattableYear,
    /// The number of days in the previous year.
    pub days_in_prev_year: u32,
    /// The next year.
    pub next_year: FormattableYear,
}

/// A day number in a month. Usually 1-based.
#[allow(clippy::exhaustive_structs)] // this is a newtype
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DayOfMonth(pub u32);

/// A week number in a month. Usually 1-based.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::exhaustive_structs)] // this is a newtype
pub struct WeekOfMonth(pub u32);

/// A week number in a year. Usually 1-based.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::exhaustive_structs)] // this is a newtype
pub struct WeekOfYear(pub u32);

/// A day of week in month. 1-based.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::exhaustive_structs)] // this is a newtype
pub struct DayOfWeekInMonth(pub u32);

impl From<DayOfMonth> for DayOfWeekInMonth {
    fn from(day_of_month: DayOfMonth) -> Self {
        DayOfWeekInMonth(1 + ((day_of_month.0 - 1) / 7))
    }
}

#[test]
fn test_day_of_week_in_month() {
    assert_eq!(DayOfWeekInMonth::from(DayOfMonth(1)).0, 1);
    assert_eq!(DayOfWeekInMonth::from(DayOfMonth(7)).0, 1);
    assert_eq!(DayOfWeekInMonth::from(DayOfMonth(8)).0, 2);
}

/// This macro defines a struct for 0-based date fields: hours, minutes, seconds
/// and fractional seconds. Each unit is bounded by a range. The traits implemented
/// here will return a Result on whether or not the unit is in range from the given
/// input.
macro_rules! dt_unit {
    ($name:ident, $storage:ident, $value:expr, $docs:expr) => {
        #[doc=$docs]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $name($storage);

        impl $name {
            /// Gets the numeric value for this component.
            pub const fn number(&self) -> $storage {
                self.0
            }
        }

        impl FromStr for $name {
            type Err = DateTimeError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                let val: $storage = input.parse()?;
                if val > $value {
                    Err(DateTimeError::Overflow {
                        field: "$name",
                        max: $value,
                    })
                } else {
                    Ok(Self(val))
                }
            }
        }

        impl TryFrom<$storage> for $name {
            type Error = DateTimeError;

            fn try_from(input: $storage) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(DateTimeError::Overflow {
                        field: "$name",
                        max: $value,
                    })
                } else {
                    Ok(Self(input))
                }
            }
        }

        impl TryFrom<usize> for $name {
            type Error = DateTimeError;

            fn try_from(input: usize) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(DateTimeError::Overflow {
                        field: "$name",
                        max: $value,
                    })
                } else {
                    Ok(Self(input as $storage))
                }
            }
        }

        impl From<$name> for $storage {
            fn from(input: $name) -> Self {
                input.0
            }
        }

        impl From<$name> for usize {
            fn from(input: $name) -> Self {
                input.0 as Self
            }
        }

        impl $name {
            /// Attempts to add two values.
            /// Returns `Some` if the sum is within bounds.
            /// Returns `None` if the sum is out of bounds.
            pub fn try_add(self, other: $storage) -> Option<Self> {
                let sum = self.0.saturating_add(other);
                if sum > $value {
                    None
                } else {
                    Some(Self(sum))
                }
            }

            /// Attempts to subtract two values.
            /// Returns `Some` if the difference is within bounds.
            /// Returns `None` if the difference is out of bounds.
            pub fn try_sub(self, other: $storage) -> Option<Self> {
                self.0.checked_sub(other).map(Self)
            }
        }
    };
}

dt_unit!(
    IsoHour,
    u8,
    24,
    "An ISO-8601 hour component, for use with ISO calendars.\n\nMust be within inclusive bounds `[0, 24]`."
);

dt_unit!(
    IsoMinute,
    u8,
    60,
    "An ISO-8601 minute component, for use with ISO calendars.\n\nMust be within inclusive bounds `[0, 60]`."
);

dt_unit!(
    IsoSecond,
    u8,
    61,
    "An ISO-8601 second component, for use with ISO calendars.\n\nMust be within inclusive bounds `[0, 61]`."
);

dt_unit!(
    NanoSecond,
    u32,
    999_999_999,
    "A fractional second component, stored as nanoseconds.\n\nMust be within inclusive bounds `[0, 999_999_999]`."
);

#[test]
fn test_iso_hour_arithmetic() {
    const HOUR_MAX: u8 = 24;
    const HOUR_VALUE: u8 = 5;
    let hour = IsoHour(HOUR_VALUE);

    // middle of bounds
    assert_eq!(
        hour.try_add(HOUR_VALUE - 1),
        Some(IsoHour(HOUR_VALUE + (HOUR_VALUE - 1)))
    );
    assert_eq!(
        hour.try_sub(HOUR_VALUE - 1),
        Some(IsoHour(HOUR_VALUE - (HOUR_VALUE - 1)))
    );

    // edge of bounds
    assert_eq!(hour.try_add(HOUR_MAX - HOUR_VALUE), Some(IsoHour(HOUR_MAX)));
    assert_eq!(hour.try_sub(HOUR_VALUE), Some(IsoHour(0)));

    // out of bounds
    assert_eq!(hour.try_add(1 + HOUR_MAX - HOUR_VALUE), None);
    assert_eq!(hour.try_sub(1 + HOUR_VALUE), None);
}

#[test]
fn test_iso_minute_arithmetic() {
    const MINUTE_MAX: u8 = 60;
    const MINUTE_VALUE: u8 = 5;
    let minute = IsoMinute(MINUTE_VALUE);

    // middle of bounds
    assert_eq!(
        minute.try_add(MINUTE_VALUE - 1),
        Some(IsoMinute(MINUTE_VALUE + (MINUTE_VALUE - 1)))
    );
    assert_eq!(
        minute.try_sub(MINUTE_VALUE - 1),
        Some(IsoMinute(MINUTE_VALUE - (MINUTE_VALUE - 1)))
    );

    // edge of bounds
    assert_eq!(
        minute.try_add(MINUTE_MAX - MINUTE_VALUE),
        Some(IsoMinute(MINUTE_MAX))
    );
    assert_eq!(minute.try_sub(MINUTE_VALUE), Some(IsoMinute(0)));

    // out of bounds
    assert_eq!(minute.try_add(1 + MINUTE_MAX - MINUTE_VALUE), None);
    assert_eq!(minute.try_sub(1 + MINUTE_VALUE), None);
}

#[test]
fn test_iso_second_arithmetic() {
    const SECOND_MAX: u8 = 61;
    const SECOND_VALUE: u8 = 5;
    let second = IsoSecond(SECOND_VALUE);

    // middle of bounds
    assert_eq!(
        second.try_add(SECOND_VALUE - 1),
        Some(IsoSecond(SECOND_VALUE + (SECOND_VALUE - 1)))
    );
    assert_eq!(
        second.try_sub(SECOND_VALUE - 1),
        Some(IsoSecond(SECOND_VALUE - (SECOND_VALUE - 1)))
    );

    // edge of bounds
    assert_eq!(
        second.try_add(SECOND_MAX - SECOND_VALUE),
        Some(IsoSecond(SECOND_MAX))
    );
    assert_eq!(second.try_sub(SECOND_VALUE), Some(IsoSecond(0)));

    // out of bounds
    assert_eq!(second.try_add(1 + SECOND_MAX - SECOND_VALUE), None);
    assert_eq!(second.try_sub(1 + SECOND_VALUE), None);
}

#[test]
fn test_iso_nano_second_arithmetic() {
    const NANO_SECOND_MAX: u32 = 999_999_999;
    const NANO_SECOND_VALUE: u32 = 5;
    let nano_second = NanoSecond(NANO_SECOND_VALUE);

    // middle of bounds
    assert_eq!(
        nano_second.try_add(NANO_SECOND_VALUE - 1),
        Some(NanoSecond(NANO_SECOND_VALUE + (NANO_SECOND_VALUE - 1)))
    );
    assert_eq!(
        nano_second.try_sub(NANO_SECOND_VALUE - 1),
        Some(NanoSecond(NANO_SECOND_VALUE - (NANO_SECOND_VALUE - 1)))
    );

    // edge of bounds
    assert_eq!(
        nano_second.try_add(NANO_SECOND_MAX - NANO_SECOND_VALUE),
        Some(NanoSecond(NANO_SECOND_MAX))
    );
    assert_eq!(nano_second.try_sub(NANO_SECOND_VALUE), Some(NanoSecond(0)));

    // out of bounds
    assert_eq!(
        nano_second.try_add(1 + NANO_SECOND_MAX - NANO_SECOND_VALUE),
        None
    );
    assert_eq!(nano_second.try_sub(1 + NANO_SECOND_VALUE), None);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Time {
    /// 0-based hour.
    pub hour: IsoHour,

    /// 0-based minute.
    pub minute: IsoMinute,

    /// 0-based second.
    pub second: IsoSecond,

    /// Fractional second
    pub nanosecond: NanoSecond,
}

impl Time {
    /// Do not validate the numeric input for this component.
    pub const fn new(
        hour: IsoHour,
        minute: IsoMinute,
        second: IsoSecond,
        nanosecond: NanoSecond,
    ) -> Self {
        Self {
            hour,
            minute,
            second,
            nanosecond,
        }
    }

    pub fn try_new(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Result<Self, DateTimeError> {
        Ok(Self {
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
            nanosecond: nanosecond.try_into()?,
        })
    }
}

/// A weekday in a 7-day week, according to ISO-8601.
///
/// The discriminant values correspond to ISO-8601 weekday numbers (Monday = 1, Sunday = 7).
///
/// # Examples
///
/// ```
/// use icu::datetime::input::IsoWeekday;
///
/// assert_eq!(1, IsoWeekday::Monday as usize);
/// assert_eq!(7, IsoWeekday::Sunday as usize);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)] // The weekday variants should be self-obvious.
#[repr(i8)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_calendar::types),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_enums)] // This is stable
pub enum IsoWeekday {
    Monday = 1,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<usize> for IsoWeekday {
    /// Convert from an ISO-8601 weekday number to an [`IsoWeekday`] enum. 0 is automatically converted
    /// to 7 (Sunday). If the number is out of range, it is interpreted modulo 7.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::input::IsoWeekday;
    ///
    /// assert_eq!(IsoWeekday::Sunday, IsoWeekday::from(0));
    /// assert_eq!(IsoWeekday::Monday, IsoWeekday::from(1));
    /// assert_eq!(IsoWeekday::Sunday, IsoWeekday::from(7));
    /// assert_eq!(IsoWeekday::Monday, IsoWeekday::from(8));
    /// ```
    fn from(input: usize) -> Self {
        let mut ordinal = (input % 7) as i8;
        if ordinal == 0 {
            ordinal = 7;
        }
        unsafe { core::mem::transmute(ordinal) }
    }
}
