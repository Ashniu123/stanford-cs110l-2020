use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| format!("{}\n", l.unwrap()))
        .collect();
    Ok(lines)
}

fn read_words(lines: &Vec<String>) -> Vec<String> {
    lines
        .into_iter()
        .flat_map(|l| l.split_whitespace())
        .fold(Vec::new(), |mut acc, w| {
            acc.push(String::from(w));
            acc
        })
}

fn read_chars(lines: &Vec<String>) -> Vec<String> {
    lines
        .into_iter()
        .flat_map(|l| l.chars())
        .fold(Vec::new(), |mut acc, c| {
            acc.push(String::from(c));
            acc
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];

    let lines = read_file_lines(filename).unwrap();
    let words = read_words(&lines);
    let chars = read_chars(&lines); // reason for having it on basis of lines and not words is that ' ' and '\n'
    println!(
        "{} {} {} {}",
        lines.len(),
        words.len(),
        chars.len(),
        filename
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("simple.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], "ab\n");
    }

    #[test]
    fn test_read_words() {
        let lines = read_file_lines(&String::from("simple.txt")).unwrap();
        let words = read_words(&lines);
        assert_eq!(words.len(), 6);
        assert_eq!(words[0], "ab");
    }

    #[test]
    fn test_read_chars() {
        let lines = read_file_lines(&String::from("simple.txt")).unwrap();
        let chars = read_chars(&lines);
        assert_eq!(chars.len(), 17);
        assert_eq!(chars[0], "a");
    }
}
