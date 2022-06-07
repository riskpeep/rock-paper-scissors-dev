//
// Rock, Paper, Sissors
//
// A game by Robert Kerr

use std::fmt;
use std::io;
use std::str;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

enum RockPaperSissorsGuess {
    Rock,
    Paper,
    Sissors,
}

pub trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}

enum RockPaperSissorsCompare {
    RockCrushesSissors,
    PaperCoversRock,
    SissorsCutPaper,
}

enum RockPaperSissorsResult {
    Win(RockPaperSissorsCompare),
    Loss(RockPaperSissorsCompare),
    Tie,
}

impl Compare<RockPaperSissorsGuess, RockPaperSissorsResult> for RockPaperSissorsGuess{
    fn compare(&self, b: &RockPaperSissorsGuess) -> RockPaperSissorsResult {
        match self {
            RockPaperSissorsGuess::Rock => {
                match b {
                    RockPaperSissorsGuess::Rock    =>
                        RockPaperSissorsResult::Tie,
                    RockPaperSissorsGuess::Paper   =>
                        RockPaperSissorsResult::Loss(RockPaperSissorsCompare::PaperCoversRock),
                    RockPaperSissorsGuess::Sissors =>
                        RockPaperSissorsResult::Win(RockPaperSissorsCompare::RockCrushesSissors),
                }
            }
            RockPaperSissorsGuess::Paper => {
                match b {
                    RockPaperSissorsGuess::Rock    =>
                        RockPaperSissorsResult::Win(RockPaperSissorsCompare::PaperCoversRock),
                    RockPaperSissorsGuess::Paper   =>
                        RockPaperSissorsResult::Tie,
                    RockPaperSissorsGuess::Sissors =>
                        RockPaperSissorsResult::Loss(RockPaperSissorsCompare::SissorsCutPaper),
                }
            }
            RockPaperSissorsGuess::Sissors => {
                match b {
                    RockPaperSissorsGuess::Rock    =>
                        RockPaperSissorsResult::Loss(RockPaperSissorsCompare::RockCrushesSissors),
                    RockPaperSissorsGuess::Paper   =>
                        RockPaperSissorsResult::Win(RockPaperSissorsCompare::SissorsCutPaper),
                    RockPaperSissorsGuess::Sissors =>
                        RockPaperSissorsResult::Tie,
                }
            }
        }
    }
}

#[derive(Debug)]
enum RockPaperSissorsParseError {
    Unknown,
}

impl str::FromStr for RockPaperSissorsGuess {
    type Err = RockPaperSissorsParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" => Ok(RockPaperSissorsGuess::Rock),
            "p" => Ok(RockPaperSissorsGuess::Paper),
            "s" => Ok(RockPaperSissorsGuess::Sissors),
            _   => Err(RockPaperSissorsParseError::Unknown),
        }
    }
}

impl fmt::Display for RockPaperSissorsGuess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperSissorsGuess::Rock    => write!(f, "Rock"),
            RockPaperSissorsGuess::Paper   => write!(f, "Paper"),
            RockPaperSissorsGuess::Sissors => write!(f, "Sissors"),
        }
    }
}

impl Distribution<RockPaperSissorsGuess> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RockPaperSissorsGuess {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => RockPaperSissorsGuess::Rock,
            1 => RockPaperSissorsGuess::Paper,
            2 => RockPaperSissorsGuess::Sissors,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for RockPaperSissorsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperSissorsResult::Win(result) => {
                match result {
                    RockPaperSissorsCompare::RockCrushesSissors => write!(f, "Rock crushes sissors"),
                    RockPaperSissorsCompare::PaperCoversRock => write!(f, "Paper covers rock"),
                    RockPaperSissorsCompare::SissorsCutPaper => write!(f, "Sissors cut paper"),
                }
            },
            RockPaperSissorsResult::Loss(result) => {
                match result {
                    RockPaperSissorsCompare::RockCrushesSissors => write!(f, "Rock crushes sissors"),
                    RockPaperSissorsCompare::PaperCoversRock => write!(f, "Paper covers rock"),
                    RockPaperSissorsCompare::SissorsCutPaper => write!(f, "Sissors cut paper"),
                }
            },
            RockPaperSissorsResult::Tie => write!(f, ""),
        }
    }
}

fn main() {
    println!("Hello, Lets play Rock, Paper, Sissors!");

    println!("Let's play best 3 out of 5 rounds.");

    let mut player_wins = 0;
    let mut comp_wins = 0;

    loop {

        let comp_move: RockPaperSissorsGuess = rand::thread_rng().gen();

        loop {

            let mut player_move = String::new();

            println!("Please input your move.");

            io::stdin()
                .read_line(&mut player_move)
                .expect("Failed to read move");

            let player_move: Result<RockPaperSissorsGuess, RockPaperSissorsParseError>
                = player_move.trim().parse();

            // TODO can we clean up the *.unwrap()'s all over the place?  We DO want the
            // Result so we can catch the error, but once we know it isn't an error, we
            // want to ditch the unwrap and get back to the result.  Seems like there is
            // a better way.
            match player_move {
                Ok(_) => {
                    println!("You chose {}", &(player_move.as_ref().unwrap())); // TODO figure out why this needs as_ref
                    println!("I chose {}", comp_move);
                }
                Err(_) => {
                    println!("That is not a valid guess, try again."); // TODO Figure out how to name the character here.
                    continue
                },
            }

            let result: RockPaperSissorsResult = player_move.unwrap().compare(&comp_move); 

            match result {
                RockPaperSissorsResult::Win(_) => {
                    player_wins += 1;
                    println!("{}", result);
                    println!("You won this round.");
                },
                RockPaperSissorsResult::Tie => println!("Tie..."),
                RockPaperSissorsResult::Loss(_) => {
                    comp_wins += 1;
                    println!("{}", result);
                    println!("You lost this round.");
                },
            }

            break;
        }

        if player_wins == 3 {
            println!("Congratulations, You won the game!");
            break;
        } else if comp_wins == 3 {
            println!("Too bad...You lost the game! Better luck next time.");
            break;
        } else {
            println!("You have {} wins, and I have {} wins.", player_wins, comp_wins);
        }
    }
}
