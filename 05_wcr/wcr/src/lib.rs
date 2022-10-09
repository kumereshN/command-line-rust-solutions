use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config{
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Kumeresh <kumeresh6@hotmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Number of lines to display")
                .takes_value(false)
                .conflicts_with_all(&["words", "bytes", "chars"])
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .help("Number of words to display")
                .takes_value(false)
                .conflicts_with_all(&["lines", "bytes", "chars"])
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Number of bytes to display")
                .takes_value(false)
                .conflicts_with_all(&["lines", "words", "chars"])
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Number of chars to display")
                .takes_value(false)
                .conflicts_with_all(&["lines", "bytes", "words"])
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.is_present("lines"),
        words: matches.is_present("words"),
        bytes: matches.is_present("bytes"),
        chars: matches.is_present("chars")
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}