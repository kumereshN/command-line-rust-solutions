use clap::{App, Arg};
use std::error::Error;

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
                .help("Input file(s)")
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
        in_file: matches.value_of("infile").unwrap().to_string(),
        out_file: matches.value_of("outfile").map(|s| s.to_string()),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}