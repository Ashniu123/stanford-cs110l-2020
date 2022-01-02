// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.

extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    println!("Welcome to CS110L Hangman!");
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    dbg!(secret_word);

    // Your code here! :)

    let mut i = 0;
    let mut guessed_i: Vec<usize> = Vec::new();
    let mut guessed_c: Vec<char> = Vec::new();

    while i < NUM_INCORRECT_GUESSES && guessed_i.len() != secret_word_chars.len() {
        print!("\nThe word so far is: ");
        for x in 0..secret_word_chars.len() {
            print!(
                "{}",
                if guessed_i.contains(&x) {
                    secret_word_chars.get(x).unwrap()
                } else {
                    &'_'
                }
            );
        }

        print!("\nYou have guessed the following letter:");
        for x in &guessed_c {
            print!("{}", *x);
        }
        println!("\nYou have {} guesses left.", NUM_INCORRECT_GUESSES - i);
        io::stdout().flush().expect("Error flushing stdout.");

        let mut guess = String::new();
        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess.");
        let guess_c = guess.trim().chars().next().unwrap();
        if !guessed_c.contains(&guess_c) {
            guessed_c.push(guess_c);
        } else {
            println!("letter already guessed");
            continue;
        }

        let mut did_guess = false;
        for x in 0..secret_word_chars.len() {
            if *secret_word_chars.get(x).unwrap() == guess_c {
                guessed_i.push(x);
                did_guess = true;
            }
        }

        if !did_guess {
            println!("Sorry, that letter is not in the word");
            i += 1;
        }
    }

    io::stdout().flush().expect("Error flushing stdout.");
    if guessed_i.len() == secret_word_chars.len() {
        println!("Congratulations! You guessed the secret word!");
    } else if i == NUM_INCORRECT_GUESSES {
        println!("Sorry! You ran out of guesses!");
    }
}
