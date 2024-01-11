use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::point::Point3D;

pub struct Triangle2D {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle2D {
    pub fn _new((x1, y1): (i32, i32), (x2, y2): (i32, i32), (x3, y3): (i32, i32)) -> Self {
        Self {
            p1: Point::new(x1, y1),
            p2: Point::new(x2, y2),
            p3: Point::new(x3, y3),
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let half_width = screen_width as i32 / 2;
        let half_height = screen_height as i32 / 2;

        let p1_2 = Point::new(self.p1.x + half_width, self.p1.y + half_height);
        let p2_2 = Point::new(self.p2.x + half_width, self.p2.y + half_height);
        let p3_2 = Point::new(self.p3.x + half_width, self.p3.y + half_height);

        canvas.draw_line(p1_2, p2_2)?;
        canvas.draw_line(p1_2, p3_2)?;
        canvas.draw_line(p2_2, p3_2)?;

        Ok(())
    }
}

pub struct Triangle3D {
    pub p1: Point3D,
    pub p2: Point3D,
    pub p3: Point3D,
}

impl Triangle3D {
    pub fn new(
        (x1, y1, z1): (i32, i32, i32),
        (x2, y2, z2): (i32, i32, i32),
        (x3, y3, z3): (i32, i32, i32),
    ) -> Self {
        Self {
            p1: Point3D::new(x1, y1, z1),
            p2: Point3D::new(x2, y2, z2),
            p3: Point3D::new(x3, y3, z3),
        }
    }

    pub fn rasterize(&self, near: i32, camera: &Point3D) -> Triangle2D {
        Triangle2D {
            p1: self.p1.rasterize(near, camera),
            p2: self.p2.rasterize(near, camera),
            p3: self.p3.rasterize(near, camera),
        }
    }
}
