use crate::game::space_objects::*;
use crate::game::shot::*;

pub struct Spaceship {
    position: Position,
    direction: Direction,
    screen_size: ScreenSize,
    color: Color,
    size: f64,
    shoots: Vec<Shot>
}

impl Spaceship {
    pub fn new(position: Position, direction: Direction, screen_size: ScreenSize,
                color: Color, size: f64) -> Spaceship {
        Spaceship {
            position: position,
            direction: direction,
            screen_size: screen_size,
            color: color,
            size: size ,
            shoots: Vec::new(),
        }
    }

    pub fn screen_size(&self) -> Position {
        self.screen_size
    }


    pub fn move_to(&mut self, position: Position) {
        self.position = position;
    }

    pub fn update_shot_position(&mut self) {
        let screen_size = self.screen_size();
        
        self.shoots.iter_mut()
            .for_each(|shot| shot.action());
        self.shoots
            .retain(|shot| shot.is_visible(screen_size));
    }

    pub fn fire(&mut self) {
        let position = self.position();
        let direction = self.direction();

        self.shoots.push(Shot::new(position, direction));
    }

    pub fn as_space_objects(&self) -> Vec<Box<& dyn SpaceObject>> {
        let mut objects:Vec<Box<& dyn SpaceObject>> = vec![];
        
        self.shoots
            .iter()
            .for_each(|shot| objects.push(Box::new(shot)));
        
        objects.push(Box::new(self));
        
        objects
    }
}

impl SpaceObject for Spaceship {
    fn color(&self) -> Color {
       self.color
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn size(&self) -> f64 {
        self.size
    }
    
    fn position(&self) -> Position {
        self.position
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn spacheship_shots(spaceship: Spaceship) -> Vec<Shot> {
        spaceship.shoots
    }
}