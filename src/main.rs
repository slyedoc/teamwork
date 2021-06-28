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

