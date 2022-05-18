pub type Color = [f32; 4];
pub type Position = (f64, f64);
pub type Coord = Vec<[f64; 2]>;
pub type ScreenSize = (f64, f64);
pub type FontSize = u32;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}

const SCREEN_MARGIN: f64 = 20.0;

const MIN_DISTANCE: f64 = 20.0;

pub trait SpaceObject {
    fn color(&self) -> Color;

    fn direction(&self) -> Direction;

    fn position(&self) -> Position;

    fn size(&self) -> f64;

    fn coord(&self) -> Coord {
        let (x, y) = self.position();
        let direction_y: f64 = match self.direction() {
            Direction::UP => y - self.size(),
            _ => y + self.size(),
        };

        vec![[x - self.size(), y], [x, direction_y], [x + self.size(), y]]
    }

    fn is_visible(&self, screen_size: ScreenSize) -> bool {
        let (x, y) = self.position();
        let (width, height) = screen_size;

        x < width + SCREEN_MARGIN
            && x > -SCREEN_MARGIN
            && y < height + SCREEN_MARGIN
            && y > -SCREEN_MARGIN
    }

    fn check_collision(&self, other: &dyn SpaceObject) -> bool {
        let (x, y) = self.position();
        let (other_x, other_y) = other.position();

        (x - other_x).abs() < MIN_DISTANCE && (y - other_y).abs() < MIN_DISTANCE
    }
}

pub struct GameText {
    content: String,
    color: Color,
    font_size: FontSize,
    position: Position,
}

impl GameText {
    pub fn new(content: String, color: Color, font_size: FontSize, position: Position) -> GameText {
        GameText {
            content: content,
            color: color,
            font_size: font_size,
            position: position,
        }
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn font_size(&self) -> FontSize {
        self.font_size
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

pub fn min(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}

pub fn max(x: f64, y: f64) -> f64 {
    if x > y {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyObject {
        position: Position,
    }

    impl SpaceObject for DummyObject {
        fn color(&self) -> Color {
            [1.0, 1.0, 1.0, 1.0]
        }
        fn direction(&self) -> Direction {
            Direction::DOWN
        }
        fn size(&self) -> f64 {
            50.0
        }
        fn position(&self) -> Position {
            self.position
        }
    }

    #[test]
    fn is_visible_should_return_true_when_object_is_in_the_middle_of_screen() {
        // Arrange
        let (width, height) = (100.0, 100.0);
        let space_object = DummyObject {
            position: (width / 2.0, height / 2.0),
        };

        // Act
        let visible = space_object.is_visible((width, height));

        // Assert
        assert!(visible);
    }

    #[test]
    fn is_visible_should_return_false_when_object_x_is_greater_than_the_width_plus_margin() {
        // Arrange
        let (width, height) = (100.0, 100.0);
        let space_object = DummyObject {
            position: (width + SCREEN_MARGIN + 1.0, height / 2.0),
        };

        // Act
        let visible = space_object.is_visible((width, height));

        // Assert
        assert_eq!(visible, false);
    }

    #[test]
    fn is_visible_should_return_false_when_object_x_is_lower_than_the_minus_margin() {
        // Arrange
        let (width, height) = (100.0, 100.0);
        let space_object = DummyObject {
            position: (-1.0 * SCREEN_MARGIN - 1.0, height / 2.0),
        };

        // Act
        let visible = space_object.is_visible((width, height));

        // Assert
        assert_eq!(visible, false);
    }

    #[test]
    fn is_visible_should_return_false_when_object_y_is_greater_than_the_height_plus_margin() {
        // Arrange
        let (width, height) = (100.0, 100.0);
        let space_object = DummyObject {
            position: (width / 2.0, height + SCREEN_MARGIN + 1.0),
        };

        // Act
        let visible = space_object.is_visible((width, height));

        // Assert
        assert_eq!(visible, false);
    }

    #[test]
    fn is_visible_should_return_false_when_object_y_is_lower_than_the_minus_margin() {
        // Arrange
        let (width, height) = (100.0, 100.0);
        let space_object = DummyObject {
            position: (width / 2.0, -1.0 * SCREEN_MARGIN - 1.0),
        };

        // Act
        let visible = space_object.is_visible((width, height));

        // Assert
        assert_eq!(visible, false);
    }

    #[test]
    fn is_visible_should_return_true_when_object_is_on_the_first_corner() {
        // Arrange
        let (width, height) = (100.0, 100.0);
        let space_object = DummyObject {
            position: (0.0, 0.0),
        };

        // Act
        let visible = space_object.is_visible((width, height));

        // Assert
        assert!(visible);
    }

    #[test]
    fn check_collision_should_return_true_when_the_objects_are_at_same_place() {
        // Arrange
        let (x, y) = (100.0, 100.0);

        let space_object = DummyObject { position: (x, y) };

        let other = DummyObject { position: (x, y) };

        // Act
        let collision = space_object.check_collision(&other);

        // Assert
        assert!(collision);
    }

    #[test]
    fn check_collision_should_return_false_when_the_objects_x_is_farther_than_min_distance() {
        // Arrange
        let (x, y) = (100.0, 100.0);

        let space_object = DummyObject { position: (x, y) };

        let other = DummyObject {
            position: (x + MIN_DISTANCE + 1.0, y),
        };

        // Act
        let collision = space_object.check_collision(&other);

        // Assert
        assert_eq!(collision, false);
    }

    #[test]
    fn check_collision_should_return_false_when_the_objects_y_is_farther_than_min_distance() {
        // Arrange
        let (x, y) = (100.0, 100.0);

        let space_object = DummyObject { position: (x, y) };

        let other = DummyObject {
            position: (x, y + MIN_DISTANCE + 5.0),
        };

        // Act
        let collision = space_object.check_collision(&other);

        // Assert
        assert_eq!(collision, false);
    }

    #[test]
    fn check_collision_should_return_false_when_the_objects_x_and_y_is_farther_than_min_distance() {
        // Arrange
        let (x, y) = (100.0, 100.0);

        let space_object = DummyObject { position: (x, y) };

        let other = DummyObject {
            position: (x + MIN_DISTANCE + 12.0, y + MIN_DISTANCE + 5.0),
        };

        // Act
        let collision = space_object.check_collision(&other);

        // Assert
        assert_eq!(collision, false);
    }

    #[test]
    fn check_collision_should_return_false_when_the_objects_x_is_closer_than_min_distance() {
        // Arrange
        let (x, y) = (100.0, 100.0);

        let space_object = DummyObject { position: (x, y) };

        let other = DummyObject {
            position: (x + MIN_DISTANCE - 5.0, y),
        };

        // Act
        let collision = space_object.check_collision(&other);

        // Assert
        assert!(collision);
    }

    #[test]
    fn check_collision_should_return_false_when_the_objects_y_is_closer_than_min_distance() {
        // Arrange
        let (x, y) = (100.0, 100.0);

        let space_object = DummyObject { position: (x, y) };

        let other = DummyObject {
            position: (x, y + MIN_DISTANCE - 1.0),
        };

        // Act
        let collision = space_object.check_collision(&other);

        // Assert
        assert!(collision);
    }
}
