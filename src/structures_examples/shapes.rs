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

struct Triangle {
    shape: Shape,
    base: f32,
    height: f32,
}

enum ShapeType{
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

    fn render_infos(&self) {
        match self {
            ShapeType::Circle(item) => println!("This is a {} (which has id {} and {} shape(s)) of radius {} and area {}", item.shape.name, item.shape.id, item.shape.edges, item.radius, self.calculate_area()),
            ShapeType::Triangle(item) => println!("This is a {} (which has id {} and {} shape(s)) with basis {} and height {} and area {}", item.shape.name, item.shape.id, item.shape.edges, item.base, item.height, self.calculate_area())
        }
    }
}

pub fn shape_manager() {
    let circle = Circle {
        shape: Shape {
            id: 1,
            edges: 0,
            name: String::from("circle"),
        },
        radius:2,
    };

    let triangle = Triangle {
        shape: Shape {
            id:2,
            edges:3,
            name: String::from("triangle"),
        },
        height: 3.0,
        base: 2.0,
    };

    let shape_type_circle = ShapeType::Circle(circle);
    shape_type_circle.render_infos();

    let shape_type_triangle = ShapeType::Triangle(triangle);
    shape_type_triangle.render_infos();
}