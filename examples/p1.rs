// Password Generate
//
// Its common to need to create new passwords for things
//
// I would like you to create me a commandline tool that can do these for me, I want something like:

//$pass-gen
//
// Things to think about

// What characters do you want in passwords you have to type, uppercase, and lowercase.
// how would you store them

// How do you generate random numbers (using crate rand)
//      use rand::{Rng, thread_rng};
//      let mut rng =  thread_rng();
//      let number = rng.gen_range(0..=10)

use rand::Rng;

fn main() {

    let mut rng = rand::thread_rng();
    let length = rng.gen_range(10..15);
    let letters = [ 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k' ];

    // using iterators
    let password_iter = (0..length)
        .map(|_| {
            let random_index = rng.gen_range(0..letters.len());
            letters[random_index]
        }).collect::<String>();

    // Same as above, but using a for loop instead
    let mut password_for = "".to_string();
    for _ in 0..length {
        let random_index = rng.gen_range( 0..letters.len() );
        password_for.push(letters[random_index]);
    }

    println!("iter = {:?}", password_iter);
    println!("for = {:?}", password_for);
}

