use colorsys::Rgb;
use spacewars_game::Color;
use spacewars_game::{GameText, ScreenSize, SpaceObject};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const COLOR_MAX_RANGE: f32 = 255.0;
const GAME_FONT: &str = "Arial";
const BLACK_COLOR_HEX: &str = "#000000";

fn convert_color(color: Color) -> String {
    let [red, green, blue, alpha] = color;
    Rgb::from((
        red * COLOR_MAX_RANGE,
        green * COLOR_MAX_RANGE,
        blue * COLOR_MAX_RANGE,
        alpha,
    ))
    .to_hex_string()
}

pub fn draw_polygon(object: &Box<&dyn SpaceObject>, context: &CanvasRenderingContext2d) {
    context.set_fill_style(&JsValue::from(convert_color(object.color())));
    context.begin_path();

    for (i, [x, y]) in object.coord().iter().enumerate() {
        if i == 0 {
            context.move_to(*x, *y);
        } else {
            context.line_to(*x, *y);
        }
    }
    context.close_path();
    context.fill();
}

pub fn draw_text(text: &GameText, context: &CanvasRenderingContext2d) {
    let (x, y) = text.position();

    context.set_font(format!("{} {}", text.font_size(), GAME_FONT).as_str());
    context.set_fill_style(&JsValue::from(convert_color(text.color())));
    context
        .fill_text(text.content().as_str(), x, y)
        .expect("Error Filling Text");
}

pub fn draw_background(context: &CanvasRenderingContext2d, screen_size: ScreenSize) {
    let (width, height) = screen_size;
    context.set_fill_style(&JsValue::from(BLACK_COLOR_HEX));
    context.fill_rect(0.0, 0.0, width, height);
}
