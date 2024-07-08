use command::BotCommand;

#[derive(BotCommand, Debug, PartialEq)]
enum Command {
    Test,
    Start(Option<String>),
}

#[test]
fn test_unnamed_option_enum_to_str() {
    assert_eq!(String::from(Command::Start(None)), "/start");
    assert_eq!(
        format!("{}", Command::Start(Some(String::from("test")))),
        "/start \"[\"test\"]\""
    );
}

#[test]
fn test_unnamed_option_enum_from_str() {
    assert_eq!(Command::try_from("/start"), Ok(Command::Start(None)));
    assert_eq!(
        Command::try_from("/start \"[\"test\"]\""),
        Ok(Command::Start(Some("test".to_owned())))
    );
}
