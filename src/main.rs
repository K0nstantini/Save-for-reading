use anyhow::Result;
use crate::commands::Commands;

mod input;
mod commands;

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    loop {
        let command = match input::handle() {
            Ok(command) => {
                dbg!(&command);
                command
            }
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        match command {
            Commands::Save(_) => { dbg!(&command); }
            Commands::Get => { dbg!(&command); }
            Commands::Exit => return Ok(())
        }
    }
}

