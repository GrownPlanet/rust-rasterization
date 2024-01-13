use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::camera::Camera;
use crate::point::Point3D;

pub struct Triangle2D {
    pub points: [Point; 3],
}

impl Triangle2D {
    pub fn _new((x1, y1): (i32, i32), (x2, y2): (i32, i32), (x3, y3): (i32, i32)) -> Self {
        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);
        let p3 = Point::new(x3, y3);

        Self {
            points: [p1, p2, p3],
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

        let p1 = self.points[0];
        let p2 = self.points[1];
        let p3 = self.points[2];

        let p1_2 = Point::new(p1.x + half_width, p1.y + half_height);
        let p2_2 = Point::new(p2.x + half_width, p2.y + half_height);
        let p3_2 = Point::new(p3.x + half_width, p3.y + half_height);

        canvas.draw_line(p1_2, p2_2)?;
        canvas.draw_line(p1_2, p3_2)?;
        canvas.draw_line(p2_2, p3_2)?;

        Ok(())
    }
}

pub struct Triangle3D {
    pub points: [Point3D; 3],
}

impl Triangle3D {
    pub fn new(
        (x1, y1, z1): (i32, i32, i32),
        (x2, y2, z2): (i32, i32, i32),
        (x3, y3, z3): (i32, i32, i32),
    ) -> Self {
        let p1 = Point3D::new(x1, y1, z1);
        let p2 = Point3D::new(x2, y2, z2);
        let p3 = Point3D::new(x3, y3, z3);

        Self {
            points: [p1, p2, p3],
        }
    }

    pub fn rasterize(&self, near: i32, camera: &Camera) -> Option<Triangle2D> {
        let moved_points = self.points.iter().map(|p| p.move_point(camera));
        let rotated_points = moved_points.map(|p| p.rotate_x(camera).rotate_y(camera));

        rotated_points.clone().filter(|p| p.z > 0).nth(1)?;

        let projected_points: Vec<Point> = rotated_points.map(|p| p.rasterize(near)).collect();

        Some(Triangle2D {
            points: projected_points.try_into().unwrap(),
        })
    }
}
