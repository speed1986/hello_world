use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut no_retries = 1;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error during reading line");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(err_val) => {
                println!("incorrect value: {} \n Provide correct integer - error: ", err_val);
                continue;
            }
        };

        match input.cmp(&secret_number) {
            Ordering::Greater => {
                println!("Too big!");
                no_retries +=1;
            },
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Less => {
                println!("Too small!");
                no_retries +=1;
            },
        }
    }

    println!("Resolved in {} attempt(s)", no_retries)
}
