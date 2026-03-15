#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Debug,
    Help,
    Exit,
    Quit,
    Unknown,
}

impl Command {
    pub fn from_input(input: &str) -> Self {
        match input.to_lowercase().as_str() {
            "debug" => Command::Debug,
            "help" => Command::Help,
            "exit" => Command::Exit,
            _ => Command::Unknown,
        }
    }
}
