use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

pub mod save;
pub mod get;

#[derive(Debug)]
pub enum Commands {
    Save(Vec<String>),
    Get,
    Exit,
}

impl FromStr for Commands {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = match s {
            str if str.starts_with("save") => get_save_command(str)?,
            "get" => Commands::Get,
            "exit" => Commands::Exit,
            _ => bail!("Parsing command error")
        };
        Ok(command)
    }
}

fn get_save_command(s: &str) -> Result<Commands> {
    let (_, urls) = s.split_once(' ')
        .context("Passing save command error")?;
    let urls = urls
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.into())
        .collect();
    Ok(Commands::Save(urls))
}
