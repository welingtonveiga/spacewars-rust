use crate::game::game_objects::*;
use crate::game::shot::*;

pub struct Spaceship {
    destroyed: bool,
    position: Position,
    direction: Direction,
    screen_size: ScreenSize,
    color: Color,
    size: f64,
    shots: Vec<Shot>
}

impl Spaceship {
    pub fn new(position: Position, direction: Direction, screen_size: ScreenSize,
                color: Color, size: f64) -> Spaceship {
        Spaceship {
            destroyed: false,
            position: position,
            direction: direction,
            screen_size: screen_size,
            color: color,
            size: size ,
            shots: Vec::new(),
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
        
        self.shots.iter_mut()
            .for_each(|shot| shot.action());
        self.shots
            .retain(|shot| shot.is_visible(screen_size));
    }

    pub fn fire(&mut self) {
        let position = self.position();
        let direction = self.direction();

        self.shots.push(Shot::new(position, direction));
    }

    pub fn as_game_objects(&self) -> Vec<Box<& dyn SpaceObject>> {
        let mut objects:Vec<Box<& dyn SpaceObject>> = vec![];
        
        self.shots
            .iter()
            .for_each(|shot| objects.push(Box::new(shot)));
        
        objects.push(Box::new(self));
        
        objects
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
    }

    pub fn hits(&self, other: &Spaceship) -> bool {
        !other.is_destroyed() 
            && self.shots.iter()
                .any(|shot| other.check_collision(shot))
    }

    pub fn is_destroyed(&self) -> bool {
        self.destroyed
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
        spaceship.shots
    }

    #[test]
    fn hits_should_return_false_whe_the_other_spaceship_is_destroyed() {
        // Arrange
        let (width, height) = (100.0, 100.0);
        let (x, y) = (50.0, 50.0);

        let spaceship =  Spaceship::new (
            (x, y),
            Direction::UP,
            (width, height),
            [1.0, 1.0, 1.0, 1.0],
            20.0
        );

        let mut other =  Spaceship::new (
            (x+100.0, y+100.0),
            Direction::UP,
            (width, height),
            [1.0, 1.0, 1.0, 1.0],
            20.0
        );

        other.destroyed = true;

        // Act
        let hits = spaceship.hits(&other);

        // Assert
        assert_eq!(hits, false);
    }

    #[test]
    fn hits_should_return_true_whe_the_other_spaceship_is_at_the_same_position_as_a_shot() {
        // Arrange
        let (width, height) = (100.0, 100.0);
        let (x, y) = (50.0, 50.0);
        let (other_x, other_y) = (50.0, 50.0);

        let mut spaceship =  Spaceship::new (
            (x, y),
            Direction::UP,
            (width, height),
            [1.0, 1.0, 1.0, 1.0],
            20.0
        );

        let other =  Spaceship::new (
            (other_x, other_y),
            Direction::UP,
            (width, height),
            [1.0, 1.0, 1.0, 1.0],
            20.0
        );

        spaceship.shots.push(Shot::new((other_x, other_y), Direction::DOWN));

        // Act
        let hits = spaceship.hits(&other);

        // Assert
        assert_eq!(hits, true);
    }
    #[test]
    fn hits_should_return_false_whe_the_other_destroyed_spaceship_is_at_the_same_position_as_a_shot() {
        // Arrange
        let (width, height) = (100.0, 100.0);
        let (x, y) = (50.0, 50.0);
        let (other_x, other_y) = (50.0, 50.0);

        let mut spaceship =  Spaceship::new (
            (x, y),
            Direction::UP,
            (width, height),
            [1.0, 1.0, 1.0, 1.0],
            20.0
        );
        spaceship.shots.push(Shot::new((other_x, other_y), Direction::DOWN));

        let mut other =  Spaceship::new (
            (other_x, other_y),
            Direction::UP,
            (width, height),
            [1.0, 1.0, 1.0, 1.0],
            20.0
        );
        other.destroyed = true;

        // Act
        let hits = spaceship.hits(&other);

        // Assert
        assert_eq!(hits, false);
    }
}