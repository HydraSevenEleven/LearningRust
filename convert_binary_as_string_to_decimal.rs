///
/// Obviously, it is not a good idea to pass a binary number as a string.
/// The point here is to play with the 'map' and the 'enumerate' functions.
/// On the other hand, you can easily convert a binary number into a decimal number
/// just like this:
///     let binary_number = 0b10;
///     let decimal_number = binary_number as u32;
///     println!("The decimal equivalent of the binary 0b{:b} is {}", binary_number, decimal_number);
/// 
fn convert_binary_as_string_to_decimal(s: &str) -> i64 {
    s.chars().rev()
    .map(|item| item.to_digit(2).unwrap() as i64)
    .enumerate()
    .map(|(index,item)| {
        item * i64::pow(2,index as u32)        
    })
    .sum()   
}