use aho_corasick::AhoCorasick;
use clap::Parser;

use crate::argparse::*;
use crate::engine::{FileType, Table};
use crate::gram_data::GramData;

mod argparse;
mod engine;
mod gram_data;

fn main() -> anyhow::Result<()> {
    let args = &CliArgs::parse();

    let path = args.path()?;
    let ty: FileType = path.metadata()?.file_type().into();
    if ty != FileType::Undefined && !ty.has_flag(FileType::Sym) {
        let ngrams = &args.ngrams()?;
        let aho = AhoCorasick::new(&ngrams.grams)?;
        let table = engine::run_analysis(ty, path, aho)?;
        print_result(ngrams, table)?;
    } else {
        println!("Unsupport type: {:?}", ty);
    }

    Ok(())
}

fn print_result(ngrams: &GramData, result: Table) -> anyhow::Result<()> {
    for i in 0..ngrams.grams.len() {
        let i = &i;
        println!("{}: {}", ngrams.grams[*i], result.get(i).unwrap_or(&0));
    }

    // let out = toml::to_string_pretty(&result)?;
    // println!("{}", out);
    Ok(())
}
