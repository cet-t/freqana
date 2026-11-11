use aho_corasick::AhoCorasick;
use clap::Parser;
use hashbrown::HashMap;

use crate::argparse::*;
use crate::engine::{FileType, Table};
use crate::gram_data::GramData;

mod argparse;
mod engine;
mod gram_data;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let path = args.path()?;
    let ty: FileType = path.metadata()?.file_type().into();
    if ty != FileType::Undefined && !ty.has_flag(FileType::Sym) {
        let ngrams = args.ngrams()?;
        let aho = AhoCorasick::new(&ngrams.grams)?;
        let table = engine::run_analysis(ty, path, aho)?;
        print_result(&ngrams, &table)?;
    } else {
        println!("Unsupport type: {:?}", ty);
    }

    Ok(())
}

fn print_result(ngrams: &GramData, result: &Table) -> anyhow::Result<()> {
    let mut out = HashMap::new();
    for i in 0..ngrams.grams.len() {
        out.insert(&ngrams.grams[i], result.get(&i).unwrap_or(&0));
    }

    println!("{}", serde_json::to_string_pretty(&out)?);

    Ok(())
}
