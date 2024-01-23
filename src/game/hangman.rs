use crate::utils::cli::hangman;

pub struct Hangman {
    pub attemps: usize,
    pub progress: usize,
}

impl Hangman {
    pub fn new() -> Self {
        Self {
            progress: 0,
            attemps: 0,
        }
    }

    pub fn print_hangman(&self) {
        println!("{}", hangman(&HANGMAN_PRINT[self.attemps].to_string()))
    }

    pub fn progress(&mut self) {
        self.progress += 1
    }
    pub fn attemp(&mut self) {
        self.attemps += 1
    }
}

pub const HANGMAN_PRINT: [&str; 7] = [
    r#"

            
            
            
            
            
==========
    "#,
    r#"
        +  
        |  
        |  
        |  
        |  
        |  
==========
    "#,
    r#"
    +---+  
        |  
        |  
        |  
        |  
        |  
==========
    "#,
    r#"
    +---+  
    |   |  
    O   |  
        |  
        |  
        |  
==========
    "#,
    r#"
    +---+  
    |   |  
    O   |  
   /|\  |  
        |  
        |  
==========
    "#,
    r#"
    +---+  
    |   |  
    O   |  
   /|\  |  
   / \  |  
        |  
==========
    "#,
    r#"
    +---+  
    '   |  
        |  
   \O/  |  
    |   |  
   / \  |  
==========
    "#,
];
