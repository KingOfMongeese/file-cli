use clap::{Parser, Subcommand};

/// A simple file util cli
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {

    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {

    /// Write to a file
    Write {

        /// The text to write
        text: String,

        /// The file to write to, will create all paths if they dont exist
        #[clap(short='o', required = true)]
        out_file: String,
    },

    Read {
        /// the file to read
        in_file: String,

        /// show the line numbers when printing
        #[clap(short='n', default_value_t=false)]
        show_line_numbers: bool,

    },
}