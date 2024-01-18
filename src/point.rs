use crate::camera::Camera;

pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn as_tuple(&self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn move_point(&self, camera: &Camera) -> Self {
        Self {
            x: self.x - camera.location.x,
            y: self.y - camera.location.y,
            z: self.z - camera.location.z,
        }
    }

    pub fn rasterize(&self, near: i32) -> Point2D {
        let mut z = self.z;
        if z == 0 {
            z = 1;
        }

        let screenx = (near * self.x) / z;
        let screeny = (near * self.y) / z;

        Point2D::new(screenx, screeny)
    }

    pub fn rotate_x(&self, camera: &Camera) -> Self {
        let yaw = camera.yaw;

        let rotated_x = (self.x as f32 * yaw.cos() - self.z as f32 * yaw.sin()) as i32;
        let rotated_z = (self.z as f32 * yaw.cos() + self.x as f32 * yaw.sin()) as i32;

        Self {
            x: rotated_x,
            y: self.y,
            z: rotated_z,
        }
    }
    pub fn rotate_y(&self, camera: &Camera) -> Self {
        let pitch = camera.pitch;

        let rotated_y = (self.y as f32 * pitch.cos() - self.z as f32 * pitch.sin()) as i32;
        let rotated_z = (self.z as f32 * pitch.cos() + self.y as f32 * pitch.sin()) as i32;

        Self {
            x: self.x,
            y: rotated_y,
            z: rotated_z,
        }
    }
}
