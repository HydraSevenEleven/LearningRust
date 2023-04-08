pub fn vector_in_a_for_loop() {
    let mut my_vec: Vec<i32> = vec![1,2,3,4,5];

    my_vec
    .iter_mut()
    .map(|item|{
        *item += 1;  // To do this I need to define my_vec as mutable and use the iter_mut() fuunction 
        println!("{:?}", item);
    })
    .for_each(drop);
    
    println!("{:?}", my_vec);  // my_vec has changed
}
