//! Play rock 🪨, paper 🧻, scissors ✂️ against the computer
//!
//! Demonstrates uses of `enum`
//!
//! # Examples
//! ```
//! cargo run -q
//! Let's play! 🪨🧻✂️
//! What do you throw?
//! rock
//! You threw... Rock 🪨
//! Computer throws Rock 🪨
//! Result: It's a tie! 👔
//! Press ENTER to play again, or anything else to quit
//!
//! Let's play! 🪨🧻✂️
//! What do you throw?
//! paper
//! You threw... Paper 🧻
//! Computer throws Scissors ✂️
//! Result: Sorry, you lose 😿
//! Press ENTER to play again, or anything else to quit
//!
//! Let's play! 🪨🧻✂️
//! What do you throw?
//! scissors
//! You threw... Scissors ✂️
//! Computer throws Paper 🧻
//! Result: You win, congrats! 🎉
//! Press ENTER to play again, or anything else to quit
//! ```
//!
//! A single argument can also be passed to play just once
//!
//! ```
//! cargo run -q -- rock
//! Let's play 🪨🧻✂️!
//! You threw... Rock 🪨
//! Computer throws Rock 🪨
//! Result: It's a tie! 👔
//! ```
//!

use clap::Parser;
use colored::*;
use rand::prelude::*;
use std::cmp::*;
use std::fmt;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "rps")]
#[command(about = "Rock, Paper, Scissors Game", long_about = None)]
struct Args {
    #[arg()]
    throw: Option<Throw>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
enum Throw {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

impl Ord for Throw {
    fn cmp(&self, other: &Self) -> Ordering {
        use Throw::*;
        match (*self, *other) {
            (Rock, Paper) => Ordering::Less,
            (Rock, Scissors) => Ordering::Greater,
            (Rock, Rock) => Ordering::Equal,
            (Paper, Rock) => Ordering::Greater,
            (Paper, Scissors) => Ordering::Less,
            (Paper, Paper) => Ordering::Equal,
            (Scissors, Rock) => Ordering::Less,
            (Scissors, Paper) => Ordering::Greater,
            (Scissors, Scissors) => Ordering::Equal,
            (_, _) => Ordering::Equal,
        }
    }
}

impl fmt::Display for Throw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            Self::Rock => "Rock 🪨",
            Self::Paper => "Paper 🧻",
            Self::Scissors => "Scissors ✂️",
            Self::Invalid => "Invalid Input 🐛",
        };
        write!(f, "{}", res)
    }
}

#[derive(Debug)]
enum GameResult {
    YouWin,
    YouLose,
    Tie,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            Self::YouWin => "You win, congrats! 🎉",
            Self::YouLose => "Sorry, you lose 😿",
            Self::Tie => "It's a tie! 👔",
        };
        write!(f, "{}", res)
    }
}

impl FromStr for Throw {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match &s.to_lowercase().trim_end()[..] {
            "rock" => Throw::Rock,
            "paper" => Throw::Paper,
            "scissors" => Throw::Scissors,
            _ => Throw::Invalid,
        };
        Ok(res)
    }
}

impl Throw {
    fn user(self) -> Self {
        println!("{} {}", "You threw...".purple().bold(), self);
        if matches!(self, Throw::Invalid) {
            eprintln!("{}", "I'm not playing with you now.".red().bold());
            std::process::exit(1)
        }
        self
    }

    fn computer() -> Self {
        let mut rng = rand::thread_rng();
        let r = [Throw::Rock, Throw::Paper, Throw::Scissors]
            .choose(&mut rng)
            .clone();
        println!("{} {}", "Computer throws".purple().bold(), r.unwrap());
        r.unwrap().to_owned()
    }
}

fn play(a: Throw, b: Throw) -> GameResult {
    let result = match a.cmp(&b) {
        Ordering::Equal => GameResult::Tie,
        Ordering::Greater => GameResult::YouWin,
        Ordering::Less => GameResult::YouLose,
    };

    println!("{} {}", "Result:".purple().bold(), result);

    result
}

fn lets_play() {
    print!("\x1B[2J\x1B[1;1H"); // clear screen
    println!(
        "Let's {}{}{}{} 🪨🧻✂️!",
        "p".red().bold(),
        "l".green().bold(),
        "a".yellow().bold(),
        "y".blue().bold()
    );
}

fn main() {
    let args = Args::parse();

    match args.throw {
        Some(val) => {
            lets_play();
            let user = val.user();
            let computer = Throw::computer();
            play(user, computer);
            std::process::exit(0)
        }
        None => loop {
            lets_play();
            println!("{}", "What do you throw?".purple().bold());
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).ok();
            let user = Throw::from_str(user_input.as_str()).unwrap().user();
            let computer = Throw::computer();
            play(user, computer);
            println!(
                "{}",
                "Press ENTER to play again, or anything else to quit"
                    .green()
                    .bold()
            );
            user_input = "".to_string();
            std::io::stdin().read_line(&mut user_input).ok();
            match &user_input.to_lowercase().trim_end()[..] {
                "" => (),
                _ => std::process::exit(0),
            }
        },
    };
}
