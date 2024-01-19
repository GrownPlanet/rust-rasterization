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
        //            |
        //  0---------|---------1
        //  |         |         |
        //  |   4-----|-----5   |
        //  |   |     |     |   |
        //  |   |     |     |   |
        // -----------+------------> x
        //  |   |     |     |   |
        //  |   |     |     |   |
        //  |   7-----|-----6   |
        //  |         |         |
        //  3---------|---------2
        //            V y

        let points = [
            (-250., -250., -250.), // 0
            (250., -250., -250.),  // 1
            (250., 250., -250.),   // 2
            (-250., 250., -250.),  // 3
            (-250., -250., 250.),  // 4
            (250., -250., 250.),   // 5
            (250., 250., 250.),    // 6
            (-250., 250., 250.),   // 7
        ];

        let colors = [
            Color::from_rgb(1., 1., 1.),  // white
            Color::from_rgb(1., 1., 0.),  // yellow
            Color::from_rgb(0., 0., 1.),  // blue
            Color::from_rgb(0., 1., 0.),  // green
            Color::from_rgb(1., 0.5, 0.), // oragne
            Color::from_rgb(1., 0., 0.),  // red
        ];

        let verts = [
            // front
            (0, 1, 2, 0),
            (0, 2, 3, 0),
            // back
            (4, 5, 6, 1),
            (4, 6, 7, 1),
            // right
            (5, 1, 2, 2),
            (5, 2, 6, 2),
            // left
            (0, 4, 3, 3),
            (4, 7, 3, 3),
            // top
            (0, 1, 4, 4),
            (4, 1, 5, 4),
            // bottom
            (7, 2, 3, 5),
            (7, 6, 2, 5),
        ];

        let triangles = verts
            .iter()
            .map(|v| Triangle3D::new(points[v.0], points[v.1], points[v.2], colors[v.3]))
            .collect();

        let near = 500.;
        let camera = Camera::new(Point3D::new(0., 0., -1000.), 0., 0., near);

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

    let s = draw.size();
    draw.circle(2.5)
        .position(s.0 / 2., s.1 / 2.)
        .color(Color::from_rgb(0., 0., 0.));

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
