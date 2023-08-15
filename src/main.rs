extern crate clap;

use std::io::{self, BufRead, BufWriter, Write};
use std::fs::File;
use clap::{App, Arg};

fn main() -> io::Result<()> {
    // defining command-line arguments using clap
    let matches = App::new("xSplit")
        .version("1.0")
        .author("AJ Sethi <aditya.sethi@anu.edu.au>")
        .about("Splits a table into n files, grouping by sequential shared values in a common column (c) 2023 AJ Sethi")
        .arg(Arg::with_name("splits")
            .short("n")
            .long("splits")
            .value_name("NUMBER")
            .help("Number of splits")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("prefix")
            .short("p")
            .long("prefix")
            .value_name("PREFIX")
            .help("Prefix for the output files")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("column")
            .short("c")
            .long("column")
            .value_name("INDEX")
            .help("Common column index")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("delimiter")
            .short("d")
            .long("delimiter")
            .value_name("DELIMITER")
            .help("Delimiter for splitting columns (default: tab)")
            .takes_value(true))
        .get_matches();

    // parsing command-line arguments
    let n: usize = matches.value_of("splits").unwrap().parse().expect("Number of splits should be a positive integer");
    let prefix = matches.value_of("prefix").unwrap().to_string();
    let c: usize = matches.value_of("column").unwrap().parse().expect("Common column index should be a positive integer");
    let c = if c > 0 { c - 1 } else { panic!("Column index must be greater than 0"); };
    let delimiter = matches.value_of("delimiter").unwrap_or("\t").to_string();

    // preparing to read from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let header = lines.next().unwrap()?;
    if !header.contains(&delimiter) {
        eprintln!("Warning: No delimiter values observed in the input. Please check the delimiter setting.");
    }

    // initializing file writers
    let mut file_index = 0;
    let mut current_value = String::new();
    let mut writers: Vec<BufWriter<File>> = (0..n)
        .map(|i| {
            let filename = format!("{}{}.tsv", prefix, i);
            let file = File::create(&filename).unwrap();
            BufWriter::new(file)
        })
        .collect();

    // write header to all files
    for writer in &mut writers {
        writeln!(writer, "{}", header)?;
    }

    // process lines, write to appropriate file based on common column value
    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split(&delimiter).collect();
        if current_value != parts[c] {
            file_index = (file_index + 1) % n;
            current_value = parts[c].to_string();
        }
        writeln!(writers[file_index], "{}", line)?;
    }

    Ok(())
}
