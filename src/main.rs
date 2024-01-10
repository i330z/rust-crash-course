use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){

    let secret_number =  rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);

    loop {
        println!("Enter your number:");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read number");
        println!("your guess was : {guess}");
    
        let guess: u32 = guess.trim().parse().expect("Please enter number.");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
    
}
