use crate::game::game_objects::*;
use crate::game::spaceship::*;

pub trait Player {

    fn spaceship(&self) -> &Spaceship;

    fn spaceship_mut(&mut self) -> &mut Spaceship;
    
    fn action(&mut self);

    fn hits(&self, other: &mut dyn Player) -> bool {
        if self.spaceship().hits(other.spaceship()) {
            let spaceship = other.spaceship_mut();
            spaceship.destroy();
            return true;
        }

        false
    }

    fn is_destroyed(&self) -> bool {
        self.spaceship().is_destroyed()
    }

    fn is_visible(&self, screen_size: ScreenSize) -> bool {
        self.spaceship().is_visible(screen_size)
    }
}
