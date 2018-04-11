extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("dojo", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0], g);
        });
    }
}
