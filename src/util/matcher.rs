#![allow(unused)]
use regex::Regex;

pub fn is_custom_emote(msg: &str) -> bool {
    EMOJI_MATCHER.is_match(msg)
}

lazy_static::lazy_static! {
    static ref ROLE_ID_MATCHER: Regex = Regex::new(r"<@&(\d+)>").unwrap();

    static ref CHANNEL_ID_MATCHER: Regex = Regex::new(r"<#(\d+)>").unwrap();

    static ref MENTION_MATCHER: Regex = Regex::new(r"<@!?(\d+)>").unwrap();

    static ref EMOJI_MATCHER: Regex = Regex::new(r"<(a?):([^:\n]+):(\d+)>").unwrap();

    pub static ref QUERY_SYNTAX_REGEX: Regex = Regex::new(r#"\b(?P<key>\w+)(?P<op>(:|=|(>|<)(:|=)?))(?P<value>("".*"")|(\S*))"#).unwrap();
}
