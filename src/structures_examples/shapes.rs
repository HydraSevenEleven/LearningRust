pub mod shape {
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

pub mod shape_manager {

    use crate::structures_examples::shape_type::shape_type::ShapeType;   

    pub fn shape_manager(vector_of_shape_type: Vec<ShapeType>) {
        vector_of_shape_type.iter().map(|item| item.render_infos()).for_each(drop);    
    }
}