/*
    error[E0658]: exclusive range pattern syntax is experimental
    --> src/main.rs:34:9
    |
    34 |         60..70 => 'D',
    |         ^^^^^^
    |
    = note: see issue #37854 <https://github.com/rust-lang/rust/issues/37854> for more information
 */

fn match_example() {
    let marks: i32 = 65;
    let grade: char = match marks {
        90..=100 => 'A',  // range include 100
        80..=89 => 'B', 
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F'
    };
    println!("The grade you achieve is {}", grade);
}