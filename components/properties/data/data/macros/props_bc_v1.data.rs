// @generated
/// Implement [`DataProvider<BidiClassV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_bc_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_BC_V1: &'static <icu_properties::provider::BidiClassV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointMapV1::CodePointTrie(icu_collections::codepointtrie::CodePointTrie::from_parts(icu_collections::codepointtrie::CodePointTrieHeader { high_start: 1114112u32, shifted12_high_start: 272u16, index3_null_offset: 897u16, data_null_offset: 247u32, null_value: 0u32, trie_type: icu_collections::codepointtrie::TrieType::Small }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0@\0\x7F\0\xBF\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xFE\0<\x01|\x01\x8C\x01\xCB\x01\xD5\x01\xF7\0\xF7\0\x12\x02\xF7\0\xF7\0\xF7\0H\x02\x86\x02\xC6\x02\xFB\x02,\x03V\x03\x90\x03\xC5\x03\xDF\x03\x1F\x04]\x04\x8B\x04\xBB\x04\xF1\x04.\x05m\x05\xAC\x05\xEB\x05*\x06i\x06*\x06\xA8\x06\xE8\x06&\x07d\x07\xA4\x07\xE4\x07#\x08\xAC\x05b\x08\x84\x08\xC3\x08\x02\t8\tO\t\x8F\t\x9E\t\r\x02\xDB\t\x19\nS\n\xA7\x05\xA1\x08\xBB\x08\xC9\x08\xDF\x08\xFF\x08\x1A\t2\tQ\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\t\x92\tq\tq\tr\tq\tq\tq\tr\tq\tq\tq\tr\t\0\0\x10\0 \x000\0@\0P\0`\0p\0\x7F\0\x8F\0\x9F\0\xAF\0\xBF\0\xCF\0\xDF\0\xEF\0\xF7\0\x07\x01\x17\x01'\x01\xF7\0\x07\x01\x17\x01'\x01\xF7\0\x07\x01\x17\x01'\x01\xF7\0\x07\x01\x17\x01'\x01\xFE\0\x0E\x01\x1E\x01.\x01<\x01L\x01\\\x01l\x01|\x01\x8C\x01\x9C\x01\xAC\x01\x8C\x01\x9C\x01\xAC\x01\xBC\x01\xCB\x01\xDB\x01\xEB\x01\xFB\x01\xD5\x01\xE5\x01\xF5\x01\x05\x02\xF7\0\x07\x01\x17\x01'\x01\xF7\0\x07\x01\x17\x01'\x01\x12\x02\"\x022\x02B\x02\xF7\0\x07\x01\x17\x01'\x01\xF7\0\x07\x01\x17\x01'\x01\xF7\0\x07\x01\x17\x01'\x01H\x02X\x02h\x02x\x02\x86\x02\x96\x02\xA6\x02\xB6\x02\xC6\x02\xD6\x02\xE6\x02\xF6\x02\xFB\x02\x0B\x03\x1B\x03+\x03,\x03<\x03L\x03\\\x03V\x03f\x03v\x03\x86\x03\x90\x03\xA0\x03\xB0\x03\xC0\x03\xC5\x03\xD5\x03\xE5\x03\xF5\x03\xDF\x03\xEF\x03\xFF\x03\x0F\x04\x1F\x04/\x04?\x04O\x04]\x04m\x04}\x04\x8D\x04\x8B\x04\x9B\x04\xAB\x04\xBB\x04\xBB\x04\xCB\x04\xDB\x04\xEB\x04\xF1\x04\x01\x05\x11\x05!\x05.\x05>\x05N\x05^\x05m\x05}\x05\x8D\x05\x9D\x05\xAC\x05\xBC\x05\xCC\x05\xDC\x05\xEB\x05\xFB\x05\x0B\x06\x1B\x06*\x06:\x06J\x06Z\x06i\x06y\x06\x89\x06\x99\x06*\x06:\x06J\x06Z\x06\xA8\x06\xB8\x06\xC8\x06\xD8\x06\xE8\x06\xF8\x06\x08\x07\x18\x07&\x076\x07F\x07V\x07d\x07t\x07\x84\x07\x94\x07\xA4\x07\xB4\x07\xC4\x07\xD4\x07\xE4\x07\xF4\x07\x04\x08\x14\x08#\x083\x08C\x08S\x08\xAC\x05\xBC\x05\xCC\x05\xDC\x05b\x08r\x08\x82\x08\x92\x08\x84\x08\x94\x08\xA4\x08\xB4\x08\xC3\x08\xD3\x08\xE3\x08\xF3\x08\x02\t\x12\t\"\t2\t8\tH\tX\th\tO\t_\to\t\x7F\t\x8F\t\x9F\t\xAF\t\xBF\t\x9E\t\xAE\t\xBE\t\xCE\t\r\x02\x1D\x02-\x02=\x02\xDB\t\xEB\t\xFB\t\x0B\n\x19\n)\n9\nI\nS\nc\ns\n\x83\n\xA7\x05\xB7\x05\xC7\x05\xD7\x05\xF7\0\xF7\0=\n\x93\n\xF7\0\xA2\n\x1B\x02\xAF\n\xBD\n\xA0\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0=\n\xF7\0\xF7\0\xF7\0\xCD\n\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0@\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xDD\n,\x01\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xEB\n\xF7\0\x8D\x05\xF7\0\x8D\x05\xF7\0\x8D\x05\xF7\0\xF7\0\xF7\0\xF7\nz\t\x01\x0B\xF7\0\xCD\n\x11\x0B\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x8A\x05\xF7\0\xA4\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0 \x0B.\x0B>\x0B\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0M\0k\x01k\x01\xF7\0G\x0B\xF7\0\xF7\0\xF7\0S\x0Ba\x0Bn\x0B\xF7\0\xF7\0\xF7\0|\x01\xAD\x01\xF7\0\xF7\0\xF7\0\x18\x02\xF7\0\xF7\0~\x0B\xAB\x05\xF7\0?\n\x18\x02\x1A\x02\xF7\0\x8C\x0B\xF7\0\xF7\0\xF7\0\x9A\x0B\x1A\x02\xF7\0\xF7\0>\n\xA9\x0B\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0h\n\xB9\x0B\xC2\x0B\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0|\x01|\x01|\x01|\x01\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xCC\x0B\xDB\x0BN\0N\0*\x01\xEB\x0Bk\x01\xFB\x0B\x0B\x0C\x17\x0C\x1C\x0C,\x0C<\x0CL\x0C\xF7\0\\\x0C\\\x0C\\\x0C|\x01|\x01\x1B\x02l\x0Cx\x0C\x86\x0C-\x01\x96\x0Ck\x01\xF7\0\xF7\0\xA4\x0Ck\x01k\x01k\x01k\x01k\x01k\x01k\x01\xB4\x0Ck\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01;\0\xF7\0\xF7\0\xF7\0P\0k\x01e\x01k\x01k\x01k\x01k\x01k\x01k\x01[\x08\xF7\0Q\x01\xF7\0k\x01k\x01\xBC\x0C\xC4\x0C\xF7\0\xF7\0\xF7\0\xF7\0Q\0k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01\xD4\x0Ck\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01H\x01k\x01d\x01k\x01k\x01k\x01k\x01k\x01k\x01\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xE4\x0C\xF3\x0C\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0Y\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0|\x01|\x01k\x01k\x01k\x01k\x01k\x01>\x01\xF7\0\xF7\0k\x01\xFC\x0Ck\x01k\x01k\x01k\x01k\x01=\0k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01;\0\xF7\0P\x01\x0C\rk\x01\x1B\r+\r\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0;\r@\0\xF7\0\xF7\0\xF7\0\xF7\0\xCB\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0k\x01k\x01=\0\xF7\0*\x01\xF7\0\xF7\0\xF7\0k\x01\xF7\0H\r\xF7\0\xF7\0\xF7\0j\x01O\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0W\r\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0M\0\xF7\0L\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0k\x01k\x01k\x01k\x01\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0k\x01k\x01k\x01[\x08\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0N\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0Y\x05g\r\xF7\0\xFF\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x1A\x02k\x01k\x01?\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xCE\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0w\r\xF7\0\x83\r\x15\x06\xF7\0\xF7\0\xF7\0\x9C\x0C\xF7\0\xF7\0\xF7\0\xF7\0\x8B\x05\xF7\0|\x01\x93\r\xF7\0\xF7\0\x90\t\xF7\0C\n\x1A\x02\xF7\0\xF7\0\x19\x02\xF7\0\xF7\0\x9F\r\xF7\0\xF7\0\xA8\x05\xF7\0\xF7\0\xAD\r\xBC\r\xC9\r\xF7\0\xF7\0\xA1\x05\xF7\0\xF7\0\xF7\0\xD9\r\xAC\x05\xF7\0\x01\x06\xA7\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0-\x01\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xE9\r\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\r\x06\x0E\x8E\x02\x8E\x02,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03\x16\x0Ek\x01,\x03,\x03,\x03,\x03,\x03,\x03,\x03&\x0E\x85\0\x85\0(\x0E|\x01\xCD\n|\x01k\x01k\x018\x0EH\x0E,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03X\x0Eh\x0E0\0@\0P\0@\0P\0;\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0x\x0E\x88\x0E\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xD5\0\xF7\0\xF7\0\xF7\0k\x01k\x01k\x01k\x01O\x01O\x01@\0\xF7\0\xF7\0\xF7\0\xF7\0\xA0\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x98\x0E\xC4\x0C\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xA8\x0E\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\xB8\x0E\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\xC8\x0E\x8E\x02\x8E\x02\xD8\x0E\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\xE8\x0E\x8E\x02\x8E\x02\x8E\x02\x8E\x02\xEF\x0E\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02,\x03,\x03\xFF\x0E\x0F\x0F\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x1F\x0F \x0F\x8E\x02\x8E\x02/\x0F\x8E\x02,\x03,\x03,\x03\xF9\x02\x8E\x02\x8E\x02\x8E\x02,\x03\0\x03\xE0\x02,\x03\x8E\x02=\x0F\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\xAC\x05\xF7\0\xF7\0B\n\x15\x02<\x01;\0M\x0F\x1A\x02\xF7\0\xF7\0Y\x0F\xAB\x05\xF7\0\xF7\0\xF7\0\x19\x02\xF7\0d\x0F\x17\x02\xF7\0\xF7\0\xF7\0\xAA\x05\x1A\x02\xF7\0\xF7\0t\x0FS\x0F\xF7\0\xF7\0\xF7\0Y\x05\x81\x0F\xAC\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0Y\x05\x93\t\xF7\0\x1A\x02\xF7\0\xF7\0\x02\x06\x1B\x02\xF7\0\x0F\x02\x17\x02\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0B\nH\tZ\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x90\x0F)\x06\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x9D\x0F\x1B\x02\x01\x06\xF7\0\xF7\0\xF7\0\xAD\x0F\x1B\x02\xF7\0O\x01\xF7\0\xF7\0\xF7\0]\x05\xE2\x06\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0=\n\xBD\x0F\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0Y\x05P\n\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xC9\x0F\xAA\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0X\x0F\x1B\x02\xF7\0\xD8\x0F\xF7\0\xF7\0\xE5\x0F\xA6\x05\xF4\x0F\xF7\0\xF7\0@\n\x04\x10\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x14\x10\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0H\n$\x103\x10\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0B\x10\xE2\x06\xF7\0\xF7\0\xF7\0\xF7\0Q\x10\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x8C\x05\x1A\x02\xF7\0\xF7\0\xA8\x0E\xE7\x06\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\\\x10k\x10?\0\xF7\0\xF7\0\xF7\0\xF7\0s\x0F\x16\x02\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x17\x02\xF7\0\xF7\0\xF7\0\x15\x02\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0Y\x05\xF7\0\xF7\0\xF7\0Y\x05\x19\x02\xF7\0\xF7\0\xF7\0\xF7\0{\x10\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\0\x06\x8B\x10\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0|\x01|\x01\xAE\x01|\x01\x15\x02\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xE6\n\x98\x10\xA5\x10\xF7\0R\x0F\xF7\0\xF7\0\xF7\0.\x01\xF7\0k\x01k\x01k\x01k\x01\xB5\x10\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0k\x01k\x01k\x01k\x01k\x01[\x08\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xCB\0\xF7\0\xF7\0\xF7\0\xD1\0\xF7\0\xF7\0L\0\xF7\0\xF7\0\xF7\0\xCD\0\xF7\0\xF7\0\xF7\0\xC2\x10\xD0\x10\xD0\x10\xD0\x10|\x01|\x01|\x01\xE0\x10|\x01|\x01\xAF\x01\xA8\x05\xA9\x05?\nk\n\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0d\n\xEB\x10\xF9\x10\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0Y\x05\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x15\x02\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0Z\x05\xF7\0\xF7\0\xF7\0>\n\x04\x11\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0>\n\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x14\x11\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02 \x11\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02,\x03,\x03,\x03,\x03,\x03\x8E\x02\x8E\x02\x8E\x02\x8E\x02,\x03,\x03,\x03,\x03,\x03\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03,\x03$\x0E\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02\x8E\x02k\x01k\x01P\x01k\x01k\x01k\x01k\x01k\x01k\x01=\0l\x10j\x01j\x01j\x01k\x01;\x000\x11\xF7\0L\0\xF7\0\xF7\0\xF7\0Q\0\xF7\0\xF7\0\xF7\0\xC9\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0;\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01;\x11O\x01O\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01<\x11k\x01k\x01k\x01k\x01k\x01\xCD\nP\x01@\0P\x01k\x01k\x01k\x01\x14\r\xCD\nk\x01k\x01\x14\rk\x01>\x01?\0\xF7\0\xF7\0\xF7\0\xF7\0k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01=\0>\x01O\x01G\x11k\x01k\x01W\x11f\x11P\x01G\x11G\x11k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01k\x01g\x01k\x01k\x01Q\x01\xF7\0\xF7\0\x9E\x0E\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0\xF7\0v\x11\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0|\x01|\x01|\x01|\x01|\x01|\x01|\x01|\x01|\x01|\x01|\x01|\x01|\x01|\x01|\x01\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x85\0\x84\0\x9C\0\xBC\0\xDC\0\xFC\0\x1C\x01<\x01\\\x01|\x01\x87\x01\xA7\x01\xBF\x01\xDF\x01\xFF\x01\x1F\x02?\x02_\x02~\x02\x9C\x02\xB2\x02\xD2\x02\xE2\x02\x02\x03\"\x03B\x03a\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x85\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\xA5\x03\xC5\x03\xE5\x03\x04\x04\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03#\x048\x04X\x04x\x04\x98\x04\x81\x03\x81\x03\xB8\x04\xD8\x04\xEC\x04\x06\x05&\x05D\x05a\x05\x7F\x05\x9D\x05\xBD\x05\xDA\x05\xF4\x05\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x14\x06\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03%\x06\x81\x039\x06\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03X\x06\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03h\x06}\x06\x9D\x06\x81\x03\xB3\x06\x81\x03\xD3\x06\x81\x03\x81\x03\xF3\x06\t\x07\x1B\x07\x81\x03;\x07P\x07i\x07\x89\x07\xA9\x07\xC4\x07\xD4\x07\xE7\x07\x07\x08\"\x08\x81\x03B\x08\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03B\x08b\x08\x81\x08\x81\x08\x81\x08\x81\x08\x81\x08\x81\x08\x81\x08\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03\x81\x03") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x12\x12\x12\x12\x12\x12\x12\x12\x12\x08\x07\x08\t\x07\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x07\x07\x07\x08\t\n\n\x04\x04\x04\n\n\n\n\n\x03\x06\x03\x06\x06\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x06\n\n\n\n\n\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\n\n\n\n\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\n\n\n\x12\x12\x12\x12\x12\x07\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x12\x06\n\x04\x04\x04\x04\n\n\n\n\0\n\n\x12\n\n\x04\x04\x02\x02\n\0\n\n\n\x02\0\n\n\n\n\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\n\0\0\0\0\0\n\n\n\n\n\n\n\n\n\n\n\n\n\n\0\0\n\n\n\n\n\n\n\n\n\n\n\n\n\n\0\0\0\0\0\n\n\n\n\n\n\n\n\n\0\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\n\n\0\0\0\0\0\0\0\0\n\0\0\0\0\n\n\0\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\0\0\n\n\x04\x01\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x01\x11\x11\x01\x11\x11\x01\x11\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x05\x05\x05\x05\x05\x05\n\n\r\x04\x04\r\x06\r\n\n\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x04\x05\x05\r\r\r\x11\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\x11\x11\x11\x11\x11\x11\x11\x05\n\x11\x11\x11\x11\x11\x11\r\r\x11\x11\n\x11\x11\x11\x11\r\r\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\x11\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x11\x11\x11\x11\x11\x11\x11\x11\x11\x01\x01\n\n\n\n\x01\x01\x01\x11\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x11\x11\x11\x11\x01\x11\x11\x11\x11\x11\x11\x11\x11\x11\x01\x11\x11\x11\x01\x11\x11\x11\x11\x11\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x11\x11\x11\x01\x01\x01\x01\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\x05\x05\r\r\r\r\r\r\x11\x11\x11\x11\x11\x11\x11\x11\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x05\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\x11\0\0\0\x11\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\x11\0\0\0\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\0\x11\x11\x11\x11\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x04\x04\0\0\0\0\0\0\0\x04\0\0\x11\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\0\x11\x11\0\0\0\0\x11\x11\0\0\x11\x11\x11\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\x11\x11\x11\x11\x11\0\x11\x11\0\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\x11\x11\x11\x11\x11\x11\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\x11\x11\x11\x11\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\n\n\n\n\n\x04\n\0\0\0\0\0\x11\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\x11\x11\0\0\0\0\0\x11\x11\x11\0\x11\x11\x11\x11\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\n\n\n\n\n\n\n\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\x11\x11\x11\x11\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\x11\x11\x11\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\x04\0\0\0\0\0\0\0\x11\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\x11\x11\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\x11\0\x11\n\n\n\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\0\x11\x11\0\0\0\0\0\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\0\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\0\0\0\x11\0\x11\x11\x11\x11\x11\x11\0\x11\x11\0\0\x11\x11\0\0\0\0\0\0\0\0\x11\x11\0\0\0\0\x11\x11\x11\x11\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\x11\x11\0\0\0\0\0\0\x11\0\0\n\n\n\n\n\n\n\n\n\n\0\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\x11\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\0\0\0\x04\0\x11\0\0\n\n\n\n\n\n\n\n\n\n\n\x11\x11\x11\x12\x11\x11\x11\0\0\0\0\x11\x11\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\x11\x11\x11\0\0\0\0\n\0\0\0\n\n\0\0\0\0\0\0\0\0\0\0\x11\x11\0\0\x11\0\0\0\0\0\0\x11\0\x11\x11\x11\x11\x11\x11\x11\0\x11\0\0\x11\x11\x11\x11\x11\x11\x11\x11\0\0\0\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\0\0\x11\0\0\0\0\x11\0\x11\x11\x11\x11\x11\0\x11\0\0\0\x11\x11\x11\x11\0\0\x11\x11\0\x11\x11\x11\0\0\0\0\0\0\x11\0\x11\x11\0\0\0\x11\0\x11\x11\x11\x11\0\0\x11\x11\0\0\0\0\0\0\0\0\x11\0\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\x11\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\n\0\n\n\0\0\0\0\0\0\0\0\0\0\0\n\n\n\t\t\t\t\t\t\t\t\t\t\t\x12\x12\x12\0\x01\n\n\n\n\n\n\n\n\t\x07\x0B\x0E\x10\x0C\x0F\x06\x04\x04\x04\x04\x04\n\n\n\n\n\n\n\n\n\n\n\x06\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\t\x12\x12\x12\x12\x12\x12\x14\x15\x13\x16\x12\x12\x12\x12\x12\x12\x02\0\0\0\x02\x02\x02\x02\x02\x02\x03\x03\n\n\n\0\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x03\x03\n\n\n\0\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\x04\n\n\0\n\n\n\n\0\n\n\0\0\0\0\0\0\n\0\n\n\n\0\0\0\0\0\n\n\n\n\0\n\0\n\0\n\0\0\0\0\x04\0\n\n\n\n\n\0\0\0\0\0\n\n\n\n\0\0\0\0\0\0\0\0\0\n\n\n\0\0\0\0\n\n\x03\x04\n\n\n\n\n\n\n\n\n\n\n\n\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\0\0\0\0\n\n\n\n\n\n\n\n\n\n\n\n\0\n\n\n\0\0\0\0\0\n\n\n\n\n\n\0\0\0\0\x11\x11\0\0\0\0\0\0\0\n\n\n\n\n\n\n\n\n\n\0\n\n\n\n\n\t\n\n\n\n\0\0\0\n\n\n\n\n\n\n\n\0\0\0\0\0\0\0\0\0\x11\x11\x11\x11\0\0\n\0\0\0\0\0\n\n\0\0\0\0\0\n\n\n\0\0\0\0\0\0\0\0\0\x11\x11\n\n\0\0\0\0\0\0\0\0\0\0\0\0\n\n\n\0\0\0\0\0\0\0\n\n\n\n\0\0\0\0\0\x11\x11\x11\n\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\n\n\0\0\x11\0\0\0\x11\0\0\0\0\x11\0\0\0\0\0\x11\x11\0\n\n\n\n\x11\0\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\0\0\x11\x11\x11\x11\0\0\x11\x11\0\0\0\0\0\0\0\0\0\x11\x11\x11\x11\x11\x11\0\x11\x11\0\0\x11\x11\0\0\0\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\0\x11\0\0\0\x11\0\x11\x11\x11\0\0\x11\x11\0\0\0\0\0\x11\x11\0\0\0\0\0\x11\0\0\x11\0\0\0\0\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x11\x01\x01\x01\x01\x01\x01\x01\x01\x01\x03\x01\x01\x01\x01\x01\x01\r\r\r\r\r\r\r\r\r\r\r\r\r\r\n\n\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\n\n\n\x06\n\x06\0\n\x06\n\n\n\n\n\n\n\n\n\x04\n\n\x03\x03\n\n\n\0\n\x04\x04\n\0\0\0\0\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\x12\0\n\n\x04\x04\x04\n\n\n\n\n\x03\x06\x03\x06\x06\x04\x04\n\n\n\x04\x04\0\n\n\n\n\n\n\n\0\x12\x12\x12\x12\x12\x12\x12\x12\x12\n\n\n\n\n\x12\x12\x11\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\0\0\0\0\0\0\x11\x11\x11\x11\x11\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\n\x01\x11\x11\x11\x01\x11\x11\x01\x01\x01\x01\x01\x11\x11\x11\x11\x01\x01\x01\x01\x01\x01\x01\x01\x11\x11\x11\x01\x01\x01\x01\x11\x01\x01\x01\x01\x01\x11\x11\x01\x01\x01\x01\x01\x01\x01\x01\x01\n\n\n\n\n\n\n\r\r\r\r\x11\x11\x11\x11\r\r\r\r\r\r\r\r\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\r\r\r\r\r\r\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x11\x11\x01\x01\x01\x11\x11\x11\x11\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x11\0\0\x11\x11\0\0\0\0\0\0\0\0\0\0\x11\x11\x11\x11\0\0\x11\x11\0\0\0\0\0\0\0\x11\x11\x11\x11\x11\0\x11\x11\x11\0\0\0\0\0\0\x11\x11\x11\x11\x11\x11\x11\x11\x11\0\0\x11\0\x11\x11\0\0\0\0\0\0\x11\0\0\0\x11\x11\x11\x11\x11\x11\0\x11\0\0\0\0\x11\x11\x11\x11\0\0\0\0\0\0\x11\x11\0\x11\0\0\0\x11\x11\x11\x11\x11\x11\x11\x11\0\0\x11\0\x11\0\0\x11\x11\x11\x11\0\x11\x11\x11\x11\x11\0\0\0\0\0\0\0\0\0\0\0\x11\x11\0\x11\0\x11\x11\x11\x11\x11\x11\0\0\x11\x11\0\0\0\0\0\x11\x11\x11\x11\x11\x11\0\0\x11\x11\x11\x11\0\x11\x11\x11\x11\x11\x11\0\0\x11\x11\x11\0\0\0\0\x11\x11\x11\x11\x11\x11\x11\0\x11\x11\0\0\0\0\0\0\x11\x11\x11\x11\x11\x11\x11\0\x11\x11\x11\x11\x11\x11\0\0\x11\x11\x11\x11\x11\x11\x11\x11\0\0\x11\x11\x11\x11\x11\x11\0\x11\x11\0\x11\x11\0\0\0\0\0\0\0\0\0\x11\x11\x11\x11\x11\x11\0\0\0\x11\0\x11\x11\0\x11\x11\0\0\0\x11\0\x11\0\0\0\0\0\0\0\0\n\n\n\n\n\n\n\n\x04\x04\x04\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\0\0\n\0\x11\0\0\0\0\0\0\0\0\0\0\0\x12\x12\x12\x12\0\0\0\0\0\0\0\0\0\0\0\0\x12\x12\x12\x12\x12\x12\x12\x12\x11\x11\x11\x11\x11\0\0\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\n\n\x11\x11\x11\n\0\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\0\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x11\x11\x11\x11\x11\x11\x11\0\0\0\0\x11\x11\x11\x11\x11\x11\x11\x11\x11\0\0\x11\x11\x11\x11\x11\0\x11\x11\0\x11\x11\x11\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x04\x11\x11\x11\x11\x11\x11\x11\x01\x01\x01\x01\x01\x01\x01\x01\x01\x11\x11\x11\x11\x11\x11\x11\x01\x01\x01\x01\x01\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\n\n\n\n\n\n\n\n\0\0\0\0\n\n\n\n\n\n\n\n\n\0\0\0\0\0\0\0\n\n\n\n\n\n\n\n\n\n\n\n\n\n\0\n\n\n\n\n\n\0\0\0\0\0\0\0\0\n\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x12\x12\x12\0") }, icu_properties::BidiClass(0u8)));
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::BidiClassV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::BidiClassV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_BC_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::BidiClassV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
