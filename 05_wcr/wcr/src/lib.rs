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
                .short("n")
                .long("lines")
                .help("Number of lines to display")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .short("w")
                .long("words")
                .help("Number of words to display")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .help("Number of bytes to display")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .short("m")
                .long("chars")
                .help("Number of chars to display")
                .takes_value(false)
                .conflicts_with("bytes")
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false){
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}