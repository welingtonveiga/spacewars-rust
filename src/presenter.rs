
use piston::input::{GenericEvent, Button, Key, MouseButton};
use graphics::{Context};
use opengl_graphics::{GlGraphics};

use crate::game::{Game, Direction};
use crate::scene::Scene;


pub struct Presenter {
    scene:  Scene,  
    game:  Game,      
}

impl Presenter {

    pub fn new( scene: Scene, game: Game) -> Presenter {
        Presenter {
            scene: scene,
            game: game
        }
    }

    pub fn render(&mut self, context: Context, graphics: &mut GlGraphics) {        
        self.scene.draw(context, graphics, self.game.next_tick());
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {     
            // Find coordinates relative to upper left corner.
            println!("Mouse Click!");
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            println!("Keyboard Click!");
            println!("{:#?}", key);
            match key {
                Key::Right => self.move_hero(Direction::LEFT),
                Key::Left => self.move_hero(Direction::RIGHT),               
                _ => {}
            }
        }
    }

    pub fn move_hero(&mut self, direction: Direction) {
        self.game.move_player(direction);
    }
}

