use std::path;

#[derive(Debug, clap::Parser)]
#[command(author, version, about)]
pub struct Opt {
    /// File(s) to be counted for words
    #[arg(name = "FILE")]
    pub files: Vec<path::PathBuf>,
}
