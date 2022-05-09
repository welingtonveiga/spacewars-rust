use crate::game::game_objects::*;

#[derive(Copy, Clone, Debug)]
pub struct Shot {
    position: Position,
    direction: Direction,
}

impl Shot {
    pub const COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
    pub const SPEED: f64 = 10.0;
    pub const SIZE: f64 = 5.0;

    pub fn new(position: Position, direction: Direction) -> Shot {
        Shot {
            position: position,
            direction: direction,
        }
    }

    pub fn action(&mut self) {
        let (cur_x, cur_y) = self.position;
        let new_y = match self.direction() {
            Direction::UP => cur_y - Shot::SPEED,
            Direction::DOWN => cur_y + Shot::SPEED,
            direction => panic!("Unexpected Durection {:?}", direction),
        };
        self.position = (cur_x, new_y);
    }
}

impl SpaceObject for Shot {
    fn color(&self) -> Color {
        Shot::COLOR
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn size(&self) -> f64 {
        Shot::SIZE
    }

    fn position(&self) -> Position {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_should_not_change_x() {
        // Arrange
        let x = 10.0;
        let position = (x, 50.0);
        let direction = Direction::DOWN;
        let mut shot = Shot::new(position, direction);

        // Act
        shot.action();

        // Assert
        let (new_x, _) = shot.position;
        assert_eq!(new_x, x);
    }

    #[test]
    fn update_should_increase_y_by_speed_if_direction_is_down() {
        // Arrange
        let y = 140.0;
        let position = (50.0, y);
        let direction = Direction::DOWN;
        let mut shot = Shot::new(position, direction);

        // Act
        shot.action();

        // Assert
        let (_, new_y) = shot.position;
        assert_eq!(new_y, y + Shot::SPEED);
    }

    #[test]
    fn update_should_decrease_y_by_speed_if_direction_is_up() {
        // Arrange
        let y = 140.0;
        let position = (50.0, y);
        let direction = Direction::UP;
        let mut shot = Shot::new(position, direction);

        // Act
        shot.action();

        // Assert
        let (_, new_y) = shot.position;
        assert_eq!(new_y, y - Shot::SPEED);
    }

    #[test]
    #[should_panic]
    fn update_should_panic_if_direction_is_not_up_and_down() {
        // Arrange
        let position = (50.0, 100.0);
        let direction = Direction::LEFT;
        let mut shot = Shot::new(position, direction);

        // Act
        shot.action();
    }
}
