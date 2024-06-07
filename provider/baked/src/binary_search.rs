// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data stored as slices, looked up with binary search

#[cfg(feature = "export")]
use databake::*;
#[cfg(feature = "export")]
use icu_provider::prelude::*;

#[cfg(feature = "export")]
pub fn bake(
    struct_type: &TokenStream,
    data: impl IntoIterator<Item = ((DataLocale, DataMarkerAttributes), proc_macro2::Ident)>,
) -> TokenStream {
    let mut data = data
        .into_iter()
        .map(|((l, a), i)| {
            (
                l.to_string(),
                a.to_string(),
                quote!(#i),
            )
        })
        .collect::<Vec<_>>();

    data.sort_by(|(al, aa, _), (bl, ba, _)| (al, aa).cmp(&(bl, ba)));
    let n = data.len();

    if data.iter().any(|(_, a, _)| !a.is_empty()) {
        let data = data.iter().map(|(l, a, i)| quote!((#l, #a, &#i)));

        quote! {
            static DATA: [(&str, &str, & #struct_type); #n] = [#(#data,)*];
            fn lookup(req: icu_provider::DataRequest) -> Option<&'static #struct_type> {
                DATA.binary_search_by(|(l, a, _)| req.locale.strict_cmp(l.as_bytes()).reverse().then_with(|| a.cmp(&&**req.marker_attributes)))
                    .map(|i| (*unsafe { DATA.get_unchecked(i) }).2)
                    .ok()
            }
        }
    } else {
        let data = data.iter().map(|(l, _, i)| quote!((#l, &#i)));

        quote! {
            static DATA: [(&str, & #struct_type); #n] = [#(#data,)*];
            fn lookup(req: icu_provider::DataRequest) -> Option<&'static #struct_type> {
                DATA.binary_search_by(|(l, _)| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(|i| (*unsafe { DATA.get_unchecked(i) }).1)
                    .ok()
            }
        }
    }
}
