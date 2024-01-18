pub fn my_function(name1 : &str, name2: &str) -> String {
    let statement = format!("the names are {0}, {1}", name1, name2);
    return statement;
}


pub mod private_fun {
    pub fn get_age_plus_5(age: u16) -> u16 {
        return age + 5;
    }
}