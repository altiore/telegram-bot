use crate::utils::get_ident;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Field, GenericArgument, PathArguments, Token, Type};

fn type_to_token_stream(
    ident: &Option<Ident>,
    ty: &Type,
    i: usize,
) -> Result<(TokenStream, TokenStream, TokenStream), String> {
    if let Type::Path(t_p) = ty {
        match t_p.path.segments.len() {
            1 => {
                let ident = get_ident(&ident, i);
                let prop_ref = quote! { #ident };
                let mut prop_to_str = quote! {};
                let mut prop_value = quote! {};
                match t_p.path.segments[0].ident.to_string().as_str() {
                    "String" => {
                        prop_to_str = quote! { format!("\"{}\"", #ident) };
                        prop_value = quote! { <#ty>::from_str(&params[#i]).unwrap_or_default(), };
                    }
                    "Option" => {
                        if let PathArguments::AngleBracketed(ref args) =
                            &t_p.path.segments[0].arguments
                        {
                            if let GenericArgument::Type(ref ty) = &args.args[0] {
                                // TODO: возможно, есть смысл сделать здесь рекурсивную обработку данных
                                if let Type::Path(t_p) = ty {
                                    prop_value = quote! {
                                        {
                                            if params.len() == 0 {
                                                None
                                            } else {
                                                if &params[#i] == "" {
                                                    None
                                                } else {
                                                    Some(<#ty>::from_str(&params[#i]).unwrap_or_default())
                                                }
                                            }
                                        },
                                    };

                                    match t_p.path.segments[0].ident.to_string().as_str() {
                                        "String" => {
                                            prop_to_str = quote! {match &#ident {
                                                Some(t) => format!("\"{}\"", t),
                                                None => String::new(),
                                            }};
                                        }
                                        _ => {
                                            prop_to_str = quote! {match &#ident {
                                                Some(t) => format!("{}", t),
                                                None => String::new(),
                                            }};
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        prop_to_str = quote! { format!("{}", #ident) };
                        prop_value = quote! { <#ty>::from_str(&params[#i]).unwrap_or_default(), };
                    }
                }

                return Ok((prop_ref, prop_to_str, prop_value));
            }
            _ => Err(String::from(
                "Default parser works only with exactly 1 field",
            )),
        }
    } else {
        Err(String::from("Default parser works only with Type::Path"))
    }
}

pub fn enum_to_from_str(
    enum_name: &Ident,
    enum_branch: &Ident,
    fields: &Punctuated<Field, Token![,]>,
    cmd_str: &str,
) -> (TokenStream, TokenStream) {
    let mut prop_refs: Vec<TokenStream> = Vec::new();
    let mut prop_values = quote! {};
    let mut prop_to_strs: Vec<TokenStream> = Vec::new();

    fields
        .iter()
        .enumerate()
        .for_each(|(i, f)| match type_to_token_stream(&f.ident, &f.ty, i) {
            Ok((prop_ref, prop_to_str, prop_value)) => {
                prop_refs.push(prop_ref);
                prop_to_strs.push(prop_to_str);
                prop_values.extend(prop_value);
            }
            Err(_s) => {}
        });

    let s = quote! {
        {
            let s = [#(&*#prop_to_strs),*].join(",");
            if s.len() == 0 {
                String::new()
            } else {
                [" \"[", &s, "]\""].join("")
            }
        }
    };
    (
        quote! { #enum_name::#enum_branch ( #(ref #prop_refs),* ) =>
        ::core::fmt::Display::fmt(&format!("{}{}", #cmd_str, #s), f) },
        quote! {
            {
                #enum_name::#enum_branch(#prop_values)
            }
        },
    )
}
