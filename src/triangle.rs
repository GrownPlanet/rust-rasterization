use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Triangle2D {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle2D {
    pub fn new((x1, y1): (i32, i32), (x2, y2): (i32, i32), (x3, y3): (i32, i32)) -> Self {
        Self {
            p1: Point::new(x1, y1),
            p2: Point::new(x2, y2),
            p3: Point::new(x3, y3),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_line(self.p1, self.p2)?;
        canvas.draw_line(self.p1, self.p3)?;
        canvas.draw_line(self.p2, self.p3)?;
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

    pub fn rasterize(&self, near: i32) -> Triangle2D {
        Triangle2D {
            p1: self.p1.rasterize(near),
            p2: self.p2.rasterize(near),
            p3: self.p3.rasterize(near),
        }
    }
}

pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn rasterize(&self, near: i32) -> Point {
        let screenx = (near * self.x) / self.z;
        let screeny = (near * self.y) / self.z;
        Point::new(screenx, screeny)
    }
}
