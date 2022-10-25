use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    collections::HashSet,
};
use std::io::Read;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config{
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Kumeresh <kumeresh6@hotmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("infile")
                .value_name("INFILES")
                .help("Input file")
                .default_value("-")
        )
        .arg(
            Arg::with_name("outfile")
                .value_name("OUTFILES")
                .help("Output filename")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .short("c")
                .long("count")
                .help("Shows the count of unique chars appearing in the file")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config{
        in_file: matches.value_of("infile").map(String::from).unwrap(),
        out_file: matches.value_of("outfile").map(|s| s.to_string()),
        count: matches.is_present("count"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut seen: HashSet<String> = HashSet::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        // From here, write your code
        seen.insert(line.strip_suffix('\n').unwrap().to_string());
        print!("{}", line);
        line.clear();
    }
    print!("{:?}", seen);
    Ok(())
}