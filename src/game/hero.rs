use crate::game::game_objects::*;
use crate::game::player::*;
use crate::game::spaceship::*;
use std::time::Instant;

pub struct Hero {
    spaceship: Spaceship,
    last_attack: Option<Instant>,
}

impl Hero {
    pub const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const SPEED: f64 = 30.0;
    pub const SIZE: f64 = 25.0;
    pub const DIRECTION: Direction = Direction::UP;
    pub const ATTACK_THRESHOLD: u64 = 200;

    pub fn new(screen_size: ScreenSize) -> Hero {
        let (width, height) = screen_size;
        Hero {
            last_attack: None,
            spaceship: Spaceship::new(
                (width / 2.0, height - 30.0),
                Hero::DIRECTION,
                screen_size,
                Hero::COLOR,
                Hero::SIZE,
            ),
        }
    }

    pub fn move_to(&mut self, direction: Direction) {
        let (width, _) = self.spaceship.screen_size();
        let (cur_x, cur_y) = self.spaceship.position();
        let new_x = match direction {
            Direction::LEFT => min(cur_x + Hero::SPEED, width),
            _ => max(cur_x - Hero::SPEED, 0.0),
        };
        self.spaceship.move_to((new_x, cur_y));
    }

    pub fn attack(&mut self) {
        if self.should_attack(Hero::ATTACK_THRESHOLD) {
            self.spaceship.fire();
        }
    }

    fn should_attack(&mut self, threshold: u64) -> bool {
        if let Some(last_attack) = self.last_attack {
            if last_attack.elapsed().as_millis() < (threshold as u128) {
                return false;
            }
        }
        self.last_attack = Some(Instant::now());
        true
    }
}

impl Player for Hero {
    fn spaceship(&self) -> &Spaceship {
        &self.spaceship
    }

    fn spaceship_mut(&mut self) -> &mut Spaceship {
        &mut self.spaceship
    }

    fn action(&mut self) {
        self.spaceship.update_shot_position();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::spaceship::tests::*;
    use std::time::Duration;

    #[test]
    fn attack_should_fire_spaceship_shot_on_the_first_attack() {
        // Arrange
        let mut hero = Hero::new((800.0, 600.0));

        // Act
        hero.attack();

        // Assert
        assert_eq!(spacheship_shots(hero.spaceship).len(), 1);
    }

    #[test]
    fn last_attack_should_start_from_none() {
        // Arrange
        let hero = Hero::new((800.0, 600.0));

        // Assert
        assert_eq!(hero.last_attack, None);
    }

    #[test]
    fn last_attack_should_be_defined_after_first_attack() {
        // Arrange
        let mut hero = Hero::new((800.0, 600.0));

        // Act
        hero.attack();

        // Assert
        assert_eq!(hero.last_attack.is_some(), true);
    }

    #[test]
    fn attack_should_add_shot_if_last_attack_diff_is_greater_than_shooting_threshould() {
        // Arrange
        let threshould = Duration::from_millis(Hero::ATTACK_THRESHOLD + 1);
        let last_attack = Instant::now().checked_sub(threshould);

        let mut hero = Hero::new((800.0, 600.0));
        hero.last_attack = last_attack;

        // Act
        hero.attack();

        // Assert
        assert_eq!(spacheship_shots(hero.spaceship).len(), 1);
    }

    #[test]
    fn attack_should_add_not_shot_if_last_attack_diff_is_smaller_than_shooting_threshould() {
        // Arrange
        let threshould = Duration::from_millis(1);
        let last_attack = Instant::now().checked_sub(threshould);

        let mut hero = Hero::new((800.0, 600.0));
        hero.last_attack = last_attack;

        // Act
        hero.attack();

        // Assert
        assert_eq!(spacheship_shots(hero.spaceship).len(), 0);
    }

    #[test]
    fn attack_should_add_not_update_last_attack_whem_cannot_attack() {
        // Arrange
        let threshould = Duration::from_millis(1);
        let last_attack = Instant::now().checked_sub(threshould);

        let mut hero = Hero::new((800.0, 600.0));
        hero.last_attack = last_attack;

        // Act
        hero.attack();

        // Assert
        assert_eq!(hero.last_attack, last_attack);
    }
}
