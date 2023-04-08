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
    let mut index = 0;
    loop {
        println!("This is a finite loop! It stops if index is 5. Now is index = {}", index);
        if index == 5 { break; }
        index += 1;
    }
}

pub fn loop_with_return_value() -> i32 {
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break count;
        }
    }
}