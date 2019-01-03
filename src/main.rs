use std::io;

#[derive(Copy, Clone)]
enum Choice {
  Rock,
  Paper,
  Scissors,
  Invalid,
}

impl Choice {
  fn rock_paper_scissors(&self, other: Choice) -> Outcome {
    match (*self, other) {
      (Choice::Rock, Choice::Scissors) => Outcome::Win,
      (Choice::Rock, Choice::Paper) => Outcome::Lose,
      (Choice::Paper, Choice::Rock) => Outcome::Win,
      (Choice::Paper, Choice::Scissors) => Outcome::Lose,
      (Choice::Scissors, Choice::Paper) => Outcome::Win,
      (Choice::Scissors, Choice::Rock) => Outcome::Lose,
      _ => Outcome::Draw,
    }
  }
}

enum Outcome {
  Win,
  Lose,
  Draw,
}

fn main() {
  match first_player(player_choice()).map(|input| input.rock_paper_scissors(player_choice())) {
    Err(_) => {
      println!("Not a vaild input");
      std::process::exit(1);
    }

    Ok(outcome) => match outcome {
      Outcome::Win => println!("Player one wins!"),
      Outcome::Lose => println!("Player one loses."),
      Outcome::Draw => println!("Nothing worse than a good draw."),
    },
  }
}

fn first_player(guess: Choice) -> Result<Choice, Choice> {
  match guess {
    Choice::Rock => Ok(Choice::Rock),
    Choice::Paper => Ok(Choice::Paper),
    Choice::Scissors => Ok(Choice::Scissors),
    _ => Err(Choice::Invalid),
  }
}

fn player_choice() -> Choice {
  let mut buffer = String::new();
  io::stdin()
    .read_line(&mut buffer)
    .expect("couldnt get input");
  println!(
    "Player chose: {}",
    choice_to_string(string_to_choice(buffer.clone()))
  );
  string_to_choice(buffer)
}

fn string_to_choice(string: String) -> Choice {
  match string.trim() {
    "rock" => Choice::Rock,
    "paper" => Choice::Paper,
    "scissors" => Choice::Scissors,
    _ => Choice::Invalid,
  }
}

fn choice_to_string(choice: Choice) -> String {
  match choice {
    Choice::Rock => String::from("rock"),
    Choice::Paper => String::from("paper"),
    Choice::Scissors => String::from("scissors"),
    Choice::Invalid => String::from("Invalid choice"),
  }
}