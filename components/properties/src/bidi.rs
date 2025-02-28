// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module exposes tooling for running the [unicode bidi algorithm](https://unicode.org/reports/tr9/) using ICU4X data.
//!
//! `BidiClassAdapter` enables ICU4X to provide data to [`unicode-bidi`].
//!
//! # Examples
//!
//!```
//! use icu_collections::codepointtrie::CodePointTrie;
//! use icu_properties::bidi::BidiClassAdapter;
//! use icu_properties::{maps, BidiClass};
//! use unicode_bidi::BidiClass as DataSourceBidiClass;
//! use unicode_bidi::BidiDataSource;
//! use unicode_bidi::BidiInfo;
//! // This example text is defined using `concat!` because some browsers
//! // and text editors have trouble displaying bidi strings.
//! let text = concat!["א", "ב", "ג", "a", "b", "c",];
//!
//! // Create an adapter to provide the data to `BidiInfo`.
//! let provider = icu_testdata::get_provider();
//!
//! let data = maps::load_bidi_class(&provider).expect("The data should be valid");
//! let bc = data.as_borrowed();
//!
//! let adapter = BidiClassAdapter::new(bc);
//! // Resolve embedding levels within the text.  Pass `None` to detect the
//! // paragraph level automatically.
//!
//! let bidi_info = BidiInfo::new_with_data_source(&adapter, &text, None);
//!
//! // This paragraph has embedding level 1 because its first strong character is RTL.
//! assert_eq!(bidi_info.paragraphs.len(), 1);
//! let para = &bidi_info.paragraphs[0];
//! assert_eq!(para.level.number(), 1);
//! assert_eq!(para.level.is_rtl(), true);
//!
//! // Re-ordering is done after wrapping each paragraph into a sequence of
//! // lines. For this example, I'll just use a single line that spans the
//! // entire paragraph.
//! let line = para.range.clone();
//!
//! let display = bidi_info.reorder_line(para, line);
//! assert_eq!(display, concat!["a", "b", "c", "ג", "ב", "א",]);
//! ```

use crate::maps::CodePointMapDataBorrowed;
use crate::props::BidiClass;
use unicode_bidi::data_source::BidiDataSource;
use unicode_bidi::BidiClass as DataSourceBidiClass;

/// An adapter to convert from icu4x `BidiClass` to `unicode_bidi::BidiClass`.
///
/// # Example
///
/// ```
/// use icu_collections::codepointtrie::CodePointTrie;
/// use icu_properties::bidi::BidiClassAdapter;
/// use icu_properties::{maps, BidiClass};
/// use unicode_bidi::BidiClass as DataSourceBidiClass;
/// use unicode_bidi::BidiDataSource;
///
/// let provider = icu_testdata::get_provider();
///
/// let data = maps::load_bidi_class(&provider).expect("The data should be valid");
///
/// let adapter = BidiClassAdapter::new(data.as_borrowed());
/// assert_eq!(adapter.bidi_class('a'), DataSourceBidiClass::L);
/// assert_eq!(adapter.bidi_class('ع'), DataSourceBidiClass::AL);
/// ```
pub struct BidiClassAdapter<'a> {
    data: CodePointMapDataBorrowed<'a, BidiClass>,
}

impl<'a> BidiClassAdapter<'a> {
    /// Creates new instance of `BidiClassAdapter`.
    pub fn new(data: CodePointMapDataBorrowed<'a, BidiClass>) -> BidiClassAdapter<'a> {
        BidiClassAdapter { data }
    }
}

impl<'a> BidiDataSource for BidiClassAdapter<'a> {
    /// Returns a [`DataSourceBidiClass`] given a unicode character.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_collections::codepointtrie::CodePointTrie;
    /// use icu_properties::bidi::BidiClassAdapter;
    /// use icu_properties::{maps, BidiClass};
    /// use unicode_bidi::BidiClass as DataSourceBidiClass;
    /// use unicode_bidi::BidiDataSource;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let data = maps::load_bidi_class(&provider).expect("The data should be valid");
    ///
    /// let adapter = BidiClassAdapter::new(data.as_borrowed());
    /// assert_eq!(adapter.bidi_class('a'), DataSourceBidiClass::L);
    /// ```
    ///
    /// [`CodePointTrie`]: icu_collections::codepointtrie::CodePointTrie
    fn bidi_class(&self, c: char) -> DataSourceBidiClass {
        let bidi_class = self.data.get(c);
        match bidi_class {
            BidiClass::LeftToRight => DataSourceBidiClass::L,
            BidiClass::RightToLeft => DataSourceBidiClass::R,
            BidiClass::EuropeanNumber => DataSourceBidiClass::EN,
            BidiClass::EuropeanSeparator => DataSourceBidiClass::ES,
            BidiClass::EuropeanTerminator => DataSourceBidiClass::ET,
            BidiClass::ArabicNumber => DataSourceBidiClass::AN,
            BidiClass::CommonSeparator => DataSourceBidiClass::CS,
            BidiClass::ParagraphSeparator => DataSourceBidiClass::B,
            BidiClass::SegmentSeparator => DataSourceBidiClass::S,
            BidiClass::WhiteSpace => DataSourceBidiClass::WS,
            BidiClass::OtherNeutral => DataSourceBidiClass::ON,
            BidiClass::LeftToRightEmbedding => DataSourceBidiClass::LRE,
            BidiClass::LeftToRightOverride => DataSourceBidiClass::LRO,
            BidiClass::ArabicLetter => DataSourceBidiClass::AL,
            BidiClass::RightToLeftEmbedding => DataSourceBidiClass::RLE,
            BidiClass::RightToLeftOverride => DataSourceBidiClass::RLO,
            BidiClass::PopDirectionalFormat => DataSourceBidiClass::PDF,
            BidiClass::NonspacingMark => DataSourceBidiClass::NSM,
            BidiClass::BoundaryNeutral => DataSourceBidiClass::BN,
            BidiClass::FirstStrongIsolate => DataSourceBidiClass::FSI,
            BidiClass::LeftToRightIsolate => DataSourceBidiClass::LRI,
            BidiClass::RightToLeftIsolate => DataSourceBidiClass::RLI,
            BidiClass::PopDirectionalIsolate => DataSourceBidiClass::PDI,
            _ =>
            // This must not happen.
            {
                DataSourceBidiClass::ON
            }
        }
    }
}
