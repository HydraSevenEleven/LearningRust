struct Person {  // by convention in CamelCase 
    surname: String,
    first_name:String,    
    reward: i32
}

impl Person{
    fn new() -> Self {
        Person {
            surname: String::from("Doe"),
            first_name: String::from("John"),
            reward: 1,
        }
    }
    fn compute_taxes(&self, tax_rate:f32) -> i32 {  // always a reference on the instance of the struct
        (self.reward as f32 * tax_rate) as i32
    }
}

struct Dalton { // the Dalton structure 'inherit' from the Person structure and add a new property
    id: i32,
    person: Person,
}

impl Dalton {
    fn new() -> Self {
        Dalton {
            id: 1,
            person: Person::new(),
        }
    }
}

pub fn person_manager() {
    let dalton = Dalton {
        id: 1,
        person: Person {
            surname: String::from("Dalton"),
            first_name: String::from("Joe"),        
            reward: 100_000,
        },
    };

    println!("The Dalton with id {} is {} {}. For him you could earn a reward of {}$. Catch him (if you can).. but you have to pay {}$ in taxes", 
        dalton.id,
        dalton.person.first_name, 
        dalton.person.surname, 
        dalton.person.reward, 
        dalton.person.compute_taxes(0.3)
    );

    let default_person = Person::new();
    println!("The default person is {} {}. For him you could earn a reward of {}$. Catch him (if you can).. but you have to pay {}$ in taxes", 
        default_person.first_name, 
        default_person.surname, 
        default_person.reward, 
        default_person.compute_taxes(0.3)
    );

    let default_dalton = Dalton::new();

    println!("The default Dalton with id {} is {} {}. For him you could earn a reward of {}$. Catch him (if you can).. but you have to pay {}$ in taxes", 
        default_dalton.id,
        default_dalton.person.first_name, 
        default_dalton.person.surname, 
        default_dalton.person.reward, 
        default_dalton.person.compute_taxes(0.3)
    );
}