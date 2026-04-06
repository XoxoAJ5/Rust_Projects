use std::io;
use std::cmp::Ordering;
fn main() {
    let num: u32 = rand::random_range(1..=100);
    loop {
        println!("what is your guess?:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("please type a number");
        match guess.cmp(&num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("correct");
                break;
            }
        }
    }
}