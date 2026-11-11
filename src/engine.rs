use std::result::Result as StdResult;
use std::{fs::read_dir, path::PathBuf, sync::Mutex};

use aho_corasick::AhoCorasick;
use anyhow::anyhow;
use rayon::prelude::*;

pub type Table = hashbrown::HashMap<usize, u128>;
pub type Result = anyhow::Result<Table>;

#[bitflags2::flags]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    #[flag(0x0000)]
    Undefined,
    #[flag]
    Raw,
    #[flag]
    Sym,
    #[flag]
    File,
    #[flag]
    Dir,
}

impl From<std::fs::FileType> for self::FileType {
    fn from(value: std::fs::FileType) -> Self {
        let mut ty = FileType::Undefined;

        if value.is_dir() {
            ty = FileType::Dir;
        } else if value.is_file() {
            ty = FileType::File;
        }

        if value.is_symlink() {
            ty |= FileType::Sym;
        }

        ty
    }
}

pub fn run_analysis(ty: FileType, path: PathBuf, aho: AhoCorasick) -> Result {
    if ty.has_flag(FileType::Dir) {
        dir_mode(path, aho)
    } else {
        file_mode(path, aho)
    }
}

const PAR_THRES: usize = 5;
fn dir_mode(path: PathBuf, aho: AhoCorasick) -> Result {
    let entries: Vec<_> = read_dir(&path)?.filter_map(StdResult::ok).collect();
    if entries.is_empty() {
        return Err(anyhow!(""));
    }

    let results = Mutex::new(Table::new());
    let len = entries.len();

    let merge = |e: &std::fs::DirEntry| -> anyhow::Result<()> {
        let table = run_analysis(e.file_type()?.into(), e.path(), aho.clone())?;
        let mut results = results.try_lock().unwrap();
        for (id, count) in table {
            let entry = results.entry(id).or_insert(0);
            *entry += count;
        }
        anyhow::Ok(())
    };

    if len < PAR_THRES {
        entries.iter().try_for_each(merge)?;
    } else {
        entries.par_iter().try_for_each(merge)?;
    }

    if let Ok(r) = results.into_inner() {
        Ok(r)
    } else {
        Err(anyhow!(""))
    }
}

fn file_mode(path: PathBuf, aho: AhoCorasick) -> Result {
    let data = std::fs::read_to_string(path)?;

    let mut mapres = Table::new();

    for m in aho.find_overlapping_iter(&data) {
        let id = m.pattern();
        let entry = mapres.entry(id.as_usize()).or_insert(0);
        *entry += 1;
    }

    Ok(mapres)
}
