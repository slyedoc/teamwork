use rand::Rng;

fn main() {

    let mut rng = rand::thread_rng();
    let length = rng.gen_range(10..15);
    let mut letters = vec![];
    
    // Build list of letters to use, using for loop
    for c in 'A'..'Z' {
        letters.push(c);
    }
    // add to list using iterators
    ('a'..'z').for_each(|c| letters.push(c) );

    // Same as above, but using a for loop instead
    let mut password_for = "".to_string();
    for _ in 0..length {
        let random_index = rng.gen_range( 0..letters.len() );
        password_for.push(letters[random_index]);
    }

    // using iterators
    let password_iter = (0..length)
    .map(|_| {
        let random_index = rng.gen_range(0..letters.len());
        letters[random_index]
    }).collect::<String>();

    println!("iter = {}", password_iter);
    println!("for = {}", password_for);
}

