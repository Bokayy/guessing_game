use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    //guessing game :)
    let mut is_correct: bool = false;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    

    println!("Guess the number!");

    while is_correct == false
    {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Secret  number has a higher value!"),
            Ordering::Equal => {println!("You win motherfucker! Fucking kill yourself!"); is_correct = true; break;},
            Ordering::Greater => println!("Secret number has a lower value!"),
        }
        };

        
    }
    //println!("The secret number is: {secret_number}");

    //println!("You guessed: {guess} ");

    /* let x = 5;
    let y = 10;
    println!("x={ } and y = { }",x,y); */

