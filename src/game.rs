use rand::{thread_rng, Rng};

pub type Color = [f32; 4];
pub type Position = (f64, f64);
pub type Coord = Vec<[f64; 2]> ;
pub type ScreenSize = (f64, f64);

#[derive(Copy, Clone)]
pub enum Direction {
    LEFT, UP, RIGHT, DOWN
}

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

pub trait SpaceObject {
    fn color(&self) -> Color;
    fn direction(&self) -> Direction;
    fn position(&self) -> Position;
    fn size(&self) -> f64;
    fn coord(&self) -> Coord  {
        let (x, y) = self.position();
        let direction_y:f64 = match self.direction() {
            Direction::UP =>  y - self.size(),
            _ =>  y + self.size(),
        };

        vec![[x-self.size(), y], [x, direction_y], [x+self.size(), y]]
    }
}

pub struct Player {
    position: Position,
    screen_size: ScreenSize
}

impl Player {
    pub const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const SPEED: f64 = 30.0;
    pub const SIZE: f64 = 25.0;
    pub const DIRECTION: Direction = Direction::UP;

    pub fn new(screen_size: ScreenSize) -> Player {
        let (width, height) = screen_size;
        Player {
            position: (width/2.0, height-30.0),
            screen_size: screen_size
        }
    }

    pub fn move_to(&mut self, direction: Direction) {
        let (width, _) = self.screen_size;
        let (cur_x, cur_y) =  self.position;
        let new_x = match direction {
            Direction::LEFT => min(cur_x + Player::SPEED, width),
            _ => max(cur_x - Player::SPEED, 0.0),
        };
        self.position = (new_x, cur_y);
    }
}

impl SpaceObject for Player {
    
    fn color(&self) -> Color {
        Player::COLOR
    }

    fn direction(&self) -> Direction {
        Player::DIRECTION
    }

    fn size(&self) -> f64 {
        Player::SIZE
    }
    
    fn position(&self) -> Position {
        self.position
    }
}

#[derive(Copy, Clone)]
pub struct Star {
    size: f64,
    position: Position,
    screen_size: ScreenSize
}

impl Star {
    pub const COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const SPEED: f64 = 5.0;

    fn new(screen_size: ScreenSize) -> Star {
        let mut rng = thread_rng();
        
        let (width, height) = screen_size;
        let gen_x = rng.gen_range(0..(width as i32));
        let gen_y = rng.gen_range(0..(height as i32)); 
        let size:i32 = rng.gen_range(1..=4);

        Star {
            size: f64::from(size),
            position: (f64::from(gen_x), f64::from(gen_y)),
            screen_size: screen_size
        }
    }

    fn fall(&mut self) -> Star {
        let (_, height) = self.screen_size;
        let (cur_x, cur_y) = self.position();
        let new_y = if cur_y + Star::SPEED < height { cur_y + Star::SPEED } else { 0.0};

        Star {
            position: (cur_x, new_y),
            size: self.size(),
            screen_size: self.screen_size
        }
    }
}

impl SpaceObject for Star {
    
    fn color(&self) -> Color {
        Star::COLOR
    }

    fn direction(&self) -> Direction {
        Player::DIRECTION
    }

    fn size(&self) -> f64 {
        self.size
    }
    
    fn position(&self) -> Position {
        self.position
    }

    fn coord(&self) -> Coord {
        let (x, y) = self.position();
        let size = self.size();
        vec![[x, y], [x+size, y], [x+size, y+size], [x, y+size]]
    }
}

#[derive(Copy, Clone)]
pub struct Enemy {
    position: Position,
    screen_size: ScreenSize
}

impl Enemy {
    pub const COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const X_SPEED: f64 = 45.0;
    pub const Y_SPEED: f64 = 1.0;
    pub const Y_START: f64 = -30.0;    
    pub const SIZE: f64 = 20.0;
    pub const DIRECTION: Direction = Direction::DOWN;
    
    pub fn new(screen_size: ScreenSize)-> Enemy {
        let mut random = thread_rng();

        let (width, _) = screen_size;
        let min_x = Enemy::SIZE;
        let max_x = width - Enemy::SIZE;
        let gen_x = random.gen_range(min_x..max_x);       
       
       Enemy {
            position: (f64::from(gen_x), Enemy::Y_START),
            screen_size: screen_size
        }
    }

    pub fn movement(&mut self) {
        let (width, _) = self.screen_size;

        let new_x =  max(min(self.calculate_x_move(), width), 0.0);
        let new_y =  self.calculate_y_move();
        
        self.position = (new_x, new_y);
    }

    fn calculate_x_move(&self)-> f64 {
        let mut random = thread_rng();
        let (curr_x, _) = self.position();
            
        let movement = if  random.gen_bool(0.05) {
            let  move_range = 2.0 * Enemy::X_SPEED;
            random.gen_range(0.0..move_range) - Enemy::X_SPEED        
        } else {
            0.0
        };

        curr_x + movement
    }

    fn calculate_y_move(&self)-> f64 {
        let (_, curr_y) = self.position();
        curr_y + Enemy::Y_SPEED
    }
}

impl SpaceObject for Enemy {
    
    fn color(&self) -> Color {
        Enemy::COLOR
    }

    fn direction(&self) -> Direction {
        Enemy::DIRECTION
    }

    fn size(&self) -> f64 {
        Enemy::SIZE
    }
    
    fn position(&self) -> Position {
        self.position
    }
}




fn min(x:f64, y:f64) -> f64 {
    if x < y { x } else { y }
}

fn max(x:f64, y:f64) -> f64 {
    if x > y { x } else { y }
}