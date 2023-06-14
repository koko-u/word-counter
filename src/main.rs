use std::fs;
use std::io;
use std::io::BufRead;

use clap::Parser;
use env_logger::Env;
use error_stack::IntoReport;
use error_stack::ResultExt;
use word_counter::errors::AppError;
use word_counter::freqs::Frequency;
use word_counter::opts::Opt;

fn main() -> error_stack::Result<(), AppError> {
    dotenv::dotenv().into_report().change_context(AppError)?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let opt = Opt::parse();

    let mut frequency = Frequency::default();
    for file_path in opt.files {
        let file = match fs::File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                log::error!("skip file: {e:?}");
                continue;
            }
        };
        let reader = io::BufReader::new(file);
        let freqs = reader
            .lines()
            .map(|line| line.map(|line| Frequency::builder(line).by(opt.count_unit)))
            .collect::<Result<Vec<_>, _>>();
        let freq = match freqs {
            Ok(freqs) => freqs
                .into_iter()
                .fold(Frequency::default(), |mut acc, freq| {
                    acc.merge(freq.iter());
                    acc
                }),
            Err(e) => {
                log::error!("skip file: {e:?}");
                continue;
            }
        };

        frequency.merge(freq.iter());
    }

    println!("{:#?}", frequency.into_inner());

    Ok(())
}
