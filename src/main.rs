extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use graphics::{clear};

use crate::scene::Scene;
use crate::game::Game;
use crate::presenter::Presenter;

mod game;
mod presenter;
mod scene;

const WINDOW_WIDTH:f64 = 800.0;
const WINDOW_HEIGHT:f64 = 600.0;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Space Wars", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut gl = GlGraphics::new(opengl);

    // Create a new game and run it.
    let scene = Scene::new();
    let game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut presenter = Presenter::new(scene, game);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        presenter.event(&e);
        
        if let Some(args) = e.render_args() {
            
            gl.draw(args.viewport(), |c, g| {
                clear([0.0; 4], g);
                presenter.render(c, g);
            });           
        }
    }
}