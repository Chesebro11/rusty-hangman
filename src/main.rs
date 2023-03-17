// extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

struct Letter {
    character: char,
    revealed: bool
}

fn main() {
    let selected_word = select_word();
    // split the selected word into individual letters
    let letters = create_letters(&selected_word);

    display_progress(&letters);

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