struct Person {  // by convention in CamelCase 
    surname: String,
    first_name:String,    
    reward: i32
}

impl Person{
    fn compute_taxes(&self, tax_rate:f32) -> i32 {  // always a reference on the instance of the struct
        (self.reward as f32 * tax_rate) as i32
    }
}

pub fn person_manager() {
    let person = Person {
        surname: String::from("Dalton"),
        first_name: String::from("Joe"),        
        reward: 100_000,
    };
    
    println!("The reward for {} {} is {}$. Catch him (if you can).. and you have to pay {}$ in taxes", 
        person.first_name, 
        person.surname, 
        person.reward, 
        person.compute_taxes(0.3)
    );
}