use std::{fs::FileType, path::PathBuf};

use anyhow::{Result, anyhow};
use clap::Parser;

use crate::gram_data::GramData;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct CliArgs {
    #[clap(short, long, help("Input file/directory path"), required(true))]
    input: String,
    #[clap(long, help("Search n-grams"), required(true), value_delimiter(','))]
    ngrams: Vec<String>,
}

impl CliArgs {
    pub fn path(&self) -> Result<PathBuf> {
        let path = PathBuf::from(&self.input);

        if !path.exists() {
            Err(anyhow!("Input file/directory does not exist"))
        } else {
            Ok(path)
        }
    }

    pub fn ngrams(&self) -> Result<GramData> {
        let grams = &self.ngrams;
        if grams.is_empty() {
            return Err(anyhow!(""));
        }

        let first = grams.first().cloned().unwrap_or_default();
        let n = first.len();
        if grams.iter().any(|g| g.len() != n) {
            return Err(anyhow!(""));
        }
        Ok(GramData {
            n,
            grams: grams.to_vec(),
        })
    }
}

#[allow(unused)]
pub trait FileTypeDisplay {
    fn display(&self) -> String;
}

impl FileTypeDisplay for FileType {
    fn display(&self) -> String {
        (if self.is_dir() {
            "Directory"
        } else if self.is_file() {
            "File"
        } else if self.is_symlink() {
            "Symlink"
        } else {
            "Unknown"
        })
        .into()
    }
}
