use crate::design_pattern::strategy::locomotion_trait::LocomotionBehaviour;

pub struct WalkWithFeet;

impl LocomotionBehaviour for WalkWithFeet {
    fn locomotion(&self) {
        println!("I can walk!")
    }
}