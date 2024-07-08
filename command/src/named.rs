use proc_macro2::{Ident, Span};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Field, Token, Type};

pub fn enum_to_from_str(
    enum_name: &Ident,
    enum_branch: &Ident,
    fields: &Punctuated<Field, Token![,]>,
    cmd_str: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let l = fields.len();
    let mut s = String::from(" \"{{");
    let mut prop_idents: Vec<proc_macro2::TokenStream> = vec![];
    let mut initialization = quote! {};

    fields.iter().enumerate().for_each(|(i, f)| {
        let ty = &f.ty;
        initialization.extend(quote! { <#ty>::from_str(&params[#i]).unwrap_or_default(), });

        if let Type::Path(t_p) = &f.ty {
            let ident = Ident::new(
                &match &f.ident {
                    Some(ident) => format!("{}", ident),
                    None => format!("field_{}", i),
                },
                Span::call_site(),
            );
            prop_idents.push(quote! { #ident });

            let param_type_name = format!("{}", t_p.path.segments[0].ident);
            match param_type_name.as_str() {
                "String" => {
                    s.push_str(&format!(
                        "\"{{{}}}\"{}",
                        i,
                        if i < l - 1 { "," } else { "}}\"" }
                    ));
                }
                _ => {
                    s.push_str(&format!(
                        "{{{}}}{}",
                        i,
                        if i < l - 1 { "," } else { "}}\"" }
                    ));
                }
            }
        }
    });

    (
        quote! { #enum_name::#enum_branch { #(ref #prop_idents),* } => ::core::fmt::Display::fmt(&format!("{}{}", #cmd_str, format!(#s, #(&#prop_idents),*)), f) },
        quote! {
            {
                #enum_name::#enum_branch { #initialization }
            }
        },
    )
}
