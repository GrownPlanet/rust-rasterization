use crate::point::Point3D;

pub struct Camera {
    pub location: Point3D,
    pub yaw: f32,
    // pub pitch: f32,
}

impl Camera {
    pub fn new(location: Point3D, yaw: f32) -> Self {
        Self { location, yaw }
    }
}
