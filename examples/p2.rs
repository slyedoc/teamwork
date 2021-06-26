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
// How do you generate random numbers (using crate rand again)

fn main() {

    // Get arguments

    // Parse arguments

    // Perform Roll
}