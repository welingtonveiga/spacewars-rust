use crate::graphics::{draw_background, draw_polygon, draw_text};
use spacewars_game::{Direction, Game};
use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};
use web_sys::CanvasRenderingContext2d;

const MOVE_RIGHT_KEY: &str = "ArrowRight";
const MOVE_LEFT_KEY: &str = "ArrowLeft";
const ATTACK_KEY: &str = " ";

#[derive(Clone)]
pub struct Presenter {
    game: Rc<RefCell<Game>>,
    context: Rc<CanvasRenderingContext2d>,
    last_key: Rc<RefCell<Option<String>>>,
}

impl Presenter {
    pub fn new(game: Game, context: CanvasRenderingContext2d) -> Presenter {
        Presenter {
            game: Rc::new(RefCell::new(game)),
            context: Rc::new(context),
            last_key: Rc::new(RefCell::new(None)),
        }
    }

    pub fn render(&self) {
        let mut game = self.game.borrow_mut();
        let context = self.context.borrow();

        self.handle_event(&mut game);

        game.next_turn();

        draw_background(context, game.screen_size());
        for object in game.space_objects().iter() {
            draw_polygon(object, context);
        }

        for text in game.texts().iter() {
            draw_text(text, context);
        }
    }

    pub fn last_key(&self, new_key: String) {
        RefCell::replace(&self.last_key, Some(new_key));
    }

    fn handle_event(&self, game: &mut Game) {
        if let Some(ref mut last_key) = *self.last_key.borrow_mut() {
            match last_key.as_str() {
                MOVE_RIGHT_KEY => game.move_player(Direction::RIGHT),
                MOVE_LEFT_KEY => game.move_player(Direction::LEFT),
                ATTACK_KEY => game.fire_player_attack(),
                _ => game.key_pressed(),
            }
        }

        RefCell::replace(&self.last_key, None);
    }
}
