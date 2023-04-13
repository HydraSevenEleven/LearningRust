use crate::design_pattern::strategy::concrete_strategies::walk::WalkWithFeet;
use crate::design_pattern::strategy::concrete_strategies::fly::FlyWithWings;
use crate::design_pattern::strategy::concrete_strategies::i_cannot::ICanNot;
use crate::design_pattern::strategy::locomotion_trait::LocomotionBehaviour;
use crate::design_pattern::locomotion_enum::LocomotionEnum;

pub trait Duck {
    fn get_locomotion_behaviour(&self, index: usize) -> &dyn LocomotionBehaviour;

    fn locomotion(&self, index:LocomotionEnum) {
        let s: &dyn LocomotionBehaviour = match index {
            LocomotionEnum::Fly => self.get_locomotion_behaviour(0),
            LocomotionEnum::Walk => self.get_locomotion_behaviour(1),
            _ => self.get_locomotion_behaviour(2),
        };
        s.locomotion();
    }
}

pub struct MallardDuck {
    possible_locomotion_behaviour:Vec<Box<dyn LocomotionBehaviour>>
}
    

impl Duck for MallardDuck {
    fn get_locomotion_behaviour(&self, index:usize) -> &dyn LocomotionBehaviour {
        return &(*self.possible_locomotion_behaviour[index]);
    }
}

impl MallardDuck {
    pub fn new() -> Self { 
        let possible_locomotion_behaviour: Vec<Box<dyn LocomotionBehaviour>> = vec![Box::new(FlyWithWings), Box::new(WalkWithFeet), Box::new(ICanNot)];
        MallardDuck { possible_locomotion_behaviour }
    }    
}