pub mod shape_type {
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