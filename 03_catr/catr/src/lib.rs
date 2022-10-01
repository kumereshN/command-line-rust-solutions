use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}


// <T> means return any data type
type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files{
        match open(&filename){
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => read_file_string(&filename, &config.number_lines, &config.number_nonblank_lines)?,
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config>{
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Kumeresh <kumeresh6@hotmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number-nonblank"),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>>{
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_file_string(filename: &str, no_of_lines: &bool, no_of_blank_lines: &bool) -> MyResult<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    if *no_of_lines{
        for (index, line) in reader.lines().enumerate(){
            let line = line.unwrap();
            println!("     {}	{}", index+1, line);
        }
    } else if *no_of_blank_lines{
        let mut index: i32 = 1;
        for line in reader.lines(){
            let line = line.unwrap();
            if line.is_empty(){
                println!();
                continue
            } else{
                println!("     {}	{}", index, line);
                index += 1;
            }

        }
    }
    else{
        for line in reader.lines(){
            let line = line.unwrap();
            println!("{}", line);
        }
    }

    Ok(())
}