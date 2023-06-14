use std::path;

#[derive(Debug, clap::Parser)]
#[command(author, version, about)]
pub struct Opt {
    pub files: Vec<path::PathBuf>,
}
