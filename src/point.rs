use crate::camera::Camera;

pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn move_point(&self, camera: &Camera) -> Self {
        Self {
            x: self.x - camera.location.x,
            y: self.y - camera.location.y,
            z: self.z - camera.location.z,
        }
    }

    pub fn rasterize(&self, near: f32) -> Point2D {
        let mut z = self.z;
        if z == 0. {
            z = 1.;
        }

        let screenx = (near * self.x) / z;
        let screeny = (near * self.y) / z;

        Point2D::new(screenx, screeny)
    }

    pub fn rotate_x(&self, camera: &Camera) -> Self {
        let yaw = camera.yaw;

        let rotated_x = self.x * yaw.cos() - self.z * yaw.sin();
        let rotated_z = self.z * yaw.cos() + self.x * yaw.sin();

        Self {
            x: rotated_x,
            y: self.y,
            z: rotated_z,
        }
    }
    pub fn rotate_y(&self, camera: &Camera) -> Self {
        let pitch = camera.pitch;

        let rotated_y = self.y * pitch.cos() - self.z * pitch.sin();
        let rotated_z = self.z * pitch.cos() + self.y * pitch.sin();

        Self {
            x: self.x,
            y: rotated_y,
            z: rotated_z,
        }
    }
}
