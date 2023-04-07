///
/// This function gets four i64 parameters 
/// First, it calculates the number number = (a^2 + b^2) * (c^2 + d^2)
/// Then it search two numbers e and f so, that e^2 + f^2 = n and gives them back as a Vec<(i64,i64)>.
/// I don't believe this function makes a lot of sense 
fn filter_map_example(a: i64, b: i64, c: i64, d: i64) -> Vec<(i64, i64)> 
{
    let n = (i64::pow(a,2) + i64::pow(b,2)) * (i64::pow(c,2) + i64::pow(d,2));
    let stop_condition = (n as f64).sqrt() as i64;
    
    (1..=stop_condition)
        .filter_map(|item| {
            let tmp = n as f64 - (item as f64).powf(2.0);
            if tmp.sqrt() == tmp.sqrt().round() 
            {
                Some((item, tmp.sqrt() as i64))
            } 
            else 
            {
                None
            }
        })
        .collect()
}     