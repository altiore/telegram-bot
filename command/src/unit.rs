use proc_macro2::Ident;
use quote::quote;

pub fn enum_to_from_str(
    enum_name: &Ident,
    enum_branch: &Ident,
    cmd_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    (
        quote! { #enum_name::#enum_branch => ::core::fmt::Display::fmt(#cmd_str, f) },
        quote! { #enum_name::#enum_branch },
    )
}
