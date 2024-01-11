extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use point::Point3D;
use triangle::Triangle3D;

mod point;
mod triangle;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rasterization", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let points = [
        (250, 250, 60),
        (250, -250, 60),
        (-250, -250, 60),
        (-250, 250, 60),
        (250, 250, 90),
        (250, -250, 90),
        (-250, -250, 90),
        (-250, 250, 90),
    ];

    let triangles = [
        Triangle3D::new(points[0], points[1], points[3]),
        Triangle3D::new(points[1], points[2], points[3]),
        Triangle3D::new(points[0], points[1], points[4]),
        Triangle3D::new(points[1], points[4], points[5]),
        Triangle3D::new(points[1], points[5], points[6]),
        Triangle3D::new(points[2], points[5], points[6]),
        Triangle3D::new(points[2], points[3], points[7]),
        Triangle3D::new(points[2], points[6], points[7]),
        Triangle3D::new(points[0], points[3], points[7]),
        Triangle3D::new(points[0], points[4], points[7]),
        Triangle3D::new(points[4], points[5], points[7]),
        Triangle3D::new(points[5], points[6], points[7]),
    ];

    let mut camera = Point3D::new(0, 0, 0);
    let speed = 5;

    let near = 50;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => camera.y -= speed,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => camera.y += speed,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => camera.x -= speed,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => camera.x += speed,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.clear();

        let mut rast;
        for triangle in &triangles {
            rast = triangle.rasterize(near, &camera);
            rast.draw(&mut canvas, 800, 600)?;
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
