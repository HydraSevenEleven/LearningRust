pub fn vector_in_a_for_loop() {
    let mut my_vec: Vec<i32> = vec![1,2,3,4,5];

    for item in my_vec {  //the ownership passes here from my_vec to item
        println!("{}", item);
    }

    println!("{:?}", my_vec);  // compiler error
}