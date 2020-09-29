#[path = "./utils.rs"] mod utils;

use std::env;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;
use std::collections::HashMap;
use std::convert::TryFrom;

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
    #[structopt(name = "FILE")]
    files: Vec<String>,
}

fn main() {
    let lang = env::var("LANG").is_err();
    let lcall = env::var("LC_ALL").is_err();
    let lcctype = env::var("LC_CTYPE").is_err();
    // println!("Hello, world!");
    println!("{}, {}, {}", lang, lcall, lcctype);
    let cli = Cli::from_args();
    // println!("{:#?}", cli);
    if cli.longest && cli.files.len() > 0 {
        longest_input_line_file(cli)
    } else if cli.longest {
        longest_input_line(cli)
    } else if cli.lines && cli.files.len() > 0 {
        line_count_file(cli)
    } else if cli.lines {
        line_count(cli)
    } else if cli.words && cli.files.len() > 0 {
        word_count_file(cli)
    } else if cli.words {
        word_count(cli)
    }
}

fn longest_input_line_file(cli: Cli) {
    let mut longest = 0;
    let mut longest_filename: String = String::new();

    for filename in cli.files {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(&filename).unwrap();
        let reader = BufReader::new(file);

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            if line.len() > longest {
                longest = line.len();
                longest_filename = filename.clone();
            }
        }
    }

    println!("{} {}", longest, longest_filename);
}

fn longest_input_line(_cli: Cli) {
    let mut longest = 0;
    loop {
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
            Err(error) => println!("{}", error),
        }
    }
    println!("{}", longest);
}

fn line_count_file(cli: Cli) {
    // let mut counts: HashMap<String, i64> = HashSet:<String, i64>::new();
    let mut counts: HashMap<String, usize> = HashMap::new();

    for filename in cli.files {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(&filename).unwrap();
        let reader = BufReader::new(file);

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        if let Some(x) = counts.get_mut(&filename) {
            *x += reader.lines().count();
        } else {
            counts.insert(filename.clone(), reader.lines().count());
        }
    }

    let mut total = 0;

    for (k,v) in counts {
        total += v;
        println!("{} {}", k, v);
    }

    println!("{} {}", total, "total");
}

fn line_count(_cli: Cli) {
    let mut total = 0;
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => total += 1,
            Err(error) => println!("{}", error),
        }
    }
    println!("{}", total);
}

fn word_count_file(cli: Cli) {
    let mut words: HashMap<String, i64> = HashMap::new();

    for filename in cli.files {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(&filename).unwrap();
        let reader = BufReader::new(file);

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            let count: i64 = i64::try_from(line.split_whitespace().count()).ok().unwrap();
            if let Some(word) = words.get_mut(&filename) {
                *word += count;
            } else {
                words.insert(filename.clone(), count);
            }
        }
    }

    let mut total = 0;

    for (k,v) in words {
        total += v;
        println!("{} {}", k, v);
    }

    println!("{} {}", total, "total");
}

fn word_count(_cli: Cli) {
    let mut words = 0;
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                words += input.split_whitespace().count();
            },
            Err(error) => println!("{}", error),
        }
    }
    println!("{}", words);
}
