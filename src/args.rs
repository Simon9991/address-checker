use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Path to the file containing the adresses to check
    #[arg(short, long, required = true)]
    pub file_path: String,
}

impl Arguments {
    pub fn new() -> Self {
        let args = Arguments::parse();

        Arguments {
            file_path: args.file_path,
        }
    }
}
