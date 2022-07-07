extern crate rand;
use rand::Rng;
use std::{io, ops::Add};

fn main() {
    let word: String = get_random_word().to_uppercase();
    let mut ans: String = String::new();

    println!("Welcome to Hangman game!\n\nThe goal is to guess the word in 5 attempt\n\n");
    println!("Are you ready? (y/n)");
    io::stdin().read_line(&mut ans).expect("Start failed");
    let ans: char = ans.trim().parse().expect("Failed");
    if ans == 'n' {
        println!("Ok, good bye");
    } else if ans != 'y' {
        println!("{} is not an answer...", ans);
    } else {
        println!("\n\nLet's go!!\n\n");

        let mut guess = String::from("_ ".repeat(word.len()));
        let mut attempts = 0;
        let mut index = 0;

        // game core
        loop {
            let mut letter: String = String::new();
            print_hangman(attempts);
            println!(
                "\nThe word to guess: {}\nType your letter:\n(attempts {}/6)",
                guess,
                attempts + 1
            );
            letter.clear();
            io::stdin().read_line(&mut letter).expect("Failed");
            letter = letter.trim().to_string();

            // check input
            if letter.len() == 0 {
                println!("Please input something");
                continue;
            } else if letter.len() > 1 {
                println!("Please input only one letter");
                continue;
            }

            if letter.to_uppercase() == &word[index..index + 1] {
                guess = if index == 0 {
                    String::from(&word[0..index + 1])
                        .add(" ")
                        .add(&guess[2..guess.len() - index * 2])
                } else {
                    String::from(&guess[0..index * 2])
                        .add(&word[index..index + 1])
                        .add(" ")
                        .add(&guess[(index * 2) + 2..])
                };
                println!("\n{} was the good letter", letter.to_uppercase());
                index += 1;
            } else {
                println!("{} is the wrong letter", letter.to_uppercase());
                attempts += 1;
            }

            // is the game winned
            if index == word.len() {
                println!("You find the word: {}", word);
                println!("You win the game! Congrat!");
                print_hangman(6);
                break;
            }

            // if the guess fail
            if attempts >= 5 {
                println!("Sorry you fail...");
                println!("The word was {}", word);
                print_hangman(attempts);
                break;
            }
        }
    }
}

fn get_random_word() -> String {
    const WORDS: [&str; 37] = [
        "word", "time", "number", "way", "people", "water", "day", "part", "sound", "work",
        "place", "year", "back", "thing", "name", "sentence", "man", "line", "boy", "farm", "end",
        "men", "land", "home", "hand", "picture", "air", "animal", "house", "page", "letter",
        "point", "mother", "answer", "America", "world", "food",
    ];

    String::from(WORDS[rand::thread_rng().gen_range(0..WORDS.len())])
}

fn print_hangman(index: usize) {
    const HANGMAN: [&str; 7] = [
        "          \n          \n          \n          \n          \n          \n==========",
        "       +  \n       |  \n       |  \n       |  \n       |  \n       |  \n==========",
        "   +---+  \n       |  \n       |  \n       |  \n       |  \n       |  \n==========",
        "   +---+  \n   |   |  \n   O   |  \n       |  \n       |  \n       |  \n==========",
        "   +---+  \n   |   |  \n   O   |  \n  /|\\  |  \n       |  \n       |  \n==========",
        "   +---+  \n   |   |  \n   O   |  \n  /|\\  |  \n  / \\  |  \n       |  \n==========",
        "   +---+  \n   '   |  \n       |  \n  \\O/  |  \n   |   |  \n  / \\  |  \n==========",
    ];

    println!("{}", HANGMAN[index])
}
