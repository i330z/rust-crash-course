// use std::io;

fn main(){
    // let age_to_drive : u8 = 16;
    // println!("Enter the age");
    // let mut age =  String::new();

    // io::stdin().read_line(&mut age).expect("Failed to read");
    
    // let age: u8 = age.trim().parse().expect("enter a number");

    // if age >= age_to_drive{
    //     println!("Can Drive");
    // } else {
    //     println!("Cannot Drive");
    // }
    // short_statement(age_to_drive, age);

    vector_loop();
}

#[allow(dead_code)]
fn short_statement(age_to: u8, age: u8)  {
    
    let result = if age >= age_to {"Can Drive"} else {"Cannot Drive"};
    print!("{}", result);
}

#[allow(dead_code)]
// Iterating over a range
fn looping(){
    for i in 1..=5 {
        println!("{}", i);
    }
}

#[allow(dead_code)]
 // Iterating over an array
fn looping_arr(){
    let arr = [1,2,3,4,5];

    for i in arr.iter() {
        println!("{}",i);
    }
}


//Looping a vector
fn vector_loop(){
    let vector = vec![100,200,300,49,50];

    for i in &vector {
        println!("{}", i);
    }
}