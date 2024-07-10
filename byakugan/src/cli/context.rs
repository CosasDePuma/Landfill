use super::console::Console;

pub struct Context {
    exit_signal: bool,
    pub metakey: bool,
    pub refresh_rate: u64,
    pub terminal: Console,
}
impl Context {
    pub const fn new(terminal: Console, refresh_rate: u64) -> Self {
        Self {
            exit_signal: false,
            metakey: false,
            refresh_rate,
            terminal,
        }
    }

    pub fn exit(&mut self) {
        self.exit_signal = true;
    }

    pub const fn is_running(&self) -> bool {
        !self.exit_signal
    }
}