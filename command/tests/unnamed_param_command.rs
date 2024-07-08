use command::BotCommand;

#[derive(BotCommand, Debug, PartialEq)]
enum Command {
    One(i64),
    Two(String, Option<String>),
    Tree(String, i32, u8),
}

#[test]
fn test_unnamed_enum_to_str() {
    assert_eq!(String::from(Command::One(1)), "/one \"[1]\"");
    assert_eq!(
        String::from(Command::Two(String::from("one"), Some(String::from("two")))),
        "/two \"[\"one\",\"two\"]\""
    );
    assert_eq!(
        String::from(Command::Tree(String::from("one"), -12, 1)),
        "/tree \"[\"one\",-12,1]\""
    );
}

#[test]
fn test_unnamed_str_to_enum() {
    assert_eq!(Command::try_from("/one \"[2]\""), Ok(Command::One(2)));
    assert_eq!(
        Command::try_from("/two \"[\"one\", \"two\"]\""),
        Ok(Command::Two("one".to_owned(), Some("two".to_owned())))
    );
    assert_eq!(
        Command::try_from("/tree \"[\"one\",-12,1]\""),
        Ok(Command::Tree("one".to_owned(), -12, 1))
    );
}
