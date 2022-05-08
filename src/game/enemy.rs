use rand::{thread_rng, Rng};
use crate::game::game_objects::*;
use crate::game::player::*;
use crate::game::spaceship::*;

pub struct Enemy {
    spaceship: Spaceship,
    attack_rate: f64,
}

impl Enemy {
    pub const INITIAL_ATTACK_RATE: f64 = 0.01;
    pub const COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const X_SPEED: f64 = 45.0;
    pub const Y_SPEED: f64 = 1.0;
    pub const Y_START: f64 = -15.0;    
    pub const SIZE: f64 = 20.0;
    pub const DIRECTION: Direction = Direction::DOWN;
    
    pub fn new(screen_size: ScreenSize)-> Enemy {
        let mut random = thread_rng();

        let (width, _) = screen_size;
        let min_x = Enemy::SIZE;
        let max_x = width - Enemy::SIZE;
        let gen_x = random.gen_range(min_x..max_x);       
       
       Enemy {
        attack_rate: Enemy::INITIAL_ATTACK_RATE,
        spaceship: Spaceship::new (
                (f64::from(gen_x), Enemy::Y_START),
                Enemy::DIRECTION,
                screen_size,
                Enemy::COLOR,
                Enemy::SIZE 
            )
        }
    }


    fn calculate_x_move(&self)-> f64 {
        let mut random = thread_rng();
        let (curr_x, _) = self.spaceship.position();
            
        let movement = if  random.gen_bool(0.05) {
            let  move_range = 2.0 * Enemy::X_SPEED;
            random.gen_range(0.0..move_range) - Enemy::X_SPEED        
        } else {
            0.0
        };

        curr_x + movement
    }

    fn calculate_y_move(&self)-> f64 {
        let (_, curr_y) = self.spaceship.position();
        curr_y + Enemy::Y_SPEED
    }

    fn move_spaceship(&mut self) {
        let (width, _) = self.spaceship.screen_size();

        let new_x =  max(min(self.calculate_x_move(), width), 0.0);
        let new_y =  self.calculate_y_move();
        
        self.spaceship.move_to((new_x, new_y));
    }

    fn attack(&mut self) {
        let mut random = thread_rng();       
        if random.gen_bool(self.attack_rate)  {
            self.spaceship.fire();
        }
    }
}


impl Player for Enemy {

    fn spaceship(&self) -> &Spaceship {
        &self.spaceship
    }

    fn spaceship_mut(&mut self) -> &mut Spaceship {
        &mut self.spaceship
    }

    fn action(&mut self) {
        self.move_spaceship();
        self.attack();
        self.spaceship.update_shot_position();        
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::spaceship::tests::*;

    #[test]
    fn attack_should_fire_spaceship_when_rate_is_one() {
        // Arrange
        let mut enemy = Enemy::new((800.0, 600.0));
        enemy.attack_rate = 1.0;

        // Act
        enemy.attack();
    
        // Assert
        assert_eq!(spacheship_shots(enemy.spaceship).len(), 1);
    }

    #[test]
    fn attack_should_not_fire_spaceship_when_rate_is_zero() {
        // Arrange
        let mut enemy = Enemy::new((800.0, 600.0));
        enemy.attack_rate = 0.0;

        // Act
        enemy.attack();
    
        // Assert
        assert_eq!(spacheship_shots(enemy.spaceship).len(), 0);
    }
}