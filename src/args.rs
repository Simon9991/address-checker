use std::{path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Path to the file containing the adresses to check
    #[arg(short, long, required = true)]
    pub file_path_str: String,

    pub file_path_buf: PathBuf,
}

impl Arguments {
    pub fn new() -> Self {
        let args = Arguments::parse();

        let file_path_buf = match PathBuf::from_str(args.file_path_str.as_str()) {
            Ok(value) => value,
            Err(_) => PathBuf::new(),
        };

        Arguments {
            file_path_str: args.file_path_str,
            file_path_buf,
        }
    }
}
