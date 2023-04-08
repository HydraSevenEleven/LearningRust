pub fn vector_in_a_for_loop() {
    let my_vec: Vec<i32> = vec![1,2,3,4,5];

    my_vec
    .iter()
    .map(|item|{
        *item += 1;  // item is a reference to the value. That means I cannot change the value of item 
        println!("{:?}", item);
    })
    .for_each(drop);
    
    println!("{:?}", my_vec);  
}
