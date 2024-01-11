use sdl2::rect::Point;

use crate::camera::Camera;

pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn rasterize(&self, near: i32, camera: &Camera) -> Point {
        let rotated = self.rotate(camera);

        let screenx = (near * (rotated.x - camera.location.x)) / (rotated.z - camera.location.z);
        let screeny = (near * (rotated.y - camera.location.y)) / (rotated.z - camera.location.z);

        Point::new(screenx, screeny)
    }

    fn rotate(&self, camera: &Camera) -> Self {
        let yaw = camera.yaw;
        let x = (self.x - camera.location.x) as f32;
        let z = (self.z - camera.location.z) as f32;

        let rotated_x = (x * yaw.cos() - z * yaw.sin()) as i32;
        let rotated_z = (z * yaw.cos() + x * yaw.sin()) as i32;

        Self {
            x: rotated_x,
            y: self.y,
            z: rotated_z,
        }
    }
}
