

pub type Color = [f32; 4];
pub type Position = (f64, f64);
pub type Coord = Vec<[f64; 2]> ;

#[derive(Copy, Clone)]
pub enum Direction {
    LEFT, UP, RIGHT, DOWN
}

const HERO_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const PLAYER_SPEED: f64 = 30.0;

pub struct Game {
    width: f64, 
    height: f64,
    player: Spaceship,
}

impl Game {
    pub fn new(width: f64, height: f64) -> Game {
        Game {
            width: width,
            height: height,
            player: Spaceship::new(
                HERO_COLOR ,
                25.0,                    
                Direction::UP,
                (width/2.0, height-30.0)                    
            )
        }
    }

    pub fn objects(&self)-> Vec<Box<& dyn SpaceObject>> {
        vec![Box::new(&self.player)]
    }

    pub fn move_player(&mut self, direction: Direction) {
        let (cur_x, cur_y) =  self.player.position;
        let new_x = match direction {
            Direction::LEFT => min(cur_x + PLAYER_SPEED, self.width),
            _ => max(cur_x - PLAYER_SPEED, 0.0),
        };
        self.player.move_to((new_x, cur_y));
    }
}

pub trait SpaceObject {
    fn color(&self) -> Color;
    fn direction(&self) -> Direction;
    fn coord(&self) -> Coord;
}

pub struct Spaceship {
    size: f64,
    color: Color,
    direction: Direction,
    position: Position,
}

impl Spaceship {
    pub fn new(color: Color, size: f64, direction: Direction, position: Position) -> Spaceship {
        Spaceship {
            size : size,
            color: color,
            direction: direction,
            position: position
        }
    }

    pub fn move_to(&mut self, new_pos: Position) {
        self.position = new_pos;
    }

}

impl SpaceObject for Spaceship {
    
    fn color(&self) -> Color {
        self.color
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn coord(&self) -> Coord {
        let (x, y) = self.position;
        let direction_y:f64 = match self.direction {
            Direction::UP =>  y - self.size,
            _ =>  y + self.size,
        };

        vec![[x-self.size, y], [x, direction_y], [x+self.size, y]]
    }
}

fn min(x:f64, y:f64) -> f64 {
    if x < y { x } else { y }
}

fn max(x:f64, y:f64) -> f64 {
    if x > y { x } else { y }
}