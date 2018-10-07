extern crate piston;
extern crate piston_window;
extern crate find_folder;
extern crate sdl2_window;

use sdl2_window::*;
use piston::input::*;
use piston::event_loop::*;
use piston_window::*;

struct Rectangle {
    color: [f64; 4],
    shape: [f64; 4],
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

struct Shape {
    color: Color,
    x: f64,
    y: f64,
    height: f64,
    width: f64,
    destination: [f64; 2],
}

fn get_step([x, y]: [f64; 2], [dest_x, dest_y]: [f64; 2]) -> [f64; 2] {
    let mut distance_x = dest_x - x;
    let mut distance_y = dest_y - y;
    let distance: f64 = ((distance_x).powf(2f64) + (distance_y).powf(2f64)).sqrt();

    [distance / distance_y, distance / distance_x]
}

fn main() {
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Hello Piston!", [1000, 800])
            .resizable(false)
            .vsync(true)
            .fullscreen(true)
            .exit_on_esc(true).build().unwrap();
    let mut events = Events::new(EventSettings::new().lazy(false));
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    window.set_capture_cursor(false);

    let mut message = String::from("");
    let mut cursor: [f64; 2] = [0f64, 0f64];
    let mut rectanglee = Shape {
        color: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
        x: 0.0,
        y: 0.0,
        height: 100.0,
        width: 100.0,
        destination: [0.0, 0.0],
    };

    while let Some(e) = events.next(&mut window) {
        if rectanglee.destination[0] != rectanglee.x || rectanglee.destination[1] != rectanglee.y {
            let steps = get_step([rectanglee.x, rectanglee.y], [rectanglee.destination[0], rectanglee.destination[1]]);

            rectanglee.x = rectanglee.x + steps[0];
            rectanglee.y = rectanglee.y + steps[1];
        }

        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([rectanglee.color.r, rectanglee.color.g, rectanglee.color.b, rectanglee.color.a],
                      [rectanglee.x - (rectanglee.width / 2f64), rectanglee.y - (rectanglee.height / 2f64), rectanglee.height, rectanglee.width],
                      c.transform, g);
        });

        e.mouse_cursor(|x, y| {
            cursor = [x, y];
            println!("{:?} {:?}", x, y);
        });

        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
            if button == MouseButton::Left {
                message = format!("Left, {:?}, {:?}", cursor[0], cursor[1]);
                rectanglee.destination = cursor.clone();
                // rectanglee.x = cursor[0];
                // rectanglee.y = cursor[1];
            }
        }

        window.draw_2d(&e, |c, g| {
            let transform = c.transform.trans(100.0, 100.0);
            // Set a white background
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                &message,
                &mut glyphs,
                &c.draw_state,
                transform, g
            );
        });
    }
}
