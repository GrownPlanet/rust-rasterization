use sdl2::rect::Point;

pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn rasterize(&self, near: i32, camera: &Point3D) -> Point {
        let screenx = (near * (self.x - camera.x)) / (self.z - camera.z);
        let screeny = (near * (self.y - camera.y)) / (self.z - camera.z);
        Point::new(screenx, screeny)
    }
}
