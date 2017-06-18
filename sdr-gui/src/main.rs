extern crate piston;
extern crate piston_window;

use piston::input::Input;
use piston_window::*;

fn render(window: &mut PistonWindow, e: Input) {
    let size = window.size();

    window.draw_2d(&e, |c, g| {
        clear([0.0, 0.0, 0.0, 1.0], g);

        let (w, h) = (size.width as f64, size.height as f64);

        rectangle(
            [
                0.1,
                0.6,
                0.3,
                1.0
            ],
            [
                0.0,
                0.0,
                w / 2.0,
                h / 2.0
            ],
            c.transform, g
        );

        rectangle(
            [
                0.1,
                0.6,
                0.3,
                1.0
            ],
            [
                w / 2.0,
                h / 2.0,
                w / 2.0,
                h / 2.0
            ],
            c.transform, g
        );
    });
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true)
            .samples(4)
            .build()
            .unwrap();

    while let Some(e) = window.next() {
        // println!("{:?}", &e);
        match e {
            Input::Render(_) => {
                render(&mut window, e)
            },
            Input::Update(_) => {
                render(&mut window, e)
            },
            _ => {}
        }
    }
}
