use crate::game::spaceship::*;

pub trait Player {    
    fn is_destroyed(&self) -> bool;

    fn spaceship(&self) -> &Spaceship;
    
    fn action(&mut self);
}
