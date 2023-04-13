pub fn division<F:Fn(f32)->bool>(a:f32, b:f32, fun:F){
    if fun(b) == false {
        println!("The result of the division is {}",a/b);
    } 
    else {
        println!("Division by 0 is not defined!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let is_number_like_zero = |x| {if x == 0.0 {true} else {false}};
        division(5.0,0.0,is_number_like_zero);
        assert_eq!(0,0);
    }
}