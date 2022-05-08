extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings, Filter};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use graphics::{clear};

use crate::game::Game;
use crate::presenter::Presenter;

mod game;
mod presenter;

const WINDOW_WIDTH:f64 = 800.0;
const WINDOW_HEIGHT:f64 = 600.0;
const FRAMES_PER_SECOND:u64 = 40;


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Space Wars", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .fullscreen(false)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut gl = GlGraphics::new(opengl);

    // Create a new game and run it.
    let game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut presenter = Presenter::new(game);

    let mut event_settings = EventSettings::new();
    event_settings.max_fps = FRAMES_PER_SECOND;

    let mut events = Events::new(event_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/NovaSquare-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        presenter.event(&e);

        if let Some(args) = e.render_args() {            
            gl.draw(args.viewport(), |c, g| {
                clear([0.0; 4], g);
                presenter.render(c, g, glyphs);
            });           
        }
    }
}