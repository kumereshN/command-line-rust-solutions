use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config{
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize
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

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {

    let mut buf = String::new();

    let num_bytes = file.read_line(&mut buf).unwrap();
    let num_lines = buf.lines().count();
    let num_words = { if buf.is_empty(){
                                0
                        }
                        else{
                            buf.split_whitespace().count()}
                        };
    let num_chars = buf.chars().count();

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count(){
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let r = count(file).unwrap();
                println!("{:>7} {:>7} {:>7} {}", r.num_lines, r.num_words, r.num_bytes, filename);
            }
        }
    }
    Ok(())
}