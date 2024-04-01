mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coord_u32;
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
fn main() {
    let (width, height) = (20, 20);
    let mut window: PistonWindow = WindowSettings::new(
        "comedor de maçazinha",
        [to_coord_u32(width), to_coord_u32(height)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        let mut glyphs = window.load_font("assets/coolvetica.otf").unwrap();

        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, d| {
            clear(BACK_COLOR, g);
            game.draw(&c, g, &mut glyphs);
            glyphs.factory.encoder.flush(d);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
