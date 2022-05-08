use rand::{thread_rng, Rng};

use crate::game::game_objects::*;
use crate::game::player::*;
use crate::game::hero::*;
use crate::game::enemy::*;
use crate::game::stars::*;
pub use crate::game::game_objects:: {Direction, Color, SpaceObject};

mod game_objects;
mod spaceship;
mod shot;
mod player;
mod hero;
mod enemy;
mod stars;

#[derive(Copy, Clone)]
pub enum GameScore {
    Score(u32),
    GameOver(u32),
}

impl GameScore {
    fn is_game_over(&self) -> bool {
        match *self {
            GameScore::GameOver(_) => true,
            _ => false,
        }
    }
}

pub struct Game {
    screen_size: ScreenSize,
    hero: Hero,
    enemies: Vec<Enemy>,
    background_stars: Vec<Star>,
    score: GameScore
}

impl Game {
    pub const STAR_COUNT: i32 = 100;
    pub const SCORE_TEXT_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
    pub const SCORE_TEXT_SIZE: u32 = 32;
    pub const SCORE_TEXT_POSITION: Position = (30.0 , 30.0);
    pub const GAME_OVER_POSITION_PADDING: f64 = 64.0;
    pub const FINAL_SCORE_PADDING: f64 = 32.0;
    pub const ENEMY_FREQUENCY: f64 = 0.015;
    pub const POINTS: u32 = 10;

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
            score: GameScore::Score(0),
        }
    }

    fn background_stars_movement(&mut self) {
        self.background_stars.iter_mut().for_each(|star| star.fall())
    }

    fn generate_enemies(&mut self) {
        let mut random = thread_rng();
        
        self.enemies.retain(|enemy| {
            enemy.is_visible(self.screen_size) && !enemy.is_destroyed()
        });

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

    fn current_score(&self) -> u32 {
        return match self.score {
            GameScore::Score(score) => score,
            GameScore::GameOver(score) => score,
        };
    }

    fn update_score(&mut self)  { 
        let mut score = self.current_score();

        if self.score.is_game_over() {
            return;
        }

        for enemy in self.enemies.iter_mut() {
            if enemy.hits(&mut self.hero) {
                self.score = GameScore::GameOver(score);
                return;
            }

            if self.hero.hits(enemy) {
                score += Game::POINTS;
            }
        }

       self.score = GameScore::Score(score);
    }

    fn score_as_text(&self) -> Vec<GameText> {        
        return match self.score {
            GameScore::Score(points) => vec![
                GameText::new(
                    format!("Score: {}", points),
                    Game::SCORE_TEXT_COLOR,
                    Game::SCORE_TEXT_SIZE,
                    Game::SCORE_TEXT_POSITION
                )
            ],
            GameScore::GameOver(points) => {
                let (screen_x, screen_y) = self.screen_size;
                let game_over_pos_x = screen_x/2.0 - Game::GAME_OVER_POSITION_PADDING;
                let game_over_pos_y = screen_y/2.0;

                let final_score_pos_x = game_over_pos_x;
                let final_score_pos_y = screen_y/2.0 + Game::FINAL_SCORE_PADDING;

                vec![
                    GameText::new(
                        String::from("Game Over!"),
                        Game::SCORE_TEXT_COLOR,
                        Game::SCORE_TEXT_SIZE,
                        (game_over_pos_x, game_over_pos_y)
                    ),
                    GameText::new(
                        format!("Score: {}", points),
                        Game::SCORE_TEXT_COLOR,
                        Game::SCORE_TEXT_SIZE,
                        (final_score_pos_x, final_score_pos_y)
                    )                
                ]
            }
        };
    }

    pub fn next_turn(&mut self) {        
        self.background_stars_movement();
        self.generate_enemies();
        self.enemies_action();   
        self.player_action();
        
        self.update_score();
    }

    pub fn space_objects(&self) -> Vec<Box<& dyn SpaceObject>> {
        let mut objects:Vec<Box<& dyn SpaceObject>> = vec![];

        for star in &self.background_stars {            
            objects.push(Box::new(star));
        }

        for enemy in &self.enemies {
            objects.append(&mut enemy.spaceship().as_game_objects());         
        }

        if !self.hero.is_destroyed() {
            objects.append(&mut self.hero.spaceship().as_game_objects());
        }
        
        objects  
    }


    pub fn texts(&self) -> Vec<GameText> {
       self.score_as_text()
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.hero.move_to(direction);
    }

    pub fn fire_player_attack(&mut self) {
        self.hero.attack();        
    }      

}
