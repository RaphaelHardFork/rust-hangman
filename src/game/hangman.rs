use crate::utils::cli::hangman;

// region:			--- Constants

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
// endregion:		--- Constants

#[cfg_attr(test, derive(Debug, PartialEq))]
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

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    #[test]
    fn test_create_hangman_ok() -> Result<()> {
        let fx_hangman = Hangman {
            attemps: 0,
            progress: 0,
        };
        let res = Hangman::new();

        assert_eq!(res, fx_hangman);
        Ok(())
    }

    #[test]
    fn test_progress_n_attemps_ok() -> Result<()> {
        let fx_hangman = Hangman {
            attemps: 2,
            progress: 2,
        };
        let mut hangman = Hangman::new();
        hangman.attemp();
        hangman.attemp();
        hangman.progress();
        hangman.progress();

        assert_eq!(hangman, fx_hangman);
        Ok(())
    }
}
// endregion:		--- Tests
