use command::BotCommand;

#[derive(BotCommand, Debug, PartialEq)]
enum Command {
    Start,
    NameWait,
}

#[test]
fn test_unit_enum_to_str() {
    assert_eq!(String::from(Command::Start), "/start");
    assert_eq!(String::from(Command::NameWait), "/name_wait");
}

#[test]
fn test_str_to_unit_enum() {
    assert_eq!(Command::try_from("/start"), Ok(Command::Start));
    assert_eq!(Command::try_from("/name_wait"), Ok(Command::NameWait));
}
