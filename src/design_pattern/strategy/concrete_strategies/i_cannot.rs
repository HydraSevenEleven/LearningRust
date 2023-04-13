use crate::design_pattern::strategy::locomotion_trait::LocomotionBehaviour;

pub struct ICanNot;

impl LocomotionBehaviour for ICanNot {
    fn locomotion(&self) {
        println!("I cannot do this!")
    }
}