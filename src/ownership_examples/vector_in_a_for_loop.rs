pub fn vector_in_a_for_loop() {
    let mut my_vec: Vec<i32> = vec![1,2,3,4,5];

    for item in &my_vec {  // another way is to pass only the reference into the loop-scope
        println!("{}", item);
    }

    println!("{:?}", my_vec);  
}
