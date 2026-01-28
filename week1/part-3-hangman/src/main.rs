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
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let len = secret_word_chars.len();
    let mut words = vec!['-'; len];
    let mut guess_string = String::new();
    let mut guess_last_times = NUM_INCORRECT_GUESSES;
    while guess_last_times > 0 {
        print!("The word so far is ");
        println!("{}", words.iter().collect::<String>());
        print!("You have guessed the following letters:");
        println!("{}", guess_string);
        println!("You have {} guesses left", guess_last_times);
        print!("Please guess a letter:");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");
        let guess_char = match guess.trim().chars().next() {
            Some(c) => c,
            None => continue,
        };
        guess_string.push(guess_char);
        // 遍历 secret_word_chars 进行比对
        let mut found = false;
        for (idx, &c) in secret_word_chars.iter().enumerate() {
            if c == guess_char {
                words[idx] = guess_char;
                found = true;
            }
        }
        if !found {
            guess_last_times -= 1;
        }
        if !words.contains(&'-') {
            println!("Congratulations! You win!");
            return;
        }
    }
    println!("Sorry, you lose! The word was {}", secret_word);
}
