
use wasm_bindgen::prelude::*;

use crate::presenter::Presenter;
use spacewars_game::Game;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

mod graphics;
mod presenter;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;
const FRAMES_PER_SECOND: f64 = 40.0;
const ONE_SECOND: f64 = 1000.0;
const FPS_INTERVAL: f64 = ONE_SECOND / FRAMES_PER_SECOND;
const KEYBOARD_EVENT: &str = "keydown";

use std::panic;

// https://github.com/koute/stdweb/blob/master/examples/todomvc/src/main.rs#L31-L39
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn add_event_listener_with_callback(event_type: &str, f: &Closure<dyn FnMut(KeyboardEvent)>) {
    window()
        .add_event_listener_with_callback(event_type, f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn now() -> f64 {
    js_sys::Date::now()
}

fn keyborad_handling(presenter: &Presenter) {
    let keyboard_handler = Closure::wrap(Box::new(
        enclose!( (presenter) move |event: web_sys::KeyboardEvent| {
            presenter.last_key(event.key());
        }),
    ) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    add_event_listener_with_callback(KEYBOARD_EVENT, &keyboard_handler);

    keyboard_handler.forget();
}

fn game_loop(presenter: &Presenter, last_frame: f64) {
    let mut next_frame_time = last_frame;
    let curr_frame = now();
    if curr_frame - last_frame >= FPS_INTERVAL {
        next_frame_time = curr_frame;
        presenter.render();
    }

    let callack = Closure::wrap(Box::new(enclose!((presenter) move || {
            game_loop(&presenter, next_frame_time);
    })) as Box<dyn FnMut()>);

    request_animation_frame(&callack);

    callack.forget();
}

#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let presenter = Presenter::new(game, context);

    keyborad_handling(&presenter);

    game_loop(&presenter, now());
}
