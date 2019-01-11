use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn rock_paper_scissors(&self, other: Choice) -> Outcome {
        match (self, other) {
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Rock, Choice::Paper) => Outcome::Lose,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Paper, Choice::Scissors) => Outcome::Lose,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Scissors, Choice::Rock) => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }
    fn string_to_choice(string: String) -> Result<Choice, GameError> {
        match string.trim() {
            "rock" => Ok(Choice::Rock),
            "paper" => Ok(Choice::Paper),
            "scissors" => Ok(Choice::Scissors),
            _ => Err(GameError::InvalidChoice),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Choice::Rock => write!(f, "rock"),
            Choice::Paper => write!(f, "paper"),
            Choice::Scissors => write!(f, "scissors"),
        }
    }
}

#[derive(Debug, Clone)]
enum GameError {
    InvalidChoice,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::InvalidChoice => write!(f, "Invalid Choice"),
        }
    }
}

impl error::Error for GameError {
    fn description(&self) -> &str {
        match self {
            GameError::InvalidChoice => "Invalid Choice",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Outcome::Win => write!(f, "Player one wins!"),
            Outcome::Lose => write!(f, "Player one loses!"),
            Outcome::Draw => write!(f, "Nothing worse than a good draw."),
        }
    }
}

fn main() {
    println!("Player One's choice:");
    let first_choice =
        player_choice().expect("Please make a valid choice: rock, paper, or scissors");
    println!("Player Two's choice:");
    let second_choice =
        player_choice().expect("Please make a valid choice: rock, paper, or scissors");
    println!("{} vs {}", first_choice, second_choice);
    println!("{}", first_choice.rock_paper_scissors(second_choice));
}

fn player_choice() -> Result<Choice, GameError> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("couldnt get input");
    Choice::string_to_choice(buffer)
}
