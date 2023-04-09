const PI:f32 = 3.14;

struct Shape {
    id: i32,
    edges: i8,
    name: String,
}

struct Circle {
    shape: Shape,
    radius: i32,
}

impl Circle {
    fn area(&self) -> f32 {
        f32::powf(self.radius as f32, 2.0) * PI
    }
}

pub fn shape_manager() {
    let circle = Circle {
        shape: Shape {
            id: 1,
            edges: 0,
            name: String::from("Circle"),
        },
        radius:2,
    };
    
    println!("{}) Circle type: {}, radius: {}, area: {} -- Circles have: {} edges", 
        circle.shape.id, 
        circle.shape.name, 
        circle.radius, 
        circle.area(), 
        circle.shape.edges
    );
}