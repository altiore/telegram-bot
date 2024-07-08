use proc_macro2::{Ident, Span};
use regex::Regex;
use regex_split::RegexSplit;

pub fn to_command_str(text: &str) -> String {
    format!(
        "/{}",
        Regex::new("[A-Z][^A-Z]*")
            .unwrap()
            .split_inclusive(text)
            .filter(|r| *r != "")
            .map(|r| r.to_lowercase())
            .collect::<Vec<String>>()
            .join("_")
    )
}

pub fn get_ident(ident: &Option<Ident>, i: usize) -> Ident {
    Ident::new(
        &match &ident {
            Some(ident) => format!("{}", ident),
            None => format!("field_{}", i),
        },
        Span::call_site(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_command_str() {
        assert_eq!(to_command_str("StartTest"), "/start_test");
    }
}
