// @generated
/// Implement [`DataProvider<EastAsianWidthV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_ea_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_EA_V1: &'static <icu_properties::provider::EastAsianWidthV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointMapV1::CodePointTrie(icu_collections::codepointtrie::CodePointTrie::from_parts(icu_collections::codepointtrie::CodePointTrieHeader { high_start: 1114112u32, shifted12_high_start: 272u16, index3_null_offset: 18u16, data_null_offset: 359u32, null_value: 0u32, trie_type: icu_collections::codepointtrie::TrieType::Small }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0@\0\x7F\0\xBF\0\xFC\0;\x01g\x01\x99\x01g\x01\xC8\x01g\x01\x04\x02D\x02T\x02\x84\x02\xC2\x02\x01\x031\x03g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01\x98\x04\xB2\x04\xC0\x04\xD6\x04\xF6\x04\xFF\x04\x1C\x056\x05V\x05V\x05V\x05W\x05V\x05V\x05V\x05W\x05\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04\xF6\x04w\x05\xF6\x04\xF6\x04\xF6\x04\x97\x05\x97\x05\x97\x05\x98\x05\x97\x05\x97\x05\x97\x05\x98\x05\0\0\x10\0 \x000\0@\0P\0`\0p\0\x7F\0\x8F\0\x9F\0\xAF\0\xBF\0\xCF\0\xDF\0\xEF\0\xFC\0\x0C\x01\x1C\x01,\x01;\x01K\x01[\x01k\x01g\x01w\x01\x87\x01\x97\x01\x99\x01\xA9\x01\xB9\x01\xC9\x01g\x01w\x01\x87\x01\x97\x01\xC8\x01\xD8\x01\xE8\x01\xF8\x01g\x01w\x01\x87\x01\x97\x01\x04\x02\x14\x02$\x024\x02D\x02T\x02d\x02t\x02T\x02d\x02t\x02\x84\x02\x84\x02\x94\x02\xA4\x02\xB4\x02\xC2\x02\xD2\x02\xE2\x02\xF2\x02\x01\x03\x11\x03!\x031\x031\x03A\x03Q\x03a\x03g\x01w\x01\x87\x01\x97\x01g\x01w\x01\x87\x01\x97\x01g\x01w\x01\x87\x01\x97\x01g\x01w\x01\x87\x01\x97\x01g\x01w\x01\x87\x01\x97\x01g\x01w\x01\x87\x01\x97\x01g\x01w\x01\x87\x01\x97\x01g\x01w\x01\x87\x01\x97\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01q\x03q\x03q\x03q\x03q\x03q\x03g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01\x81\x03\x91\x03\xA1\x03g\x01g\x01g\x01\x13\x01\xB0\x03g\x01\xB7\x03g\x01g\x01g\x01g\x01g\x01\xC4\x03\xD1\x03\xE0\x03g\x01g\x01\xED\x03x\x02z\x02\xC6\0z\x02g\x01\xFC\x03g\x01\n\x04\x10\x01g\x01\x1A\x04$\x043\x04B\x04P\x04d\x01`\x04g\x01h\x04s\x04\x12\x01\x91\0g\x01g\x01g\x01g\x01g\x01z\x04\x86\x04g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01\x91\x04\xA1\x04g\x01g\x01g\x01g\x01g\x01g\x01D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02\x9C\x02D\x02D\x02D\x02D\x02D\x02x\x02D\x02D\x02\x80\x02D\x02\xAF\x04\xA4\x02\xBD\x04\xC9\x04M\x01\xD5\x04\xE5\x04\xF4\x04\x04\x05g\x01g\x01\x10\x05\x1C\x05,\x05b\x03g\x01<\x05L\x05X\x05a\x05k\x05y\x05\x87\x05\x97\x05g\x01\xA3\x05\xCC\x01\xAC\x05\xBB\x05g\x01>\x02g\x01\xC6\x05g\x01\xCD\x05g\x01g\x01\xDD\x05g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01\xEB\x05g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01\xE7\x04g\x01g\x01g\x01\xFB\x05g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01q\x03\x0B\x06q\x03q\x03q\x03q\x03q\x03\x1C\x05q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03\x1A\x05g\x01\x16\x06&\x06q\x03q\x03'\x06p\x03q\x03q\x03q\x03q\x03/\x06q\x03q\x03q\x03q\x03q\x03q\x03l\x03q\x03q\x03p\x03q\x03q\x03q\x03q\x03'\x06q\x03q\x03q\x03q\x03q\x03\x1C\x05q\x03'\x06q\x03q\x038\x06q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03g\x01g\x01g\x01g\x01q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03H\x06q\x03q\x03q\x03\x19\x05g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01q\x03H\x06g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03\x1C\x05g\x01g\x01g\x01g\x01g\x01D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01D\x02K\x06g\x01q\x03q\x03\x12\x06[\x06g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01j\x06k\x06k\x06k\x06k\x06k\x06z\x06{\x06{\x06{\x06{\x06|\x06\x8B\x06\x93\x06\xA3\x06\xCC\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01\x1B\x05\xCC\x05q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03\x18\x05q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03\x1A\x05g\x01g\x01\xB3\x06g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01\xC3\x06q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03\xCB\x05\xD2\x06g\x01\xE2\x06\xEE\x06q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03\x16\x06g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01\xA7\x05g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01b\x03g\x01g\x01g\x01y\x02D\x02v\x02D\x02D\x02D\x02z\x02D\x02\xFE\x06\r\x07w\x02g\x01g\x01g\x01g\x01g\x01\xCB\x05q\x03q\x03\x16\x06\xB3\x06\xCC\x05\x1A\x05g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01q\x03q\x03\x1D\x07\x0F\x06q\x03q\x03q\x03*\x07q\x03\x1C\x05q\x03q\x038\x07\x1C\x05q\x03G\x07q\x03q\x03q\x03'\x06V\x07q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03)\x06q\x03q\x03q\x03(\x06f\x07q\x03\x18\x05\xD2\x05g\x01\x7F\x04\xA7\x05g\x01g\x01g\x01g\x01f\x03q\x03q\x03q\x03q\x03q\x03g\x01g\x01g\x01q\x03q\x03q\x03q\x03v\x07\x86\x07\xE7\x04\x8E\x07g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01\x16\x06\x9A\x07g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01e\x03q\x03q\x03,\x07\x0F\x06q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03g\x01g\x01g\x01g\x01g\x01g\x01g\x01H\x06\xB3\x06q\x03q\x03\xAA\x07\xB9\x07\x16\x06\xB3\x06\xB3\x06g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03q\x03(\x06g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01g\x01D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02g\x01D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02D\x02v\x02\x84\0\xA4\0\xC4\0\xCC\0\xCC\0\xCC\0\xCC\0\xCC\0\xEC\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x0B\x01+\x01K\x01k\x01\x8A\x01\xA3\x01\x12\0\xBB\x01\xDB\x01\xFA\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\x03\x02\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01#\x02\x12\x000\x02\x12\0\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01P\x02\x12\0\x12\0\x12\0\x12\0p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02\x80\x02\x90\x02\x12\0\xB0\x02\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\xD0\x02\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xF0\x02\xFF\x01\xFF\x01\x10\x03\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0!\x03A\x03X\x03\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0x\x03\x98\x03\xB8\x03\xD8\x03\xF8\x03\x18\x04\x12\0\x12\0\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x01\xFF\x018\x04X\x04\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0\x12\0p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02p\x02x\x04") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x04\x04\x01\x04\x04\x01\x01\0\x01\0\x04\x01\x01\x04\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\x01\x01\0\0\0\0\0\x01\x01\x01\x01\0\0\0\0\x01\0\x01\x01\x01\0\x01\x01\0\0\x01\0\x01\x01\0\0\0\x01\x01\x01\x01\0\x01\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\x01\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\x01\x01\0\0\0\x01\0\0\0\0\0\x01\x01\x01\0\0\0\0\x01\0\0\0\0\0\0\x01\x01\x01\0\x01\0\0\0\x01\x01\x01\x01\0\x01\0\0\0\0\x01\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\x01\0\x01\0\x01\0\x01\0\x01\0\x01\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\x01\0\x01\x01\x01\0\x01\0\0\x01\0\0\0\0\0\0\0\x01\x01\x01\x01\0\x01\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x01\0\0\x01\x01\x01\x01\0\x01\x01\0\0\x01\x01\0\0\x01\x01\x01\0\x01\x01\x01\x01\0\0\0\0\0\0\0\0\x01\0\x01\x01\0\x01\0\0\0\0\0\x01\0\0\x01\0\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\0\x02\0\0\x01\0\0\0\x01\0\x01\0\0\0\x01\0\0\0\0\0\0\x01\0\0\x01\0\0\0\0\0\0\0\0\0\x01\x01\0\0\0\x01\0\0\0\0\x01\0\0\0\0\x01\x01\0\0\0\0\0\0\x01\x01\x01\x01\0\0\0\0\0\0\0\0\x01\x01\0\0\0\0\0\0\x01\0\x01\0\0\0\0\0\0\0\0\0\0\0\x01\0\x01\x01\0\0\0\x01\x01\0\0\x01\0\0\0\x01\0\0\0\0\x01\0\0\x01\x01\x01\0\0\x01\0\x01\0\x01\x01\x01\x01\x01\x01\0\x01\0\0\0\0\x01\x01\x01\x01\0\0\0\0\x01\x01\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x01\x01\0\0\x01\x01\x01\x01\0\0\x01\x01\0\0\x01\x01\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\x05\x05\0\0\0\0\0\0\0\0\0\x05\x05\0\0\0\0\0\0\0\0\0\x05\x05\x05\x05\0\0\0\x05\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\x01\x01\0\0\x01\x01\0\0\0\0\x01\x01\0\0\0\0\x01\x01\x01\0\0\x01\0\0\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\x05\0\0\0\0\0\x01\x01\0\0\x01\0\0\0\0\x01\x01\0\0\0\0\x05\x05\0\0\0\0\0\0\x01\0\x01\0\0\0\0\0\x05\x05\x05\x05\x05\x05\x05\x05\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\0\x01\x01\x01\0\x01\x01\x01\x01\0\x01\x01\0\x01\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\x01\x01\0\x05\0\0\0\0\0\0\0\0\x05\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\x05\x01\x01\x01\x01\x01\x01\x01\x01\x05\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\0\0\0\0\x01\x01\x05\x01\x01\x01\x01\x01\x05\x05\x01\x05\x01\x01\x01\x01\x05\x01\x01\x05\x01\x01\0\0\0\0\0\x05\0\0\0\0\x05\x05\0\0\0\0\0\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\x05\0\x05\0\0\0\x05\x05\x05\0\x05\0\0\0\0\0\0\0\0\x05\x05\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\0\0\0\0\0\0\x04\x04\x04\x04\x04\x04\x04\x04\0\0\0\0\0\x04\x04\0\0\0\0\0\0\0\0\0\x05\0\0\0\0\x05\x01\x01\x01\x01\0\0\0\0\0\0\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\0\0\0\x03\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\0\x05\x05\x05\x05\x05\x05\x05\x05\x01\x01\x01\x01\x01\x01\x01\x01\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\0\0\0\0\0\x05\x05\x05\x05\x05\x05\x05\0\x05\x05\x05\x05\0\0\0\0\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\0\0\x02\x02\x02\x02\x02\x02\0\0\x02\x02\x02\x02\x02\x02\0\0\x02\x02\x02\0\0\0\x03\x03\x03\x03\x03\x03\x03\0\x02\x02\x02\x02\x02\x02\x02\0\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\0\0\0\0\0\0\x05\x05\x05\x05\0\x05\x05\x05\x05\x05\x05\x05\0\x05\x05\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\x05\x05\0\0\x05\0\0\0\0\0\0\0\0\0\0\x05\x05\x05\x05\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x05\x01\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x01\x01\x01\x01\x01\x05\0\0\0\0\0\0\0\0\0\0\0\0\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\0\0\0\x05\0\0\0\x05\0\0\0\x05\x05\x05\x05\x05\x05\x05\x05\0\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\0\0\0\0\0\0\0\0\0\0\x05\x05\x05\x05\0\x05\x05\x05\x05\x05\x05\0\0\0\0\0\0\x05\0\0\0\x05\x05\x05\0\0\x05\x05\x05\0\0\0\0\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\0\x05\x05\x05\x05\x05\x05\0\0\0\0\0\0\0\0\x05\x05\0\0\0") }, icu_properties::EastAsianWidth(0u8)));
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::EastAsianWidthV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::EastAsianWidthV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_EA_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::EastAsianWidthV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
