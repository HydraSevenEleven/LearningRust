pub fn vector_in_a_for_loop() {
    let mut my_vec: Vec<i32> = vec![1,2,3,4,5];

    for item in my_vec.iter() {  // the iter() function avoid to change the ownership
        println!("{}", item);
    }

    println!("{:?}", my_vec);  
}
