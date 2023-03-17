// extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;


const ALLOWED_ATTEMPTS: u8 = 5;
struct Letter {
    character: char,
    revealed: bool
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    // split the selected word into individual letters
    let mut letters = create_letters(&selected_word);

    loop {
        println!("You have {} turns left.", turns_left);

        display_progress(&letters);

        println!("Please enter a letter to guess:");
        let user_char = read_user_input_character();

        // Exit if user enters an asterisk
        if user_char == '*' {
            break;
        }

        let mut at_least_one_revealed = false;

        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }
        if !at_least_one_revealed{
            turns_left -= 1;
        }
    }

    println!("The selected word is {}", selected_word);
}

fn select_word() -> String {
    // Open File
    let mut file = File::open("words.txt")
        .expect("Could not open file!");

    // load file contents
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("An error has occured while reading the file!");

    // Get individual words
    let available_words: Vec<&str> = file_contents.trim().split(",").collect();

    // Generate random index
    let random_index = rand::thread_rng().gen_range(0..available_words.len());

    return String::from(available_words[random_index]);
}


fn create_letters(word: &String) -> Vec<Letter> {
    // Create an empty vector to store letters
    let mut letters: Vec<Letter> = Vec::new();

// Wrap each character in a Letter struct
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;
}
// Display appropriate character (Either a letter or an _)
fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:"); // _ _ A _ Y _ S

    for letter in letters {
        display_string.push(' ');

        if letter.revealed{
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    // Get user input
    match io::stdin().read_line(&mut user_input) {
        // if succesful ->
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        // if not succesful
        Err (_) => { return '*'; }
    }
}