use std::{env, path::PathBuf};

use hashbrown::HashMap;
use once_cell::sync::OnceCell;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, UserMarker},
    Id,
};

use crate::{util::Emote, BotResult, Error};

pub static CONFIG: OnceCell<BotConfig> = OnceCell::new();

#[derive(Debug)]
pub struct BotConfig {
    pub database_url: String,
    pub discord_token: String,
    pub emotes: HashMap<Emote, String>,
    pub owner: Id<UserMarker>,
    pub dev_guild: Id<GuildMarker>,
}

impl BotConfig {
    pub fn init() -> BotResult<()> {
        let emotes = HashMap::new();
        // let emotes = [];

        // let emotes = emotes
        //     .iter()
        //     .map(|emote_str| {
        //         let key = emote_str.parse().unwrap();
        //         let value = env_var(emote_str)?;

        //         Ok((key, value))
        //     })
        //     .collect::<BotResult<_>>()?;

        let config = BotConfig {
            database_url: env_var("DATABASE_URL")?,
            discord_token: env_var("DISCORD_TOKEN")?,
            emotes,
            owner: env_var("OWNER_USER_ID")?,
            dev_guild: env_var("DEV_GUILD_ID")?,
        };

        if CONFIG.set(config).is_err() {
            warn!("CONFIG was already set");
        }

        Ok(())
    }
}

trait EnvKind: Sized {
    const EXPECTED: &'static str;

    fn from_str(s: &str) -> Option<Self>;
}

macro_rules! env_kind {
    ($($ty:ty: $arg:ident => $impl:block,)*) => {
        $(
            impl EnvKind for $ty {
                const EXPECTED: &'static str = stringify!($ty);

                fn from_str($arg: &str) -> Option<Self> {
                    $impl
                }
            }
        )*
    };
}

env_kind! {
    u16: s => { s.parse().ok() },
    u64: s => { s.parse().ok() },
    PathBuf: s => { s.parse().ok() },
    String: s => { Some(s.to_owned()) },
    Id<UserMarker>: s => { s.parse().ok().map(Id::new) },
    Id<GuildMarker>: s => { s.parse().ok().map(Id::new) },
    Id<ChannelMarker>: s => { s.parse().ok().map(Id::new) },
    [u8; 4]: s => {
        if !(s.starts_with('[') && s.ends_with(']')) {
            return None
        }

        let mut values = s[1..s.len() - 1].split(',');

        let array = [
            values.next()?.trim().parse().ok()?,
            values.next()?.trim().parse().ok()?,
            values.next()?.trim().parse().ok()?,
            values.next()?.trim().parse().ok()?,
        ];

        Some(array)
    },
}

fn env_var<T: EnvKind>(name: &'static str) -> BotResult<T> {
    let value = env::var(name).map_err(|_| Error::MissingEnvVariable(name))?;

    T::from_str(&value).ok_or(Error::ParsingEnvVariable {
        name,
        value,
        expected: T::EXPECTED,
    })
}
