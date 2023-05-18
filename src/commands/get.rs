use std::fs::File;

use anyhow::{Context, Result};
use rand::prelude::SliceRandom;

use crate::file::FileOperations;

pub fn retrieve_random() -> Result<String> {
    let mut rng = rand::thread_rng();
    let urls = File::get(false)?
        .read_from_file()?;
    let url = urls
        .choose(&mut rng)
        .context("No content found")?
        .first()
        .context("Url is empty")?;

    File::get(false)?
        .remove_from_file(url)?;

    Ok(url.into())
}