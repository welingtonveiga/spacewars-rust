use rand::{thread_rng, Rng};

use crate::game::space_objects::*;
use crate::game::player::*;
use crate::game::hero::*;
use crate::game::enemy::*;
use crate::game::stars::*;
pub use crate::game::space_objects:: {Direction, SpaceObject};

mod space_objects;
mod spaceship;
mod shot;
mod player;
mod hero;
mod enemy;
mod stars;



pub struct Game {
    screen_size: ScreenSize,
    hero: Hero,
    enemies: Vec<Enemy>,
    background_stars: Vec<Star>,
}

impl Game {

    pub const STAR_COUNT: i32 = 100;
    pub const ENEMY_FREQUENCY: f64 = 0.015;

    pub fn new(width: f64, height: f64) -> Game {
       let screen_size = (width, height);

       let stars  = (0..Game::STAR_COUNT)
            .map(|_| -> Star { Star::new(screen_size)})
            .collect();

        Game {
            screen_size: screen_size,
            hero: Hero::new(screen_size),
            enemies: Vec::new(),
            background_stars: stars,
        }
    }

    fn background_stars_movement(&mut self) {
        self.background_stars.iter_mut().for_each(|star| star.fall())
    }

    fn generate_enemies(&mut self) {
        let mut random = thread_rng();
        
        self.enemies.retain(|enemy| enemy.spaceship().is_visible(self.screen_size));

        if random.gen_bool(Game::ENEMY_FREQUENCY) {
            let enemy = Enemy::new(self.screen_size);
            self.enemies.push(enemy);
        }
    }

    fn enemies_action(&mut self) {     
        self.enemies.iter_mut().for_each(|enemy| enemy.action())
    }

    fn player_action(&mut self) {     
        self.hero.action();
    }

    pub fn next_turn(&mut self) {        
        self.background_stars_movement();
        self.generate_enemies();
        self.enemies_action();   
        self.player_action();       
    }

    pub fn as_space_objects(&self) -> Vec<Box<& dyn SpaceObject>> {
        let mut objects:Vec<Box<& dyn SpaceObject>> = vec![];

        for star in &self.background_stars {            
            objects.push(Box::new(star));
        }

        for enemy in &self.enemies {
            objects.append(&mut enemy.spaceship().as_space_objects());         
        }

        objects.append(&mut self.hero.spaceship().as_space_objects());

        objects  
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.hero.move_to(direction);
    }

    pub fn fire_player_attack(&mut self) {
        self.hero.attack();        
    }        
}
