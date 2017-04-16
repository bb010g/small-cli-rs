extern crate rand;

use std::io::Write;

use rand::{Isaac64Rng, Rand};
use rand::distributions::{IndependentSample, Range};

macro_rules! try_opt { ( $e:expr ) => { match $e { Some(e) => e, None => return None } } }
macro_rules! fmt_try { ( $e:expr) => { match $e { Some(e) => e,
    None => {
        writeln!(std::io::stderr(), "Please provide a valid roll (NdS+C).")
            .expect("Couldn't write to stderr.");
        std::process::exit(1)
    }
}}}

fn main() {
    let roll = fmt_try!(std::env::args().nth(1));
    let (dice, sides, constant) = fmt_try!(parse_roll(&roll));

    let range = Range::new(1, sides + 1);
    let mut rng = Isaac64Rng::rand(&mut rand::thread_rng());
    let mut sum: u64 = constant;
    for _ in 0..dice {
        sum += range.ind_sample(&mut rng);
    }

    println!("{}", sum);
}

fn parse_roll(roll: &str) -> Option<(u64, u64, u64)> {
    let (dice, roll) = roll.split_at(try_opt!(roll.find('d')));
    let (_, roll) = roll.split_at(1);
    let dice = try_opt!(dice.parse::<u64>().ok());

    let (sides, constant) = match roll.find('+') {
        Some(mid) => {
            let (s, c) = roll.split_at(mid);
            (s, Some(c))
        }
        None => (roll, None),
    };
    let constant = constant.map(|c| c.split_at(1).1);
    let sides = try_opt!(sides.parse::<u64>().ok());
    let constant = try_opt!(constant.map_or(Some(0), |c| c.parse::<u64>().ok()));

    Some((dice, sides, constant))
}
