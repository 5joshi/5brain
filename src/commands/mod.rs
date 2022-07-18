use twilight_interactions::command::{CommandOption, CreateOption};

pub mod utility;

#[derive(Copy, Clone, CommandOption, CreateOption, Eq, PartialEq)]
pub enum ShowHideOption {
    #[option(name = "Show", value = "show")]
    Show,
    #[option(name = "Hide", value = "hide")]
    Hide,
}

#[derive(Copy, Clone, CommandOption, CreateOption, Eq, PartialEq)]
pub enum EnableDisable {
    #[option(name = "Enable", value = "enable")]
    Enable,
    #[option(name = "Disable", value = "disable")]
    Disable,
}

#[derive(CommandOption, CreateOption)]
pub enum ThreadChannel {
    #[option(name = "Stay in channel", value = "channel")]
    Channel,
    #[option(name = "Start new thread", value = "thread")]
    Thread,
}
