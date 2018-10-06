extern crate piston;
extern crate piston_window;
extern crate find_folder;

use piston::input::*;
use piston::event_loop::*;
use piston_window::*;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    let mut events = Events::new(EventSettings::new().lazy(true));
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    window.set_capture_cursor(false);

    let mut message = "Hello";

    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([0.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 90.0, 90.0],
                      c.transform, g);
        });

        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
            message = "Left Button Pressed"
        }

        window.draw_2d(&e, |c, g| {
            let transform = c.transform.trans(100.0, 100.0);
            // Set a white background
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                message,
                &mut glyphs,
                &c.draw_state,
                transform, g
            );
        });
    }
}
