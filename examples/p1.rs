// Dice rolling
//
// In alot of games people list the odds or stats of something as a dice roll in the following format:
//
//  (Number of dice)d(number of sides each dice has)+(some constant value)
//
// Example: sword does 2d6+2 dmg

// I would like you to create me a commandline tool that can do these for me, I want something like:

//$roll 2d12+2
//16

// Possible inputs: 1d3, 2d6, 1d12+12


// I would like you to create your own repo, you need the practice with git.

// 1. Create a new cargo program, name it roll
// 2. Create github repo and name it roll
// 3. Follow the instructions on github for connecting them.

// Things to think about

// How do you read a command line arguments?
//      You will need std::env::args(), look it up on docs.rs
//
// How do you split a string up, we will be doing alot of this later
//      "3d12".to_string().split("d").collect::<Vec<&str>>();
//      "3d12+2".to_string().split(&['d', '+'][..]).collect::<Vec<&str>>();
//
//  How do you covert string to i32
//        str.parse::<i32>()
//
// What is a &str and String, how are they deferent?
//
// How do you generate random numbers (using crate rand)
//      use rand::{Rng, thread_rng};
//      let mut rng =  thread_rng();
//      let number = rng.gen_range(0..=10)


// Other Improvements
// - Handle more than one argument for dice

use rand::{Rng, thread_rng};
use regex::Regex;

fn _main_old() {

    for pattern in std::env::args().skip(1) {
        let parts = pattern.split(&['d', '+'][..]).collect::<Vec<&str>>();
        let mut count = 1;
        let mut sides = 1;
        let mut constant = 0;

        if parts.len() == 3 {
            constant = parse_or_default(parts[2], constant);
        }
        if parts.len() >= 2 {
            sides = parse_or_default(parts[1], sides)
        }
        if parts.len() >= 1 {
           count = parse_or_default(parts[0], count);
        }
        roll(sides, count, constant);
    }
}

fn main() {
    for pattern in std::env::args().skip(1) {
        let re = Regex::new(r"(?P<count>\d+)*d(?P<sides>\d+)[+]*(?P<constant>\d+)*").unwrap();
        for caps in re.captures_iter(&*pattern) {
            let count = parse_or_default( &caps["count"], 1);
            let sides = parse_or_default( &caps["sides"], 1);
            let constant = parse_or_default( &caps["constant"], 0);
            roll(sides, count, constant);
        }
    }
}

fn parse_or_default(str: &str, default: i32) -> i32 {
    match str.parse::<i32>() {
        Ok(x) => x,
        Err(_) => default,
    }
}

fn roll(sides: i32, count: i32, constant: i32) {
    let mut rng =  thread_rng();
    let mut total = constant;
    for _ in 0..count {
        total += rng.gen_range(0..=sides)
    }
    println!("{}d{}+{} = {} ", count, sides, constant,  total);
}
