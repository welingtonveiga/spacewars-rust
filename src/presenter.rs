
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
        self.game.next_turn();
        self.scene.draw(context, graphics, self.game.as_space_objects());
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {     
            self.fire_attack();
        }
        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Right => self.move_player(Direction::LEFT),
                Key::Left => self.move_player(Direction::RIGHT),   
                Key::Space => self.fire_attack(),               
                _ => {}
            }
        }
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.game.move_player(direction);
    }

    pub fn fire_attack(&mut self) {
        self.game.fire_player_attack();
    }
}

