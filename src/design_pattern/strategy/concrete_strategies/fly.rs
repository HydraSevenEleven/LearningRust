use crate::design_pattern::strategy::locomotion_trait::LocomotionBehaviour;

pub struct FlyWithWings;

impl LocomotionBehaviour for FlyWithWings {
    fn locomotion(&self) {
        println!("I can fly!")
    }
}