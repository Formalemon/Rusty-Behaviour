use std::io;
use rand::Rng;

fn main() {
    println!("Enter your name: ");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let session_id = rand::thread_rng()
        .gen_range(1..=1001);

    println!("Hello {}!", name.trim());
    println!("Your current session ID is: {}", session_id);
    
    if session_id == 69 {
        println!("Nice!");
    }
}
