use std::fs::File;

use anyhow::{Context, Result};

use crate::file::FileOperations;

pub fn handle(urls: &[String]) -> Result<()> {
    urls
        .iter()
        .try_for_each(check_url)?;

    File::get(true)?
        .check_duplicates(urls)?
        .write_to_file(urls)
}

fn check_url(url: &String) -> Result<()> {
    reqwest::blocking::get(url).context("Invalid url")?;
    Ok(())
}
