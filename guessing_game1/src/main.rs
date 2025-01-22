//This is a very small variation on the guessing game proposed early in the online version of the  Rust Book
//This version uses system time instead of the "rand" crate
//As in use rand::Rng;
use std::io;
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
   
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    let solution  = (nanos/10 /*%100*/)+1;
    println!("Welcome to \"guess the number!\"");
    loop {
        println!("Insert your number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Unable to read line");
        
        if guess.trim() == "$" {
            println!("The number is: {}", solution);
            continue;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_) =>continue,
        };
        
        println!("You tried with: {}", guess);
    
        match guess.cmp(&solution) {
            Ordering::Less =>println!("Too small"),
            Ordering::Greater =>println!("Too big"),
            Ordering::Equal =>{
                println!("You won!");
                break;
            }
        }
    }
}
