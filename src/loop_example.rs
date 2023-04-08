/*
    This is an example for an infinite loop in RUST.
    You can use it in the main function of main.rs like this:
        mod loop_example;
        fn main() {
            loop_example::loop_example();
        }
    Please, take care that this file is in the same directory as main.rs (i.e. under src)
 */

pub fn loop_example() {
    loop {
        println!("This is an infinite loop!");
    }
}