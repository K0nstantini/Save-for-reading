use std::fs::{File, OpenOptions};
use anyhow::{bail, Context, Result};
use clap::Parser;
use reqwest::StatusCode;
use crate::args::Args;

pub fn handle(urls: &[String]) -> Result<()> {
    urls
        .into_iter()
        .map(check_url)
        .collect::<Result<_>>()?;

    // let file = get_file()?;
    Ok(())
}

fn get_file() -> Result<File> {
    let file_name = Args::parse().path;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;
    Ok(file)
}

fn check_url(url: &String) -> Result<()> {
    let res = reqwest::blocking::get(url)
        .context("Invalid url")?;
    if res.status() != StatusCode::OK {
        bail!("Invalid url");
    }
    Ok(())
}