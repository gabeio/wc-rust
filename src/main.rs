#[path = "./utils.rs"] mod utils;

use std::env;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The number of characters in the longest input line is written to
    /// the standard output.  When more than one file argument is
    /// specified, the longest input line of all files is reported as the
    /// value of the final “total”.
    #[structopt(short = "L")]
    longest: bool,

    /// The number of bytes in each input file is written to the standard
    /// output.  This will cancel out any prior usage of the -m option.
    #[structopt(short = "c")]
    bytes: bool,

    /// The number of lines in each input file is written to the standard
    /// output.
    #[structopt(short = "l")]
    lines: bool,

    /// The number of characters in each input file is written to the
    /// standard output.  If the current locale does not support
    /// multibyte characters, this is equivalent to the -c option.  This
    /// will cancel out any prior usage of the -c option.
    #[structopt(short = "m")]
    characters: bool,

    /// The number of words in each input file is written to the standard
    /// output.
    #[structopt(short = "w")]
    words: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let lang = env::var("LANG").is_err();
    let lcall = env::var("LC_ALL").is_err();
    let lcctype = env::var("LC_CTYPE").is_err();
    // println!("Hello, world!");
    println!("{}, {}, {}", lang, lcall, lcctype);
    let cli = Cli::from_args();
    println!("{:#?}", cli);
    if cli.longest {
        longest_input_line(cli)
    }
}

fn longest_input_line(cli: Cli) {
    let mut longest = 0;
    while true {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n > longest {
                    longest = n;
                }
                // println!("{} bytes read", n);
                // println!("{}", input);
                // return input;
            }
            Err(error) => println!("{}", longest),
        }
    }
    // println!("{}", longest);
}