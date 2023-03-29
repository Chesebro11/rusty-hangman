use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;

const ALLOWED_ATTEMPTS: u8 = 5;

pub struct Letter {
    character: char,
    revealed: bool,
}

pub fn select_word() -> String {
    // Open File
    let mut file = File::open("words.txt").expect("Could not open file!");

    // Load file contents
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("An error has occured while reading the file!");

    // Get available words, seperated by commas
    let available_words: Vec<&str> = file_contents.trim().split(",").collect();

    // Generate random index
    let random_index = rand::thread_rng().gen_range(0..available_words.len());

    return String::from(available_words[random_index]);
}

pub fn create_letters(word: &String) -> Vec<Letter> {
    // Create an empty vector to store letters
    let mut letters: Vec<Letter> = Vec::new();

    // Wrap each character in a letter struct
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false,
        });
    }

    return letters;
}

pub fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:"); // _ _ A _ S _ B

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_')
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

// Figure out how to limit user input to ONLY 1 letter at a time (or '*') if not acceptable print a message

pub fn read_user_input_character() -> char {
    let mut user_input = String::new();
    // Get user input
    match io::stdin().read_line(&mut user_input) {
        // if succesful
        Ok(_) => match user_input.chars().next() {
            Some(c) => {
                return c;
            }
            None => {
                return '*';
            }
        },
        Err(_) => {
            return '*';
        }
    }
}

// create win condition for figuring out the word, create intro message, sort words by difficulty, add minimal UI
pub fn game_loop() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    // split the selected word into individual letters
    let mut letters = create_letters(&selected_word);

    loop {
        println!("You have {} turns left.", turns_left);

        display_progress(&letters);

        if !letters.iter().any(|letter| !letter.revealed) {
            println!("Congratulations, you won!");
            break;
        }

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
        if !at_least_one_revealed {
            turns_left -= 1;
        }
        if turns_left <= 0 {
            println!("You lost :(");
            break;
        }
    }

    println!("The selected word is {}", selected_word);
}

pub fn start_menu() {
    loop {
        println!("Please select an option:");
        println!("1. Start Game");
        println!("2. Quit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => game_loop(),
            "2" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid input, please try again"),
        }
    }
}
