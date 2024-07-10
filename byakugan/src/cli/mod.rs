use crate::prelude::*;

mod console;
mod context; use context::Context;
mod layout;

pub fn run() -> Result<()> {
    console::panic_hook();
    
    let terminal = console::enter_raw_mode()?;
    let mut ctx = Context::new(terminal, 60);
    
    let err = console::render(&mut ctx);

    console::exit_raw_mode()?;
    err
}