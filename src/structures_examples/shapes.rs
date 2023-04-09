mod shape {
    pub struct Shape {
        pub id: i32,
        pub edges: i8,
        pub name: String,
    }
    pub struct Circle {
        pub shape: Shape,
        pub radius: i32,
    }
    
    pub struct Triangle {
        pub shape: Shape,
        pub base: f32,
        pub height: f32,
    }
}

mod shape_type {

    use crate::structures_examples::shapes::shape::Circle;
    use crate::structures_examples::shapes::shape::Triangle;

    const PI:f32 = 3.14;
    pub enum ShapeType{
        Circle(Circle),
        Triangle(Triangle)
    }

    impl ShapeType {
        fn calculate_area(&self) -> f32 {
            match self {
                ShapeType::Circle(item) => f32::powf(item.radius as f32, 2.0) * PI,
                ShapeType::Triangle(item) => item.base * item.height / 2.0,
            }
        }

        pub fn render_infos(&self) {
            match self {
                ShapeType::Circle(item) => println!("This is a {} (which has id {} and {} shape(s)) of radius {} and area {}", item.shape.name, item.shape.id, item.shape.edges, item.radius, self.calculate_area()),
                ShapeType::Triangle(item) => println!("This is a {} (which has id {} and {} shape(s)) with basis {} and height {} and area {}", item.shape.name, item.shape.id, item.shape.edges, item.base, item.height, self.calculate_area())
            }
        }
    }
}

pub mod shape_manager {

    use crate::structures_examples::shapes::shape_type::ShapeType;
    use crate::structures_examples::shapes::shape::Circle;
    use crate::structures_examples::shapes::shape::Shape;
    use crate::structures_examples::shapes::shape::Triangle;

    fn create_circle() -> Circle{
        Circle {
            shape: Shape {
                id: 1,
                edges: 0,
                name: String::from("circle"),
            },
            radius:2,
        }
    }

    fn create_triangle() -> Triangle {
        Triangle {
            shape: Shape {
                id:2,
                edges:3,
                name: String::from("triangle"),
            },
            height: 3.0,
            base: 2.0,
        }        
    } 

    pub fn shape_manager() {
        vec![ShapeType::Circle(create_circle()), ShapeType::Triangle(create_triangle())].iter().map(|item| item.render_infos()).for_each(drop);    
    }
}