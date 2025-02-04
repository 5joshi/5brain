bitflags::bitflags! {
    pub struct CommandFlags: u8 {
        const AUTHORITY   = 1 << 0;
        const EPHEMERAL   = 1 << 1;
        const ONLY_GUILDS = 1 << 2;
        const ONLY_OWNER  = 1 << 3;
        const SKIP_DEFER  = 1 << 4;
    }
}

impl CommandFlags {
    pub fn defer(self) -> bool {
        !self.contains(CommandFlags::SKIP_DEFER)
    }

    pub fn ephemeral(self) -> bool {
        self.contains(CommandFlags::EPHEMERAL)
    }
}
