use rand::{thread_rng, Rng};

use crate::enemy::*;
use crate::game_objects::*;
pub use crate::game_objects::{Color, Direction, SpaceObject};
use crate::hero::*;
use crate::player::*;
use crate::stars::*;

mod enemy;
mod game_objects;
mod hero;
mod player;
mod shot;
mod spaceship;
mod stars;

type GameScore = u32;

#[derive(Copy, Clone)]
enum Scene {
    StartGame,
    InGame,
    GameOver,
}

impl Scene {
    fn is_in_game(&self) -> bool {
        match *self {
            Scene::InGame => true,
            _ => false,
        }
    }

    fn is_game_over(&self) -> bool {
        match *self {
            Scene::GameOver => true,
            _ => false,
        }
    }
}

pub struct Game {
    screen_size: ScreenSize,
    hero: Hero,
    enemies: Vec<Enemy>,
    background_stars: Vec<Star>,
    score: GameScore,
    scene: Scene,
    count: u64,
}

impl Game {
    pub const STAR_COUNT: i32 = 100;
    pub const TEXT_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
    pub const TEXT_SIZE: u32 = 32;
    pub const SCORE_TEXT_POSITION: Position = (30.0, 30.0);
    pub const GAME_OVER_POSITION_LEFT_PADDING: f64 = 100.0;
    pub const FINAL_SCORE_LEFT_PADDING: f64 = 80.0;
    pub const FINAL_SCORE_TOP_PADDING: f64 = 32.0;
    pub const START_GAME_TEXT_PADDING: f64 = 200.0;
    pub const ENEMY_FREQUENCY: f64 = 0.015;
    pub const POINTS: u32 = 10;

    pub fn new(width: f64, height: f64) -> Game {
        let screen_size = (width, height);

        let stars = (0..Game::STAR_COUNT)
            .map(|_| -> Star { Star::new(screen_size) })
            .collect();

        Game {
            screen_size: screen_size,
            hero: Hero::new(screen_size),
            enemies: Vec::new(),
            background_stars: stars,
            score: 0,
            scene: Scene::StartGame,
            count: 0,
        }
    }

    pub fn next_turn(&mut self) {
        self.inc_counter();
        self.background_stars_movement();

        if self.scene.is_in_game() {
            self.generate_enemies();
            self.enemies_action();
            self.player_action();

            self.update_score();
        }
    }

    pub fn space_objects(&self) -> Vec<Box<&dyn SpaceObject>> {
        let mut objects: Vec<Box<&dyn SpaceObject>> = vec![];

        for star in &self.background_stars {
            objects.push(Box::new(star));
        }

        if self.scene.is_in_game() {
            for enemy in &self.enemies {
                objects.append(&mut enemy.spaceship().as_game_objects());
            }

            if !self.hero.is_destroyed() {
                objects.append(&mut self.hero.spaceship().as_game_objects());
            }
        }

        objects
    }

    pub fn texts(&self) -> Vec<GameText> {
        return match self.scene {
            Scene::InGame => self.in_game_text(),
            Scene::StartGame => self.start_game_text(),
            Scene::GameOver => self.game_over_text(),
        };
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.hero.move_to(direction);
    }

    pub fn fire_player_attack(&mut self) {
        self.hero.attack();
    }

    pub fn key_pressed(&mut self) {
        match self.scene {
            Scene::StartGame => {
                self.scene = Scene::InGame;
            }
            _ => {}
        }
    }

    fn background_stars_movement(&mut self) {
        self.background_stars
            .iter_mut()
            .for_each(|star| star.fall())
    }

    fn generate_enemies(&mut self) {
        let mut random = thread_rng();

        self.enemies
            .retain(|enemy| enemy.is_visible(self.screen_size) && !enemy.is_destroyed());

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

    fn update_score(&mut self) {
        if self.scene.is_game_over() {
            return;
        }

        for enemy in self.enemies.iter_mut() {
            if enemy.hits(&mut self.hero) {
                self.scene = Scene::GameOver;
                return;
            }

            if self.hero.hits(enemy) {
                self.score += Game::POINTS;
            }
        }
    }

    fn game_over_text(&self) -> Vec<GameText> {
        let (screen_x, screen_y) = self.screen_size;
        let game_over_pos_x = screen_x / 2.0 - Game::GAME_OVER_POSITION_LEFT_PADDING;
        let game_over_pos_y = screen_y / 2.0;

        let final_score_pos_x = screen_x / 2.0 - Game::FINAL_SCORE_LEFT_PADDING;
        let final_score_pos_y = screen_y / 2.0 + Game::FINAL_SCORE_TOP_PADDING;

        vec![
            GameText::new(
                String::from("Game Over!"),
                Game::TEXT_COLOR,
                Game::TEXT_SIZE,
                (game_over_pos_x, game_over_pos_y),
            ),
            GameText::new(
                format!("Score: {}", self.score),
                Game::TEXT_COLOR,
                Game::TEXT_SIZE,
                (final_score_pos_x, final_score_pos_y),
            ),
        ]
    }

    fn in_game_text(&self) -> Vec<GameText> {
        vec![GameText::new(
            format!("Score: {}", self.score),
            Game::TEXT_COLOR,
            Game::TEXT_SIZE,
            Game::SCORE_TEXT_POSITION,
        )]
    }

    fn start_game_text(&self) -> Vec<GameText> {
        let show = self.count % 20 < 15;

        return if show {
            let (screen_x, screen_y) = self.screen_size;

            vec![GameText::new(
                String::from("Press Any Button to Start..."),
                Game::TEXT_COLOR,
                Game::TEXT_SIZE,
                (
                    screen_x / 2.0 - Game::START_GAME_TEXT_PADDING,
                    screen_y / 2.0,
                ),
            )]
        } else {
            Vec::new()
        };
    }

    fn inc_counter(&mut self) {
        let new_count = self.count.checked_add(1);
        match new_count {
            Some(result) => self.count = result,
            None => self.count = 0,
        }
    }
}
