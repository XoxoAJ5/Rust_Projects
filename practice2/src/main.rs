use std::io;
fn main() {
    let mut balance: f64 = 0.0;
    loop {
        println!("Welcome! Please choose an option:");
        println!("1: Deposit");
        println!("2: Withdraw");
        println!("3: Check Balance");
        println!("4: Exit");

        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("failed to read line");
        let inp: u8 = inp.trim().parse().expect("please input a number");

        match inp {
            1 => {
                println!("how much do you want to deposit?");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("failed to read line");
                let num: f64 = num.trim().parse().expect("please input a number");
                balance += num;
                println!("");
            }
            2 => {
                println!("how much do you want to withdraw?");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("failed to read line");
                let num: f64 = num.trim().parse().expect("please input a number");
                if num <= balance {
                    balance -= num;
                } else {
                    println!("sorry not enough money");
                    println!("{}", balance);
                }
                println!("");
            }
            3 => {
                println!("{}", balance);
                println!("");
            }
            4 => {
                break;
            }
            _ => {
                println!("please input valid type");
                println!("");
            }
        }
    }
}
