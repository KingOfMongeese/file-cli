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

    /// Read data from a file, print it to console
    Read {
        /// the file to read
        in_file: String,

        /// show the line numbers when printing
        #[clap(short='n', default_value_t=false)]
        show_line_numbers: bool,

    },

    /// Append data to a file
    Append {

        /// the file to append to
        #[clap(short='o', required = true)]
        out_file: String,

        /// the text to write
        text: String,

        /// include line numbers
        #[clap(short='n', default_value_t=false)]
        add_line_numbers: bool,
    }
}