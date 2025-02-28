// @generated
type DataStruct =
    <::icu_properties::provider::EmojiComponentV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_slice_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    35u8, 0u8, 0u8, 0u8, 36u8, 0u8, 0u8, 0u8, 42u8, 0u8, 0u8, 0u8, 43u8, 0u8, 0u8,
                    0u8, 48u8, 0u8, 0u8, 0u8, 58u8, 0u8, 0u8, 0u8, 13u8, 32u8, 0u8, 0u8, 14u8,
                    32u8, 0u8, 0u8, 227u8, 32u8, 0u8, 0u8, 228u8, 32u8, 0u8, 0u8, 15u8, 254u8, 0u8,
                    0u8, 16u8, 254u8, 0u8, 0u8, 230u8, 241u8, 1u8, 0u8, 0u8, 242u8, 1u8, 0u8,
                    251u8, 243u8, 1u8, 0u8, 0u8, 244u8, 1u8, 0u8, 176u8, 249u8, 1u8, 0u8, 180u8,
                    249u8, 1u8, 0u8, 32u8, 0u8, 14u8, 0u8, 128u8, 0u8, 14u8, 0u8,
                ])
            },
            146usize,
        )
    });
