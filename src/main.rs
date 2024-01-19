use closures::test_closure;

// use std::io;
pub mod closures;

fn main() {
    let result = test_closure(1, 1);
    print!("{}", result);
}