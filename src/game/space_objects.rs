pub type Color = [f32; 4];
pub type Position = (f64, f64);
pub type Coord = Vec<[f64; 2]> ;
pub type ScreenSize = (f64, f64);

#[derive(Copy, Clone)]
pub enum Direction {
    LEFT, UP, RIGHT, DOWN
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

pub fn min(x:f64, y:f64) -> f64 {
    if x < y { x } else { y }
}

pub fn max(x:f64, y:f64) -> f64 {
    if x > y { x } else { y }
}