// @generated
/// Implement [`DataProvider<TerminalPunctuationV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_term_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_TERM_V1: &'static <icu_properties::provider::TerminalPunctuationV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\0\"\0\0\0,\0\0\0-\0\0\0.\0\0\0/\0\0\0:\0\0\0<\0\0\0?\0\0\0@\0\0\0~\x03\0\0\x7F\x03\0\0\x87\x03\0\0\x88\x03\0\0\x89\x05\0\0\x8A\x05\0\0\xC3\x05\0\0\xC4\x05\0\0\x0C\x06\0\0\r\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1D\x06\0\0 \x06\0\0\xD4\x06\0\0\xD5\x06\0\0\0\x07\0\0\x0B\x07\0\0\x0C\x07\0\0\r\x07\0\0\xF8\x07\0\0\xFA\x07\0\x000\x08\0\0?\x08\0\0^\x08\0\0_\x08\0\0d\t\0\0f\t\0\0Z\x0E\0\0\\\x0E\0\0\x08\x0F\0\0\t\x0F\0\0\r\x0F\0\0\x13\x0F\0\0J\x10\0\0L\x10\0\0a\x13\0\0i\x13\0\0n\x16\0\0o\x16\0\0\xEB\x16\0\0\xEE\x16\0\x005\x17\0\x007\x17\0\0\xD4\x17\0\0\xD7\x17\0\0\xDA\x17\0\0\xDB\x17\0\0\x02\x18\0\0\x06\x18\0\0\x08\x18\0\0\n\x18\0\0D\x19\0\0F\x19\0\0\xA8\x1A\0\0\xAC\x1A\0\0Z\x1B\0\0\\\x1B\0\0]\x1B\0\0`\x1B\0\0}\x1B\0\0\x7F\x1B\0\0;\x1C\0\0@\x1C\0\0~\x1C\0\0\x80\x1C\0\0< \0\0> \0\0G \0\0J \0\0..\0\0/.\0\0<.\0\0=.\0\0A.\0\0B.\0\0L.\0\0M.\0\0N.\0\0P.\0\0S.\0\0U.\0\0\x010\0\0\x030\0\0\xFE\xA4\0\0\0\xA5\0\0\r\xA6\0\0\x10\xA6\0\0\xF3\xA6\0\0\xF8\xA6\0\0v\xA8\0\0x\xA8\0\0\xCE\xA8\0\0\xD0\xA8\0\0/\xA9\0\x000\xA9\0\0\xC7\xA9\0\0\xCA\xA9\0\0]\xAA\0\0`\xAA\0\0\xDF\xAA\0\0\xE0\xAA\0\0\xF0\xAA\0\0\xF2\xAA\0\0\xEB\xAB\0\0\xEC\xAB\0\0P\xFE\0\0S\xFE\0\0T\xFE\0\0X\xFE\0\0\x01\xFF\0\0\x02\xFF\0\0\x0C\xFF\0\0\r\xFF\0\0\x0E\xFF\0\0\x0F\xFF\0\0\x1A\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0 \xFF\0\0a\xFF\0\0b\xFF\0\0d\xFF\0\0e\xFF\0\0\x9F\x03\x01\0\xA0\x03\x01\0\xD0\x03\x01\0\xD1\x03\x01\0W\x08\x01\0X\x08\x01\0\x1F\t\x01\0 \t\x01\0V\n\x01\0X\n\x01\0\xF0\n\x01\0\xF6\n\x01\0:\x0B\x01\0@\x0B\x01\0\x99\x0B\x01\0\x9D\x0B\x01\0U\x0F\x01\0Z\x0F\x01\0\x86\x0F\x01\0\x8A\x0F\x01\0G\x10\x01\0N\x10\x01\0\xBE\x10\x01\0\xC2\x10\x01\0A\x11\x01\0D\x11\x01\0\xC5\x11\x01\0\xC7\x11\x01\0\xCD\x11\x01\0\xCE\x11\x01\0\xDE\x11\x01\0\xE0\x11\x01\08\x12\x01\0=\x12\x01\0\xA9\x12\x01\0\xAA\x12\x01\0K\x14\x01\0N\x14\x01\0Z\x14\x01\0\\\x14\x01\0\xC2\x15\x01\0\xC6\x15\x01\0\xC9\x15\x01\0\xD8\x15\x01\0A\x16\x01\0C\x16\x01\0<\x17\x01\0?\x17\x01\0D\x19\x01\0E\x19\x01\0F\x19\x01\0G\x19\x01\0B\x1A\x01\0D\x1A\x01\0\x9B\x1A\x01\0\x9D\x1A\x01\0\xA1\x1A\x01\0\xA3\x1A\x01\0A\x1C\x01\0D\x1C\x01\0q\x1C\x01\0r\x1C\x01\0\xF7\x1E\x01\0\xF9\x1E\x01\0C\x1F\x01\0E\x1F\x01\0p$\x01\0u$\x01\0nj\x01\0pj\x01\0\xF5j\x01\0\xF6j\x01\x007k\x01\0:k\x01\0Dk\x01\0Ek\x01\0\x97n\x01\0\x99n\x01\0\x9F\xBC\x01\0\xA0\xBC\x01\0\x87\xDA\x01\0\x8B\xDA\x01\0") }, 278u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::TerminalPunctuationV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::TerminalPunctuationV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_TERM_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::TerminalPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
