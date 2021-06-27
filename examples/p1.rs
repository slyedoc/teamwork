// Password Generate
//
// Its common to need to create new passwords for things
//
// I would like you to create me a commandline tool that can do these for me, I want something like:

//$roll 2d12+2
//
// Things to think about

// How do you generate random numbers (using crust apirate rand)
//      use rand::{Rng, thread_rng};
//      let mut rng =  thread_rng();
//      let number = rng.gen_range(0..=10)

fn main() {

    let character = vec![];

    for x in 0..26 {
        character.push( char::to_digit('a').unwrap()  + x );
    }

    println!("character: {}", character);


    // How long do we want the password

    // Build list of possable characters

    // Create password

}
