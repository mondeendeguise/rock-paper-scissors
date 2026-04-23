use std::fmt;
use std::io;

use rand;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum GameError {
    InvalidChoice,
}

enum Outcome {
    Tie,
    Player1Wins,
    Player2Wins,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hand::Rock => write!(f, "Rock"),
            Hand::Paper => write!(f, "Paper"),
            Hand::Scissors => write!(f, "Scissors"),
        }
    }
}

impl rand::distr::Distribution<Hand> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized + rand::RngExt>(&self, rng: &mut R) -> Hand {
        match rng.random_range(0..=2) {
            0 => Hand::Rock,
            1 => Hand::Paper,
            _ => Hand::Scissors,
        }
    }
}

fn evaluate_hand(p1: &Hand, p2: &Hand) -> Outcome {
    match p1 {
        Hand::Rock => match p2 {
            Hand::Rock => Outcome::Tie,
            Hand::Paper => Outcome::Player2Wins,
            Hand::Scissors => Outcome::Player1Wins,
        },
        Hand::Paper => match p2 {
            Hand::Rock => Outcome::Player1Wins,
            Hand::Paper => Outcome::Tie,
            Hand::Scissors => Outcome::Player2Wins,
        },
        Hand::Scissors => match p2 {
            Hand::Rock => Outcome::Player2Wins,
            Hand::Paper => Outcome::Player1Wins,
            Hand::Scissors => Outcome::Tie,
        },
    }
}

fn determine_choice(player_input: &str) -> Result<Hand, GameError> {
    match player_input {
        "r" | "rock" | "1" => Ok(Hand::Rock),
        "p" | "paper" | "2" => Ok(Hand::Paper),
        "s" | "scissors" | "3" => Ok(Hand::Scissors),
        _ => Err(GameError::InvalidChoice),
    }
}

fn main() -> io::Result<()> {
    loop {
        println!("Rock Paper Scissors");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        buffer = buffer.trim().to_lowercase();

        if buffer == "exit" || buffer == "quit" || buffer == "leave" {
            break;
        }

        let p1: Hand = match determine_choice(&buffer) {
            Ok(value) => value,
            Err(_err) => continue,
        };

        let p2: Hand = rand::random();

        let winner: Outcome = evaluate_hand(&p1, &p2);

        print!("{} vs {} - ", p1, p2);
        match winner {
            Outcome::Tie => println!("Tie"),
            Outcome::Player1Wins => println!("You win"),
            Outcome::Player2Wins => println!("You lose"),
        }
    }

    Ok(())
}
