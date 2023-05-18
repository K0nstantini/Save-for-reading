use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};

use anyhow::Result;
use clap::Parser;

use crate::args::Args;
use crate::util::{StringOperations, ToOk};

pub trait FileOperations: Sized {
    fn get(append: bool) -> Result<Self>;
    fn check_duplicates(&mut self, strings: &[String]) -> Result<&mut Self>;
    fn write_to_file(&mut self, strings: &[String]) -> Result<()>;
    fn read_from_file(&mut self) -> Result<Vec<Vec<String>>>;
    fn remove_from_file(&mut self, url: &str) -> Result<()>;
}

impl FileOperations for File {
    fn get(append: bool) -> Result<Self> {
        let file_name = Args::parse().path;
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(append)
            .open(file_name)?
            .to_ok()
    }

    fn check_duplicates(&mut self, strings: &[String]) -> Result<&mut Self> {
        let contents = read_to_string(self)?;
        strings
            .iter()
            .try_for_each(|s| contents.contains_url(s))?;
        Ok(self)
    }

    fn write_to_file(&mut self, strings: &[String]) -> Result<()> {
        let string = format!("\n{}", strings.join(" "));
        self
            .write_all(string.as_bytes())?
            .to_ok()
    }

    fn read_from_file(&mut self) -> Result<Vec<Vec<String>>> {
        read_to_string(self)?
            .split_into_array()
            .to_ok()
    }

    fn remove_from_file(&mut self, url: &str) -> Result<()> {
        let contents = read_to_string(self)?
            .replace(&format!("{url} "), "")
            .replace(&format!("\r\n{url}"), "")
            .replace(&format!("{url}\r\n"), "")
            .replace(url, "");
        fs::write(Args::parse().path, contents)?.to_ok()
    }
}

fn read_to_string(file: &mut File) -> Result<String> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
