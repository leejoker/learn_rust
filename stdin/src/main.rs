use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    //create a random number
    let secret_number = rand::thread_rng().gen_range(1, 100);

    loop {
        println!("Please input you guess.");
        let mut guess = String::new();
        //read number
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");
        //convert string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number!");
                continue;
            }
        };
        println!("You guessed:  {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
