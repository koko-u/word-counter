use std::path;

#[derive(Debug, clap::Parser)]
#[command(author, version, about)]
pub struct Opt {
    /// File(s) to be counted for words
    #[arg(name = "FILE")]
    pub files: Vec<path::PathBuf>,
    #[arg(short = 'u', long = "unit", value_enum, default_value_t = CountUnit::Word)]
    pub count_unit: CountUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum CountUnit {
    /// count of char
    Char,
    /// count of words
    Word,
    /// count of lines
    Line,
}
