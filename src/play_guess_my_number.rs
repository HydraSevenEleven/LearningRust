/*
    You can use it in the main function of main.rs like this:
        mod loop_example;
        fn main() {
            loop_example::loop_example();
        }
    Please, take care that this file is in the same directory as main.rs (i.e. under src)
*/
use std::io::stdin;

/*
    Don't forget to add the rand dependency to Cargo.toml like this:
        [dependencies]
        rand = "0..8"
    and then execute cargo check or cargo build or cargo run 
*/
use rand::Rng;

pub fn play_guess_my_number() {
    println!("try to guess my number, which is between 1 and 10");

    let mut round = 1;
    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop {
        // define an empty new string as mutable 
        let mut input_number_as_string = String::new();
        // read a value from the std input
        stdin().read_line(&mut input_number_as_string).expect("failed to read input"); // The program will crash here if an 'Exception' is thrown

        // Convert string into an integer
        let guessed_number = input_number_as_string
        .trim() // we don't need backspaces or something like that
        .parse() // parse the string into a number
        .expect("I'm sorry, but your input is invalid!"); // the program will crash here (try to type a letter instead of a number ;-))
        
        if secret_number == guessed_number {
            println!("CONGRATULATIONS! YOU WON in {} rounds!", round);
            break;
        }
        else {
            println!("I'm sorry, {} is not the right number... try again!", guessed_number);
        }
        round += 1;
    }
}