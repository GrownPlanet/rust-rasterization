use notan::draw::*;
use notan::prelude::*;

mod camera;
mod point;
mod triangle;

use crate::camera::Camera;
use crate::camera::Dir;
use crate::point::Point3D;
use crate::triangle::Triangle2D;
use crate::triangle::Triangle3D;

#[derive(AppState)]
struct State {
    triangles: Vec<Triangle3D>,
    projected_triangles: Vec<Triangle2D>,
    camera: Camera,
    speed: i32,
    rotation_speed: f32,
}

impl State {
    fn new() -> Self {
        let points = [
            (250, 250, 450),
            (250, -250, 450),
            // (-250, -250, 450),
            (-250, 250, 450),
            // (250, 250, 950),
            // (250, -250, 950),
            // (-250, -250, 950),
            // (-250, 250, 950),
        ];

        let triangles = vec![
            Triangle3D::new(points[0], points[1], points[2]),
            // Triangle3D::new(points[0], points[1], points[3]),
            // Triangle3D::new(points[1], points[2], points[3]),
            // Triangle3D::new(points[0], points[1], points[4]),
            // Triangle3D::new(points[1], points[4], points[5]),
            // Triangle3D::new(points[1], points[5], points[6]),
            // Triangle3D::new(points[2], points[5], points[6]),
            // Triangle3D::new(points[2], points[3], points[7]),
            // Triangle3D::new(points[2], points[6], points[7]),
            // Triangle3D::new(points[0], points[3], points[7]),
            // Triangle3D::new(points[0], points[4], points[7]),
            // Triangle3D::new(points[4], points[5], points[7]),
            // Triangle3D::new(points[5], points[6], points[7]),
        ];

        let near = 500;
        let camera = Camera::new(Point3D::new(0, 0, -near), 0., 0., near);

        let speed = 15;
        let rotation_speed = 0.1;
        Self {
            triangles,
            projected_triangles: vec![],
            camera,
            speed,
            rotation_speed,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new()
        .set_title("rasterization")
        .set_resizable(true)
        .set_size(800, 600)
        .set_vsync(true);

    notan::init_with(State::new)
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::from_rgb(200., 200., 200.));

    for t in &state.projected_triangles {
        t.draw(&mut draw, 800, 600).unwrap();
    }

    gfx.render(&draw);
}

fn update(app: &mut App, state: &mut State) {
    state.projected_triangles = state
        .triangles
        .iter()
        .flat_map(|t| t.rasterize(&state.camera))
        .collect();

    let keyboard = &app.keyboard;

    if keyboard.was_pressed(KeyCode::W) {
        state.camera.move_dir(Dir::Forwards, state.speed);
    }
    if keyboard.was_pressed(KeyCode::S) {
        state.camera.move_dir(Dir::Backwards, state.speed);
    }
    if keyboard.was_pressed(KeyCode::A) {
        state.camera.move_dir(Dir::Left, state.speed);
    }
    if keyboard.was_pressed(KeyCode::D) {
        state.camera.move_dir(Dir::Right, state.speed);
    }
    if keyboard.was_pressed(KeyCode::Q) {
        state.camera.move_dir(Dir::Down, state.speed);
    }
    if keyboard.was_pressed(KeyCode::E) {
        state.camera.move_dir(Dir::Up, state.speed);
    }
    if keyboard.was_pressed(KeyCode::Right) {
        state.camera.rotate_yaw(state.rotation_speed);
    }
    if keyboard.was_pressed(KeyCode::Left) {
        state.camera.rotate_yaw(-state.rotation_speed);
    }
    if keyboard.was_pressed(KeyCode::Up) {
        state.camera.rotate_pitch(state.rotation_speed);
    }
    if keyboard.was_pressed(KeyCode::Down) {
        state.camera.rotate_pitch(-state.rotation_speed);
    }
}
