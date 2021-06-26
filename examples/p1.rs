// Password Generate
//
// Its common to need to create new passwords for things
//
// I would like you to create me a commandline tool that can do these for me, I want something like:

//$roll 2d12+2
//
// Things to think about

// How do you generate random numbers (using crate rand)
//      use rand::{Rng, thread_rng};
//      let mut rng =  thread_rng();
//      let number = rng.gen_range(0..=10)

fn main() {

    // How long do we want the password
    const PASSWORD_LEN: usize = 30;

    // Build list of possable characters
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    // Create password
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
