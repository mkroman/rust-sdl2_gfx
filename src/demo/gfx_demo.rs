extern crate sdl2;
extern crate sdl2_gfx;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keycode::KeyCode;

use sdl2_gfx::primitives::DrawRenderer;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
);

fn main() {

    let mut context = sdl2::init().video().unwrap();

    let window = context.window("rust-sdl2_gfx: draw line & FPSManager", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    {
        let mut drawer = renderer.drawer();
        drawer.set_draw_color(pixels::Color::RGB(0, 0, 0));
        drawer.clear();
        drawer.present();
    }

    let mut lastx = 0;
    let mut lasty = 0;

    let mut events = context.event_pump();

    'main: loop {
        for event in events.poll_iter() {

            match event {

                Event::Quit {..} => break 'main,

                Event::KeyDown {keycode, ..} => {
                    if keycode == KeyCode::Escape {
                        break 'main
                    } else if keycode == KeyCode::Space {
                        println!("space down");
                        for i in 0..400 {
                            renderer.pixel(i as i16, i as i16, 0xFF000FFu32).unwrap();
                        }
                        renderer.drawer().present();

                    }
                }

                Event::MouseButtonDown {x, y, ..} => {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    let _ = renderer.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    renderer.drawer().present();
                }

                _ => {}
            }
        }
    }
}
