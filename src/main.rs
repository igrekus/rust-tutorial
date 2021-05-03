use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number.");

    let secret_num = rand::thread_rng().gen_range(1..101);

    // println!("The secret num is: {}", secret_num);

    loop {
        println!("Input guess:");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("type a number"); continue; },
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => { println!("win"); break; },
        }
        
        // println!("Your guess: {}", guess);
    }
}
