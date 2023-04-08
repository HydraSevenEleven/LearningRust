pub fn vector_in_a_for_loop() {
    let mut my_vec: Vec<i32> = vec![1,2,3,4,5];

    my_vec
    .iter()
    .map(|item|{
        println!("{:?}", item);
    })
    .for_each(drop);
    
    println!("{:?}", my_vec);  
}
