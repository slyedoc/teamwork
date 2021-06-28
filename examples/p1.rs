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

use rand::{thread_rng, Rng};

fn main() {
    const PASSWORD_LENGTH_MIN: u8 = 8;
    const PASSWORD_LENGTH_MAX: u8 = 15;
    const CHAR_SET: &[u8] =
        b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjklmnpqrstuvwxyz123456789~!@#$%^&*)(";

    let mut rng = thread_rng();

    let length = rng.gen_range(PASSWORD_LENGTH_MIN..PASSWORD_LENGTH_MAX);

    let mut result = String::from("");

    for _ in 0..length {
        let number = rng.gen_range(0..CHAR_SET.len());
        result.push(CHAR_SET[number] as char);
    }
    println!("{}", result);
}
