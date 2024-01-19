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
    speed: f32,
    rotation_speed: f32,
}

impl State {
    fn new() -> Self {
        let points = [
            (250., 250., 450.),
            (250., -250., 450.),
            (-250., -250., 450.),
            (-250., 250., 450.),
            (250., 250., 950.),
            (250., -250., 950.),
            (-250., -250., 950.),
            (-250., 250., 950.),
        ];

        let colors = [
            Color::from_rgb(1., 1., 1.),
            Color::from_rgb(1., 0., 0.),
            Color::from_rgb(0., 0., 1.),
            Color::from_rgb(1., 0.5, 0.),
            Color::from_rgb(0., 1., 0.),
            Color::from_rgb(1., 1., 0.),
        ];

        let triangles = vec![
            // side0
            Triangle3D::new(points[0], points[1], points[3], colors[0]),
            Triangle3D::new(points[1], points[2], points[3], colors[0]),
            // side1
            Triangle3D::new(points[0], points[1], points[4], colors[1]),
            Triangle3D::new(points[1], points[4], points[5], colors[1]),
            // side2
            Triangle3D::new(points[1], points[5], points[6], colors[2]),
            Triangle3D::new(points[1], points[2], points[6], colors[2]),
            // side3
            Triangle3D::new(points[2], points[3], points[7], colors[3]),
            Triangle3D::new(points[2], points[6], points[7], colors[3]),
            // side4
            Triangle3D::new(points[0], points[3], points[7], colors[4]),
            Triangle3D::new(points[0], points[4], points[7], colors[4]),
            // side5
            Triangle3D::new(points[4], points[5], points[7], colors[5]),
            Triangle3D::new(points[5], points[6], points[7], colors[5]),
        ];

        let near = 500.;
        let camera = Camera::new(Point3D::new(0., 0., -near), 0., 0., near);

        let speed = 15.;
        let rotation_speed = 0.05;

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

    draw.clear(Color::from_rgb(0.8, 0.8, 0.8));

    state.projected_triangles.sort_by_key(|t1| t1.depth as i32);

    for t in &state.projected_triangles {
        t.draw(&mut draw).unwrap();
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

    if keyboard.is_down(KeyCode::W) {
        state.camera.move_dir(Dir::Forwards, state.speed);
    }
    if keyboard.is_down(KeyCode::S) {
        state.camera.move_dir(Dir::Backwards, state.speed);
    }
    if keyboard.is_down(KeyCode::A) {
        state.camera.move_dir(Dir::Left, state.speed);
    }
    if keyboard.is_down(KeyCode::D) {
        state.camera.move_dir(Dir::Right, state.speed);
    }
    if keyboard.is_down(KeyCode::Q) {
        state.camera.move_dir(Dir::Down, state.speed);
    }
    if keyboard.is_down(KeyCode::E) {
        state.camera.move_dir(Dir::Up, state.speed);
    }
    if keyboard.is_down(KeyCode::Right) {
        state.camera.rotate_yaw(state.rotation_speed);
    }
    if keyboard.is_down(KeyCode::Left) {
        state.camera.rotate_yaw(-state.rotation_speed);
    }
    if keyboard.is_down(KeyCode::Up) {
        state.camera.rotate_pitch(state.rotation_speed);
    }
    if keyboard.is_down(KeyCode::Down) {
        state.camera.rotate_pitch(-state.rotation_speed);
    }
}
