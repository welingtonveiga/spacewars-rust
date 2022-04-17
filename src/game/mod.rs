use rand::{thread_rng, Rng};
use std::time::{Instant, Duration};

use crate::game::space_objects::*;
use crate::game::player::*;
use crate::game::enemy::*;
use crate::game::stars::*;
use crate::game::shot::*;
pub use crate::game::space_objects:: {Direction, SpaceObject};

mod space_objects;
mod player;
mod enemy;
mod stars;
mod shot;


pub struct Game {
    screen_size: ScreenSize,
    player: Player,
    stars: Vec<Star>,
    enemies: Vec<Enemy>,
    shots: Vec<Shot>,
    last_attack: Option<Instant>,
}

impl Game {

    pub const STAR_COUNT: i32 = 100;
    pub const ENEMY_FREQUENCY: f64 = 0.015;
    pub const PLAYER_ATTACK_FREQUENCE: u64 = 200;

    pub fn new(width: f64, height: f64) -> Game {
       let screen_size = (width, height);

       let stars  = (0..Game::STAR_COUNT)
            .map(|_| -> Star { Star::new(screen_size)})
            .collect();


        Game {
            screen_size: screen_size,
            player: Player::new(screen_size),
            stars: stars,
            enemies: Vec::new(),
            shots: Vec::new(),
            last_attack: None
        }
    }

    fn update_stars(&mut self) {
        let mut new_stars:Vec<Star> = vec![];

        for star in self.stars.iter_mut() {
            let new_star = star.fall();
            new_stars.push(new_star);
        }

        self.stars = new_stars;
    }

    fn update_enemies(&mut self) {
        let mut random = thread_rng();
        
        if random.gen_bool(Game::ENEMY_FREQUENCY) {
            self.enemies.push(Enemy::new(self.screen_size));
        }
        
        for enemy in self.enemies.iter_mut() {
            enemy.movement();
        }
    }

    fn update_shots(&mut self) {
        self.shots.iter_mut().for_each(|shot| shot.update())
    }

    pub fn next_tick(&mut self)-> Vec<Box<& dyn SpaceObject>> {
        let mut objects:Vec<Box<& dyn SpaceObject>> = vec![];

        self.update_stars();
        self.update_enemies();
        self.update_shots();

        for star in &self.stars {            
            objects.push(Box::new(star));
        }
        
        for enemy in &self.enemies {
            objects.push(Box::new(enemy));
        }

        for shot in &self.shots {
            objects.push(Box::new(shot));
        }

        objects.push(Box::new(&self.player));

        objects         
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.player.move_to(direction);
    }

    pub fn fire_player_attack(&mut self) {
        if  let Some(last_attack) =  self.last_attack {
            if last_attack.elapsed().as_millis() < Game::PLAYER_ATTACK_FREQUENCE as u128 {
                return;
             }
        }
        self.last_attack = Some(Instant::now());     
        let shot = self.player.attack();
        self.shots.push(shot);         
    }        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fire_player_attack_should_add_new_shot_when_called() {
        // Arrange
        let mut game = Game::new(800.0, 600.0);

        // Act
        game.fire_player_attack();

        // Assert
        assert_eq!(game.shots.len(), 1);
    }

    #[test]
    fn fire_player_attack_should_start_from_none() {
        // Arrange
        let game = Game::new(800.0, 600.0);
    
        // Assert
        assert_eq!(game.last_attack, None);
    }

    #[test]
    fn fire_player_attack_should_set_last_attack_flag_when_called() {
        // Arrange
        let mut game = Game::new(800.0, 600.0);        
        
        // Act
        game.fire_player_attack();

        // Assert
        assert_eq!(game.last_attack.is_some(), true);
    }

    #[test]
    fn fire_player_attack_should_add_shot_if_last_attach_is_older_than_shooting_speed() {
        // Arrange
        let shoot_frequence = Duration::from_millis(Game::PLAYER_ATTACK_FREQUENCE);
        let last_attack = Instant::now().checked_sub(shoot_frequence);

        let mut game = Game::new(800.0, 600.0);      
        game.last_attack = last_attack;

        assert!(last_attack.is_some());

        // Act
        game.fire_player_attack();

        // Assert
        assert_eq!(game.shots.len(), 1);
    }

    #[test]
    fn fire_player_attack_should_not_add_shot_if_last_attach_is_newer_than_shooting_speed() {
        // Arrange
        let shoot_frequence = Duration::from_millis(1);
        // Future
        let last_attack = Instant::now().checked_sub(shoot_frequence);

        let mut game = Game::new(800.0, 600.0);      
        game.last_attack = last_attack;

        assert!(last_attack.is_some());

        // Act
        game.fire_player_attack();

        // Assert
        assert_eq!(game.shots.len(), 0);
    }

    #[test]
    fn fire_player_attack_should_not_update_the_last_attack_when_cannot_attach() {
        // Arrange
        let shoot_frequence = Duration::from_millis(1);
        // Future
        let last_attack = Instant::now().checked_sub(shoot_frequence);

        let mut game = Game::new(800.0, 600.0);      
        game.last_attack = last_attack;

        assert!(last_attack.is_some());

        // Act
        game.fire_player_attack();


        // Assert
        assert_eq!(game.last_attack, last_attack);
    }
}