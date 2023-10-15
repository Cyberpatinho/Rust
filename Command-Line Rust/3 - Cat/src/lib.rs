use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buffer) => {
                display_lines(buffer, 
                    config.number_lines, 
                    config.number_nonblank_lines)?;
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn display_lines(file: Box<dyn BufRead>, number_lines: bool, number_nonblank_lines: bool) -> MyResult<()> {
    
    let mut index = 0;
    for line_result in file.lines() {
        
        let line = line_result.unwrap();
        let is_blank = line.trim().is_empty();

        let prefix = if number_lines || (number_nonblank_lines && !is_blank) {
            format!("     {}	", index + 1)
        }
        else {
            "".to_string()
        };

        println!("{}{}", prefix, line);

        if !prefix.is_empty() {
            index += 1;
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")

        .author("Anon <anon@anonmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
            .default_value("-")
            .value_name("FILE")
            .help("Input file(s)")
            .required(true)
            .multiple(true)
        )
        .arg(
            Arg::with_name("number_lines")
            .short("n")
            .long("number")
            .help("Echo numbered lines")
            .takes_value(false)
            .conflicts_with("number_nonblank_lines")
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
            .short("b")
            .long("number-nonblank")
            .help("Echo numbered lines excluding blank ones")
            .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines")
    })
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}