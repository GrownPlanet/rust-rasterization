// TODO:
// - fix bug if the cube is behind the camera
// - fix devide by 0 bug
// - add pitch

extern crate sdl2;

use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
};

use std::time::Duration;

use camera::{Camera, Dir};
use point::Point3D;
use triangle::Triangle3D;

mod camera;
mod point;
mod triangle;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rasterization", 800, 600)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let points = [
        (250, 250, 450),
        (250, -250, 450),
        (-250, -250, 450),
        (-250, 250, 450),
        (250, 250, 950),
        (250, -250, 950),
        (-250, -250, 950),
        (-250, 250, 950),
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

    let near = 300;
    let mut camera = Camera::new(Point3D::new(0, 0, -near), 0.);

    let speed = 15;

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

        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::W) {
            camera.move_dir(Dir::Forwards, speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            camera.move_dir(Dir::Backwards, speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            camera.move_dir(Dir::Left, speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            camera.move_dir(Dir::Right, speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Q) {
            camera.move_dir(Dir::Down, speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::E) {
            camera.move_dir(Dir::Up, speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            camera.rotate_yaw(0.2);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            camera.rotate_yaw(-0.2);
        }

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.clear();

        let (width, height) = canvas.output_size()?;

        let mut rast;
        for triangle in &triangles {
            rast = triangle.rasterize(near, &camera);
            rast.draw(&mut canvas, width, height)?;
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
