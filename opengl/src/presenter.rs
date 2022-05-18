use graphics::{text, Context, Polygon, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::{Button, GenericEvent, Key, MouseButton};

use spacewars_game::{Direction, Game};


pub struct Presenter {
    game: Game,
}


impl Presenter {
    pub fn new(game: Game) -> Presenter {
        Presenter { game: game }
    }

    pub fn render(&mut self, context: Context, graphics: &mut GlGraphics, glyphs: &mut GlyphCache) {
        self.game.next_turn();

        for object in self.game.space_objects().iter() {
            Polygon::new(object.color()).draw(
                &object.coord(),
                &context.draw_state,
                context.transform,
                graphics,
            );
        }

        for text in self.game.texts().iter() {
            let (position_x, position_y) = text.position();
            text::Text::new_color(text.color(), text.font_size())
                .draw(
                    &text.content(),
                    glyphs,
                    &context.draw_state,
                    context.trans(position_x, position_y).transform,
                    graphics,
                )
                .unwrap();
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.fire_attack();
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.key_pressed();

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

    pub fn key_pressed(&mut self) {
        self.game.key_pressed()
    }
}