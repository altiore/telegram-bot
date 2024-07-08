use telegram_bot::*;

// #[derive(BotCommand, Debug, PartialEq)]
// pub struct BookStruct {
//     id: i64,
//     name: String,
// }

#[derive(BotCommand, Debug, PartialEq)]
pub enum MyEnum {
    Start,
    Vote { book_id: i32 },
    WithStr { book_id: i32, user_id: i64, sss: String },
    Rate { book_id: i32, user_id: i64 },
    Unnamed(String),
    // Nested { book: BookStruct },
    Unknown(String),
}

#[test]
fn it_bot_command_enum_to_str_unit() {
    assert_eq!(&format!("{}", MyEnum::Start), "/start");
}

#[test]
fn it_bot_command_enum_to_str_unnamed_none() {
    assert_eq!(&format!("{}", MyEnum::Unknown("".to_owned())), "/unknown \"[\"\"]\"");
}

#[test]
fn it_bot_command_enum_to_str_unnamed() {
    assert_eq!(&format!("{}", MyEnum::Unnamed("test".to_owned())), "/unnamed \"[\"test\"]\"");
}

#[test]
fn it_bot_command_enum_to_str_named() {
    assert_eq!(&format!("{}", MyEnum::Vote { book_id: 1 }), "/vote \"{\"book_id\":1}\"");
    assert_eq!(&format!("{}", MyEnum::WithStr { book_id: 2, user_id: 21323131312, sss: String::from("test_string") }), "/withstr \"{\"book_id\":2,\"user_id\":21323131312,\"sss\":test_string}\"");
    assert_eq!(&format!("{}", MyEnum::Rate { book_id: 2, user_id: 21323131312 }), "/rate \"{\"book_id\":2,\"user_id\":21323131312}\"");
}

// #[test]
// fn it_bot_command_enum_nested_struct() {
//     assert_eq!(&format!("{}", MyEnum::Nested { book: BookStruct { id: 43, name: String::from("Book Name") } }), "/vote \"{\"book_id\":1}\"");
// }

// #[test]
// fn it_from_string_works() {
//     assert_eq!(MyEnum::from("/start"), MyEnum::Start);
// }
