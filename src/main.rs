mod cli;
mod functions;

mod prelude {
    pub use crate::cli::*;
    pub use crate::functions::*;
}

use clap::Parser;
use prelude::*;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Write { text, out_file } => {
            if let Err(value) = write(text.replace(r"\n", "\n"), out_file) {
                eprintln!("Error, while writing file: {}", value);
            }
        }

        Commands::Read { in_file, show_line_numbers} => {
            if let Err(value) = read(in_file, show_line_numbers) {
                eprintln!("Error, while reading file: {}", value);
            }
        }

        Commands::Append { out_file, text, add_line_numbers } => {
            if let Err(value) = append(text.replace(r"\n", "\n"), out_file, add_line_numbers) {
                eprintln!("Error, while appending to file: {}", value);
            }
        }
    }

}
