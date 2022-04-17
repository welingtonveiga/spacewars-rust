use crate::game::space_objects::*;

pub struct Player {
    position: Position,
    screen_size: ScreenSize
}

impl Player {
    pub const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const SPEED: f64 = 30.0;
    pub const SIZE: f64 = 25.0;
    pub const DIRECTION: Direction = Direction::UP;

    pub fn new(screen_size: ScreenSize) -> Player {
        let (width, height) = screen_size;
        Player {
            position: (width/2.0, height-30.0),
            screen_size: screen_size
        }
    }

    pub fn move_to(&mut self, direction: Direction) {
        let (width, _) = self.screen_size;
        let (cur_x, cur_y) =  self.position;
        let new_x = match direction {
            Direction::LEFT => min(cur_x + Player::SPEED, width),
            _ => max(cur_x - Player::SPEED, 0.0),
        };
        self.position = (new_x, cur_y);
    }
}

impl SpaceObject for Player {
    
    fn color(&self) -> Color {
        Player::COLOR
    }

    fn direction(&self) -> Direction {
        Player::DIRECTION
    }

    fn size(&self) -> f64 {
        Player::SIZE
    }
    
    fn position(&self) -> Position {
        self.position
    }
}