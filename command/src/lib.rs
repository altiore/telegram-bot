mod named;
mod unit;
mod unnamed;
mod utils;

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

fn impl_command_macro(ast: &DeriveInput) -> TokenStream {
    let enum_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    match &ast.data {
        Data::Enum(ref v) => {
            let variants = &v.variants;
            let mut from_str_arms: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut to_str_arms: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut prefixed_commands: Vec<String> = Vec::new();

            for variant in variants {
                let enum_branch = &variant.ident;
                let cmd_str = utils::to_command_str(&enum_branch.to_string());

                match &variant.fields {
                    Fields::Unit => {
                        let r = unit::enum_to_from_str(enum_name, enum_branch, &cmd_str);
                        to_str_arms.push(r.0);
                        from_str_arms.push(r.1);
                    }
                    Fields::Unnamed(fields) => {
                        let r = unnamed::enum_to_from_str(
                            enum_name,
                            enum_branch,
                            &fields.unnamed,
                            &cmd_str,
                        );
                        to_str_arms.push(r.0);
                        from_str_arms.push(r.1);
                    }
                    Fields::Named(fields) => {
                        let r = named::enum_to_from_str(
                            enum_name,
                            enum_branch,
                            &fields.named,
                            &cmd_str,
                        );
                        to_str_arms.push(r.0);
                    }
                }

                prefixed_commands.push(cmd_str);
            }

            if to_str_arms.len() < variants.len() {
                to_str_arms.push(
                    quote! { _ => panic!("Попытка привести к строке необработанную Enum ветку") },
                );
            }

            (quote! {
                use std::str::FromStr;

                fn str_to_vec(text: impl Into<String>) -> Vec<String> {
                    let text = text.into();
                    let mut c = text.chars();
                    match c.next() {
                        None => vec![],
                        Some(_) => {
                            match c.next() {
                                None => vec![],
                                Some(f) => {
                                    if f.to_string() == "[" {
                                        c.as_str().replace("]\"", "").split(",").map(|s| s.trim_matches(&['"', ' ']).to_owned()).collect::<Vec<String>>()
                                    } else {
                                        vec![]
                                    }
                                }
                            }
                        }
                    }
                }

                impl #impl_generics ::core::fmt::Display for #enum_name #ty_generics #where_clause {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error> {
                        match *self {
                            #(#to_str_arms),*
                        }
                    }
                }

                impl From<#enum_name #ty_generics #where_clause> for String {
                    fn from(value: #enum_name) -> Self {
                        format!("{}", value)
                    }
                }

                impl From<&#enum_name #ty_generics #where_clause> for String {
                    fn from(value: &#enum_name) -> Self {
                        format!("{}", value)
                    }
                }

                impl TryFrom<&str> for #enum_name #ty_generics #where_clause {
                    type Error = String;

                    fn try_from(cmd_str: &str) -> Result<Self, Self::Error> {
                        let mut words = cmd_str.split(" ");

                        match words.next() {
                            None => ::std::result::Result::Err(String::from("EmptyString")),
                            Some(cmd_str) => {
                                let params = str_to_vec(words.collect::<Vec<_>>().join(" "));

                                match cmd_str.to_owned().as_str() {
                                    #(
                                        #prefixed_commands => Ok(#from_str_arms),
                                    )*
                                    _ => ::std::result::Result::Err(String::from("NotFound")),
                                }
                            },
                        }
                    }
                }

                impl #enum_name #ty_generics #where_clause {
                    pub fn cmd(&self) -> String {
                        let cmd_str = String::from(self);
                        let mut words = cmd_str.split(" ");

                        match words.next() {
                            None => String::new(),
                            Some(cmd_str) => cmd_str.to_owned(),
                        }
                    }
                }
            }).into()
        }
        // Data::Struct(_ds) => {
        //     (quote! {}).into()
        // },
        _ => panic!("This macro only supports enums."),
    }
}

/// # Examples
/// ```
/// use command::BotCommand;
///
/// #[derive(BotCommand, Debug, PartialEq)]
/// enum Command {
///     Start,
///     WaitForName(String),
///     WaitForTitle(i64, Option<i32>)
/// }
///
/// assert_eq!(String::from(Command::Start), "/start");
/// assert_eq!(String::from(Command::WaitForName("Name".to_owned())), "/wait_for_name \"[\"Name\"]\"");
/// assert_eq!(String::from(Command::WaitForTitle(32, Some(12))), "/wait_for_title \"[32,12]\"");
///
/// assert_eq!(Command::try_from("/start"), Ok(Command::Start));
/// assert_eq!(Command::try_from("/wait_for_name \"[\"Name\"]\""), Ok(Command::WaitForName("Name".to_owned())));
/// assert_eq!(Command::try_from("/wait_for_title \"[32,12]\""), Ok(Command::WaitForTitle(32, Some(12))));
/// ```
#[proc_macro_derive(BotCommand)]
pub fn bot_command_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).expect("TEST >>>>>>>>>>>>>>>>>>>>>>>>>>");

    // Build the trait implementation
    impl_command_macro(&ast)
}
