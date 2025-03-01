// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use core::convert::TryFrom;
    use diplomat_runtime::DiplomatResult;
    use icu_provider::DataProvider;
    use icu_segmenter::provider::GraphemeClusterBreakDataV1Marker;
    use icu_segmenter::{
        GraphemeClusterBreakIteratorLatin1, GraphemeClusterBreakIteratorUtf16,
        GraphemeClusterBreakIteratorUtf8, GraphemeClusterBreakSegmenter,
    };

    #[diplomat::opaque]
    /// An ICU4X grapheme-cluster-break segmenter, capable of finding grapheme cluster breakpoints
    /// in strings.
    #[diplomat::rust_link(icu_segmenter::GraphemeClusterBreakSegmenter, Struct)]
    pub struct ICU4XGraphemeClusterBreakSegmenter(GraphemeClusterBreakSegmenter);

    #[diplomat::opaque]
    pub struct ICU4XGraphemeClusterBreakIteratorUtf8<'a>(GraphemeClusterBreakIteratorUtf8<'a, 'a>);

    #[diplomat::opaque]
    pub struct ICU4XGraphemeClusterBreakIteratorUtf16<'a>(
        GraphemeClusterBreakIteratorUtf16<'a, 'a>,
    );

    #[diplomat::opaque]
    pub struct ICU4XGraphemeClusterBreakIteratorLatin1<'a>(
        GraphemeClusterBreakIteratorLatin1<'a, 'a>,
    );

    impl ICU4XGraphemeClusterBreakSegmenter {
        /// Construct an [`ICU4XGraphemeClusterBreakSegmenter`].
        #[diplomat::rust_link(icu_segmenter::GraphemeClusterBreakSegmenter::try_new, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XGraphemeClusterBreakSegmenter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            Self::try_new_impl(&provider)
        }

        fn try_new_impl<D>(
            provider: &D,
        ) -> DiplomatResult<Box<ICU4XGraphemeClusterBreakSegmenter>, ICU4XError>
        where
            D: DataProvider<GraphemeClusterBreakDataV1Marker> + ?Sized,
        {
            GraphemeClusterBreakSegmenter::try_new(provider)
                .map(|o| Box::new(ICU4XGraphemeClusterBreakSegmenter(o)))
                .map_err(Into::into)
                .into()
        }

        /// Segments a UTF-8 string.
        #[diplomat::rust_link(
            icu_segmenter::GraphemeClusterBreakSegmenter::segment_str,
            FnInStruct
        )]
        pub fn segment_utf8<'a>(
            &'a self,
            input: &'a str,
        ) -> Box<ICU4XGraphemeClusterBreakIteratorUtf8<'a>> {
            Box::new(ICU4XGraphemeClusterBreakIteratorUtf8(
                self.0.segment_str(input),
            ))
        }

        /// Segments a UTF-16 string.
        #[diplomat::rust_link(
            icu_segmenter::GraphemeClusterBreakSegmenter::segment_utf16,
            FnInStruct
        )]
        pub fn segment_utf16<'a>(
            &'a self,
            input: &'a [u16],
        ) -> Box<ICU4XGraphemeClusterBreakIteratorUtf16<'a>> {
            Box::new(ICU4XGraphemeClusterBreakIteratorUtf16(
                self.0.segment_utf16(input),
            ))
        }

        /// Segments a Latin-1 string.
        #[diplomat::rust_link(
            icu_segmenter::GraphemeClusterBreakSegmenter::segment_latin1,
            FnInStruct
        )]
        pub fn segment_latin1<'a>(
            &'a self,
            input: &'a [u8],
        ) -> Box<ICU4XGraphemeClusterBreakIteratorLatin1<'a>> {
            Box::new(ICU4XGraphemeClusterBreakIteratorLatin1(
                self.0.segment_latin1(input),
            ))
        }
    }

    impl<'a> ICU4XGraphemeClusterBreakIteratorUtf8<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[allow(clippy::should_implement_trait)]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }

    impl<'a> ICU4XGraphemeClusterBreakIteratorUtf16<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[allow(clippy::should_implement_trait)]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }

    impl<'a> ICU4XGraphemeClusterBreakIteratorLatin1<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[allow(clippy::should_implement_trait)]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }
}
