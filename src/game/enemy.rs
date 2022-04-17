use rand::{thread_rng, Rng};
use crate::game::space_objects::*;

#[derive(Copy, Clone)]
pub struct Enemy {
    position: Position,
    screen_size: ScreenSize
}

impl Enemy {
    pub const COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const X_SPEED: f64 = 45.0;
    pub const Y_SPEED: f64 = 1.0;
    pub const Y_START: f64 = -30.0;    
    pub const SIZE: f64 = 20.0;
    pub const DIRECTION: Direction = Direction::DOWN;
    
    pub fn new(screen_size: ScreenSize)-> Enemy {
        let mut random = thread_rng();

        let (width, _) = screen_size;
        let min_x = Enemy::SIZE;
        let max_x = width - Enemy::SIZE;
        let gen_x = random.gen_range(min_x..max_x);       
       
       Enemy {
            position: (f64::from(gen_x), Enemy::Y_START),
            screen_size: screen_size
        }
    }

    pub fn movement(&mut self) {
        let (width, _) = self.screen_size;

        let new_x =  max(min(self.calculate_x_move(), width), 0.0);
        let new_y =  self.calculate_y_move();
        
        self.position = (new_x, new_y);
    }

    fn calculate_x_move(&self)-> f64 {
        let mut random = thread_rng();
        let (curr_x, _) = self.position();
            
        let movement = if  random.gen_bool(0.05) {
            let  move_range = 2.0 * Enemy::X_SPEED;
            random.gen_range(0.0..move_range) - Enemy::X_SPEED        
        } else {
            0.0
        };

        curr_x + movement
    }

    fn calculate_y_move(&self)-> f64 {
        let (_, curr_y) = self.position();
        curr_y + Enemy::Y_SPEED
    }
}

impl SpaceObject for Enemy {
    
    fn color(&self) -> Color {
        Enemy::COLOR
    }

    fn direction(&self) -> Direction {
        Enemy::DIRECTION
    }

    fn size(&self) -> f64 {
        Enemy::SIZE
    }
    
    fn position(&self) -> Position {
        self.position
    }
}