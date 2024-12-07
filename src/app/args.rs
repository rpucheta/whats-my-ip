pub enum Command {
    Public,
    Private,
    Invalid,
}

pub fn parse_args(args: Vec<String>) -> Command {
    if args.len() > 1 {
        match args[1].as_str() {
            "-public" => Command::Public,
            "-private" => Command::Private,
            _ => Command::Invalid,
        }
    } else {
        Command::Invalid
    }
}