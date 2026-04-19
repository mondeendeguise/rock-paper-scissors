use std::fmt;
use std::io;

use rand;

enum Hand {
    Rock,
    Paper,
    Scissors,
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

fn evaluate_hand(p1: &Hand, p2: &Hand) -> i32 {
    match p1 {
        Hand::Rock => match p2 {
            Hand::Rock => 0,
            Hand::Paper => 2,
            Hand::Scissors => 1,
        },
        Hand::Paper => match p2 {
            Hand::Rock => 1,
            Hand::Paper => 0,
            Hand::Scissors => 2,
        },
        Hand::Scissors => match p2 {
            Hand::Rock => 2,
            Hand::Paper => 1,
            Hand::Scissors => 0,
        },
    }
}

fn main() -> io::Result<()> {

    loop {
        println!("Rock Paper Scissors");

        let p1: Hand;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        buffer = buffer.trim().to_lowercase();

        if buffer == "exit" || buffer == "quit" || buffer == "leave" {
            break;
        }

        else if buffer == "r" || buffer == "rock" || buffer == "1" {
            p1 = Hand::Rock;
        }

        else if buffer == "p" || buffer == "paper" || buffer == "2" {
            p1 = Hand::Paper;
        }

        else if buffer == "s" || buffer == "scissors" || buffer == "3" {
            p1 = Hand::Scissors;
        }

        else {
            continue;
        }

        let p2: Hand = rand::random();

        let winner: i32 = evaluate_hand(&p1, &p2);

        print!("{} vs {} - ", p1, p2);
        match winner {
            0 => println!("Tie"),
            1 => println!("Player 1 wins"),
            2 => println!("Player 2 wins"),
            _ => println!("Undefined..."),
        }
    }

    Ok(())
}
