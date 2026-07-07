use std::path::PathBuf;

use aho_corasick::AhoCorasick;

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

fn dir_mode(_path: PathBuf, _aho: AhoCorasick) -> Result {
    unimplemented!()
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
