use crate::point::Point3D;

pub struct Camera {
    pub location: Point3D,
    pub yaw: f32,
    pub pitch: f32,
    pub near: f32,
}

impl Camera {
    pub fn new(location: Point3D, yaw: f32, pitch: f32, near: f32) -> Self {
        Self {
            location,
            yaw,
            pitch,
            near,
        }
    }

    pub fn move_dir(&mut self, direction: Dir, speed: f32) {
        match direction {
            Dir::Forwards => {
                let rotations = self.get_rotations(speed, 0.);

                self.location.x += rotations.1;
                self.location.z += rotations.0;
            }
            Dir::Backwards => {
                let rotations = self.get_rotations(speed, 0.);

                self.location.x -= rotations.1;
                self.location.z -= rotations.0;
            }
            Dir::Right => {
                let rotations = self.get_rotations(speed, std::f32::consts::PI / 2.);

                self.location.x += rotations.1;
                self.location.z += rotations.0;
            }
            Dir::Left => {
                let rotations = self.get_rotations(speed, std::f32::consts::PI / 2.);

                self.location.x -= rotations.1;
                self.location.z -= rotations.0;
            }
            Dir::Up => self.location.y -= speed,
            Dir::Down => self.location.y += speed,
        }
    }

    pub fn rotate_yaw(&mut self, dir: f32) {
        self.yaw += dir;

        if self.yaw > std::f32::consts::PI * 2. {
            self.yaw = 0.;
        } else if self.yaw < 0. {
            self.yaw = std::f32::consts::PI * 2.;
        }
    }
    pub fn rotate_pitch(&mut self, dir: f32) {
        self.pitch -= dir;

        if self.pitch > std::f32::consts::PI * 2. {
            self.pitch = 0.;
        } else if self.pitch < 0. {
            self.pitch = std::f32::consts::PI * 2.;
        }
    }

    fn get_rotations(&self, speed: f32, offset: f32) -> (f32, f32) {
        let x = (self.yaw + offset).cos() * speed;
        let y = (self.yaw + offset).sin() * speed;

        (x, y)
    }
}

pub enum Dir {
    Forwards,
    Backwards,
    Left,
    Right,
    Up,
    Down,
}
