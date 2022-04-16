
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::{Context, Polygon};
use opengl_graphics::{GlGraphics};

use crate::game::{SpaceObject};

pub struct Scene{    
}

impl Scene{

    pub fn new() -> Scene {
        Scene {            
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut GlGraphics, objects: Vec<Box<& dyn SpaceObject>>) {
        for object in objects.iter() {            
            Polygon::new(object.color())
                .draw(&object.coord(), &context.draw_state, context.transform, graphics);   
        }       
    }
}
