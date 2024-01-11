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

    pub fn move_dir(&mut self, direction: Dir, speed: i32) {
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
                let rotations = self.get_rotations(speed, 90.);

                self.location.x += rotations.1;
                self.location.z += rotations.0;
            }
            Dir::Left => {
                let rotations = self.get_rotations(speed, -90.);

                self.location.x += rotations.1;
                self.location.z += rotations.0;
            }
            Dir::Up => self.location.y += speed,
            Dir::Down => self.location.y -= speed,
            // _ => panic!("Direction not implemented yet!"),
        }
    }

    fn get_rotations(&self, speed: i32, offset: f32) -> (i32, i32) {
        let x = (self.yaw + offset).cos() * speed as f32;
        let y = (self.yaw + offset).sin() * speed as f32;

        (x as i32, y as i32)
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
