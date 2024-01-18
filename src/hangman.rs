const HANGMAN: [String; 7] = [
    "          \n          \n          \n          \n          \n          \n=========="
        .to_string(),
    "       +  \n       |  \n       |  \n       |  \n       |  \n       |  \n=========="
        .to_string(),
    "   +---+  \n       |  \n       |  \n       |  \n       |  \n       |  \n=========="
        .to_string(),
    "   +---+  \n   |   |  \n   O   |  \n       |  \n       |  \n       |  \n=========="
        .to_string(),
    "   +---+  \n   |   |  \n   O   |  \n  /|\\  |  \n       |  \n       |  \n=========="
        .to_string(),
    "   +---+  \n   |   |  \n   O   |  \n  /|\\  |  \n  / \\  |  \n       |  \n=========="
        .to_string(),
    "   +---+  \n   '   |  \n       |  \n  \\O/  |  \n   |   |  \n  / \\  |  \n=========="
        .to_string(),
];

pub struct Hangman {
    actual_index: usize,
}

impl Hangman {
    pub fn new() -> Self {
        Self { actual_index: 0 }
    }

    pub fn increment_form(&self) {
        self.actual_index += 1;
    }

    pub fn print_hangman(&self, index: usize) {
        {
            println!("{}", HANGMAN[self.actual_index])
        }
    }
}
