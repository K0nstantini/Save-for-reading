use anyhow::Result;
use crate::commands::Commands;

mod input;
mod commands;
mod save;
mod args;

fn main() -> Result<()> {
    loop {
        match run() {
            Ok(_) => return Ok(()),
            Err(e) => println!("{e}")
        }
    }
}

fn run() -> Result<()> {
    loop {
        let command = input::handle()?;
        match command {
            Commands::Save(urls) => save::handle(&urls)?,
            Commands::Get => { dbg!(&command); }
            Commands::Exit => return Ok(())
        }
    }
}

