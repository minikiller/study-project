use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Hello, world!:");

    let result = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {result}",);

    // let guess1= guess.parse::<i32>().unwrap();
    // if guess1 == result {
    //     println!("You win!");
    // } else {
    //     println!("You lose!");
    // }

    loop {
        let mut guess = String::new();
        // println!("You guessed: {guess}",);

        io::stdin().read_line(&mut guess).expect("msg");
        match guess.trim().parse::<u32>() {
            Ok(num) => match num.cmp(&result) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => {
                    println!("Too big!");
                    continue;
                }
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            },
            Err(e) => {
                println!("You lose! {e}");
            }
        }
    }
}
