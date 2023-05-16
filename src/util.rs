use std::ops::Not;
use anyhow::{Result, Context};

pub trait StringOperations {
    fn contains_url(&self, pat: &str) -> Result<()>;
}

impl StringOperations for String {
    fn contains_url(&self, pat: &str) -> Result<()> {
        self
            .contains(pat)
            .not()
            .then_some(())
            .context("Url already exist")
    }
}
