extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use triangle::{Point3D, Triangle3D};

mod triangle;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let points1 = [
        (100, 100, 60),
        (500, 100, 60),
        (100, 500, 60),
        (500, 500, 60),
    ];

    let tr3d1 = Triangle3D::new(points1[0], points1[1], points1[2]);
    let tr3d2 = Triangle3D::new(points1[1], points1[2], points1[3]);

    let points2 = [
        (100, 100, 80),
        (500, 100, 80),
        (100, 500, 80),
        (500, 500, 80),
    ];

    let tr3d3 = Triangle3D::new(points2[0], points2[1], points2[2]);
    let tr3d4 = Triangle3D::new(points2[1], points2[2], points2[3]);

    let mut camera = Point3D::new(0, 0, 0);

    let near = 50;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.clear();

        let mut rast = tr3d1.rasterize(near);
        rast.draw(&mut canvas)?;
        rast = tr3d2.rasterize(near);
        rast.draw(&mut canvas)?;
        rast = tr3d3.rasterize(near);
        rast.draw(&mut canvas)?;
        rast = tr3d4.rasterize(near);
        rast.draw(&mut canvas)?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
