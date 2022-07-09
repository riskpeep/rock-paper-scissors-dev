//
// Rock, Paper, Scissors
//
// A game by Robert Kerr

use std::fmt;
use std::io;
use std::str;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

enum RockPaperScissorsGuess {
    Rock,
    Paper,
    Scissors,
}

pub trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}

enum RockPaperScissorsCompare {
    RockCrushesScissors,
    PaperCoversRock,
    ScissorsCutPaper,
}

enum RockPaperScissorsResult {
    Win(RockPaperScissorsCompare),
    Loss(RockPaperScissorsCompare),
    Tie,
}

impl Compare<RockPaperScissorsGuess, RockPaperScissorsResult> for RockPaperScissorsGuess{
    fn compare(&self, b: &RockPaperScissorsGuess) -> RockPaperScissorsResult {
        match self {
            RockPaperScissorsGuess::Rock => {
                match b {
                    RockPaperScissorsGuess::Rock    =>
                        RockPaperScissorsResult::Tie,
                    RockPaperScissorsGuess::Paper   =>
                        RockPaperScissorsResult::Loss(RockPaperScissorsCompare::PaperCoversRock),
                    RockPaperScissorsGuess::Scissors =>
                        RockPaperScissorsResult::Win(RockPaperScissorsCompare::RockCrushesScissors),
                }
            }
            RockPaperScissorsGuess::Paper => {
                match b {
                    RockPaperScissorsGuess::Rock    =>
                        RockPaperScissorsResult::Win(RockPaperScissorsCompare::PaperCoversRock),
                    RockPaperScissorsGuess::Paper   =>
                        RockPaperScissorsResult::Tie,
                    RockPaperScissorsGuess::Scissors =>
                        RockPaperScissorsResult::Loss(RockPaperScissorsCompare::ScissorsCutPaper),
                }
            }
            RockPaperScissorsGuess::Scissors => {
                match b {
                    RockPaperScissorsGuess::Rock    =>
                        RockPaperScissorsResult::Loss(RockPaperScissorsCompare::RockCrushesScissors),
                    RockPaperScissorsGuess::Paper   =>
                        RockPaperScissorsResult::Win(RockPaperScissorsCompare::ScissorsCutPaper),
                    RockPaperScissorsGuess::Scissors =>
                        RockPaperScissorsResult::Tie,
                }
            }
        }
    }
}

#[derive(Debug)]
enum RockPaperScissorsParseError {
    Unknown(String),
}

impl str::FromStr for RockPaperScissorsGuess {
    type Err = RockPaperScissorsParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" | "rock"    => Ok(RockPaperScissorsGuess::Rock),
            "p" | "paper"   => Ok(RockPaperScissorsGuess::Paper),
            "s" | "scissors" => Ok(RockPaperScissorsGuess::Scissors),
            _   => Err(RockPaperScissorsParseError::Unknown(s.to_string())),
        }
    }
}

impl fmt::Display for RockPaperScissorsGuess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperScissorsGuess::Rock    => write!(f, "Rock"),
            RockPaperScissorsGuess::Paper   => write!(f, "Paper"),
            RockPaperScissorsGuess::Scissors => write!(f, "Scissors"),
        }
    }
}

impl Distribution<RockPaperScissorsGuess> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RockPaperScissorsGuess {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => RockPaperScissorsGuess::Rock,
            1 => RockPaperScissorsGuess::Paper,
            2 => RockPaperScissorsGuess::Scissors,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for RockPaperScissorsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperScissorsResult::Win(result) => {
                match result {
                    RockPaperScissorsCompare::RockCrushesScissors => write!(f, "Rock crushes scissors"),
                    RockPaperScissorsCompare::PaperCoversRock => write!(f, "Paper covers rock"),
                    RockPaperScissorsCompare::ScissorsCutPaper => write!(f, "Scissors cut paper"),
                }
            },
            RockPaperScissorsResult::Loss(result) => {
                match result {
                    RockPaperScissorsCompare::RockCrushesScissors => write!(f, "Rock crushes scissors"),
                    RockPaperScissorsCompare::PaperCoversRock => write!(f, "Paper covers rock"),
                    RockPaperScissorsCompare::ScissorsCutPaper => write!(f, "Scissors cut paper"),
                }
            },
            RockPaperScissorsResult::Tie => write!(f, ""),
        }
    }
}

fn main() {
    println!("Hello, Lets play Rock, Paper, Scissors!");

    println!("Let's play best 3 out of 5 rounds.");

    let mut player_wins = 0;
    let mut comp_wins = 0;
    let mut quit = false;

    'game: loop {

        let comp_move: RockPaperScissorsGuess = rand::thread_rng().gen();

        loop {

            let mut player_move = String::new();

            println!("Please select (r)ock, (p)aper, or (s)issors:");

            io::stdin()
                .read_line(&mut player_move)
                .expect("Failed to read move");

            let player_move: Result<RockPaperScissorsGuess, RockPaperScissorsParseError>
                = player_move.trim().parse();

            // TODO can we clean up the *.unwrap()'s all over the place?  We DO want the
            // Result so we can catch the error, but once we know it isn't an error, we
            // want to ditch the unwrap and get back to the result.  Seems like there is
            // a better way.
            match player_move {
                Ok(_) => {
                    println!("");
                    println!("You chose {}", &(player_move.as_ref().unwrap())); // TODO figure out why this needs as_ref
                    println!("I chose {}", comp_move);
                }
                Err(RockPaperScissorsParseError::Unknown(s)) => {
                    match &s[..] {
                        "q" | "quit" => {
                            println!("Quit? Okay.");
                            quit = true;
                            break 'game;
                        },
                        _            => {
                            println!("\"{}\" is not a valid guess, try again.\n",s); // TODO Figure out how to name the character here.
                            continue
                        },
                    }
                },
            }

            let result: RockPaperScissorsResult = player_move.unwrap().compare(&comp_move); 

            match result {
                RockPaperScissorsResult::Win(_) => {
                    player_wins += 1;
                    println!("{}", result);
                    println!("You won this round.");
                },
                RockPaperScissorsResult::Tie => println!("Tie..."),
                RockPaperScissorsResult::Loss(_) => {
                    comp_wins += 1;
                    println!("{}", result);
                    println!("You lost this round.");
                },
            }

            break;
        }

        println!("");
        if player_wins == 3 {
            println!("Congratulations, You won the game!\n");
            break;
        } else if comp_wins == 3 {
            println!("Too bad...You lost the game! Better luck next time.\n");
            break;
        } else {
            println!("You have {} wins, and I have {} wins.\n", player_wins, comp_wins);
        }
    }

    if quit == true {
        println!("Well... thanks for playing.  Sorry you had to leave so soon.");
    }
}
