use std::ops::Not;

use anyhow::{Context, Result};

pub trait StringOperations {
    fn contains_url(&self, pat: &str) -> Result<()>;
    fn split_into_array(&self) -> Vec<Vec<String>>;
}

impl StringOperations for String {
    fn contains_url(&self, pat: &str) -> Result<()> {
        self
            .contains(pat)
            .not()
            .then_some(())
            .context("Url already exist")
    }

    fn split_into_array(&self) -> Vec<Vec<String>> {
        self
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s
                .split(' ')
                .map(|s| s.trim().into())
                .collect())
            .collect()
    }
}

pub trait ToOk: Sized {
    fn to_ok(self) -> Result<Self>;
}

impl<T> ToOk for T {
    fn to_ok(self) -> Result<Self> {
        Ok(self)
    }
}