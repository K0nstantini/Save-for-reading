use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use anyhow::{bail, Context, Result};
use clap::Parser;
use reqwest::StatusCode;

use crate::args::Args;
use crate::util::StringOperations;

trait FileOperations {
    fn check_duplicates(&mut self, strings: &[String]) -> Result<&mut Self>;
    fn write_to_file(&mut self, strings: &[String]) -> Result<()>;
}

impl FileOperations for File {
    fn check_duplicates(&mut self, strings: &[String]) -> Result<&mut Self> {
        let mut contents = String::new();
        self.read_to_string(&mut contents)?;
        strings
            .iter()
            .try_for_each(|s| contents.contains_url(s))?;
        Ok(self)
    }

    fn write_to_file(&mut self, strings: &[String]) -> Result<()> {
        let string = format!("\n{}", strings.join(" "));
        self.write_all(string.as_bytes())?;
        Ok(())
    }
}

pub fn handle(urls: &[String]) -> Result<()> {
    urls
        .iter()
        .try_for_each(check_url)?;

    get_file()?
        .check_duplicates(urls)?
        .write_to_file(urls)?;

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
    reqwest::blocking::get(url).context("Invalid url")?;
    Ok(())
}
