use anyhow::Result;

use crate::commands::{Commands, save, get};

mod input;
mod args;
mod util;
mod file;
mod commands;

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
            Commands::Get => {
                let url = get::retrieve_random()?;
                println!("{url}");
            }
            Commands::Exit => return Ok(())
        }
    }
}

