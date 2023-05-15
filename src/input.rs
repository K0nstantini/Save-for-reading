use std::io;
use std::str::FromStr;

use anyhow::Result;

use crate::commands::Commands;

pub fn handle() -> Result<Commands> {
    let line = read_line()?;
    Commands::from_str(&line)
}

fn read_line() -> Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().into())
}
