use rand::{thread_rng, Rng};
use crate::game::space_objects::*;
use crate::game::player::*;
use crate::game::enemy::*;
use crate::game::stars::*;
pub use crate::game::space_objects:: {Direction, SpaceObject};

mod space_objects;
mod player;
mod enemy;
mod stars;


pub struct Game {
    screen_size: ScreenSize,
    player: Player,
    stars: Vec<Star>,
    enemies: Vec<Enemy>,
}

impl Game {

    pub const STAR_COUNT:i32 = 100;
    pub const ENEMY_FREQUENCY:f64 = 0.015;

    pub fn new(width: f64, height: f64) -> Game {
       let screen_size = (width, height);

       let stars  = (0..Game::STAR_COUNT)
            .map(|_| -> Star { Star::new(screen_size)})
            .collect();


        Game {
            screen_size: screen_size,
            player: Player::new(screen_size),
            stars: stars,
            enemies: Vec::new()
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

    pub fn next_tick(&mut self)-> Vec<Box<& dyn SpaceObject>> {
        let mut objects:Vec<Box<& dyn SpaceObject>> = vec![];

        self.update_stars();
        self.update_enemies();

        for star in &self.stars {            
            objects.push(Box::new(star));
        }
        
        for enemy in &self.enemies {
            objects.push(Box::new(enemy));
        }

        objects.push(Box::new(&self.player));

        objects
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.player.move_to(direction);
    }
}