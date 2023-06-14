//! command line options

use std::path;

/// command line arguments of the application
#[derive(Debug, clap::Parser)]
#[command(author, version, about)]
pub struct Opt {
    /// File(s) to be counted for words
    #[arg(name = "FILE")]
    pub files: Vec<path::PathBuf>,
    /// unit of frequency of occurrence
    #[arg(short = 'u', long = "unit", value_enum, default_value_t = CountUnit::Word)]
    pub count_unit: CountUnit,
}

/// represent the unit of frequency of occurrence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum CountUnit {
    /// count of char
    Char,
    /// count of words
    Word,
    /// count of lines
    Line,
}
