use crate::game::game_objects::*;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub struct Star {
    size: f64,
    position: Position,
    screen_size: ScreenSize,
}

impl Star {
    pub const COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const SPEED: f64 = 5.0;
    pub const DIRECTION: Direction = Direction::DOWN;

    pub fn new(screen_size: ScreenSize) -> Star {
        let mut rng = thread_rng();

        let (width, height) = screen_size;
        let gen_x = rng.gen_range(0..(width as i32));
        let gen_y = rng.gen_range(0..(height as i32));
        let size: i32 = rng.gen_range(1..=4);

        Star {
            size: f64::from(size),
            position: (f64::from(gen_x), f64::from(gen_y)),
            screen_size: screen_size,
        }
    }

    pub fn fall(&mut self) {
        let (_, height) = self.screen_size;
        let (cur_x, cur_y) = self.position();
        let new_y = if cur_y + Star::SPEED < height {
            cur_y + Star::SPEED
        } else {
            0.0
        };

        self.position = (cur_x, new_y);
    }
}

impl SpaceObject for Star {
    fn color(&self) -> Color {
        Star::COLOR
    }

    fn direction(&self) -> Direction {
        Star::DIRECTION
    }

    fn size(&self) -> f64 {
        self.size
    }

    fn position(&self) -> Position {
        self.position
    }

    fn coord(&self) -> Coord {
        let (x, y) = self.position();
        let size = self.size();
        vec![[x, y], [x + size, y], [x + size, y + size], [x, y + size]]
    }
}
