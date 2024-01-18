pub mod healpers;

fn main(){
    let answer = healpers::my_function("jack","rose");
    println!("{}",answer);
    let age =  healpers::private_fun::get_age_plus_5(5);
    print!("The age is: {}", age);
}


