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

use rand::{Rng, thread_rng};

fn main() {

    // How long do we want the password
    const PASSWORD_LEN: usize = 30;

    // Build list of possable characters
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    // Create password
    let mut rng = thread_rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
