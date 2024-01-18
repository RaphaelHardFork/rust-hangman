use std::io;

pub struct Game {
    score: usize,
    best_score: usize,
    attemps: usize,
    username: String,
}

impl Game {
    pub fn new() -> Self {
        let username = Game::choose_username();
        Self {
            score: 0,
            best_score: 0,
            attemps: 0,
            username,
        }
    }

    pub fn load() -> Self {
        unimplemented!()
    }

    pub fn start(&self) {
        unimplemented!()
    }

    pub fn save(&self) {
        unimplemented!()
    }

    fn choose_username() -> String {
        let mut username: String = String::new();
        println!("Create an username");

        loop {
            match io::stdin().read_line(&mut username) {
                Ok(_) => {
                    username = username.trim().to_string();
                    break;
                }
                Err(e) => {
                    println!("Error in the username prompt");
                    continue;
                }
            }
        }

        println!("Let's go {}", username);
        username

        // io::stdin().read_line(&mut username).expect("Start failed");
        // let ans: char = ans.trim().parse().expect("Failed");
    }
}
