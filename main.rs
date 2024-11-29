use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match  guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!{"Please enter the valid input"}
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal =>{
                println!("You Won!");
                break;
            },
            
            Ordering::Greater => println!("The number is to big"),
            Ordering::Less => println!("The number is to small"),
        }
    }

}

