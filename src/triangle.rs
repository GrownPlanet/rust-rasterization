use notan::draw::{Draw, DrawShapes};
use notan::prelude::Color;

use crate::camera::Camera;
use crate::point::{Point2D, Point3D};

pub struct Triangle2D {
    pub points: [Point2D; 3],
    pub depth: f32,
    pub color: Color,
}

impl Triangle2D {
    pub fn draw(&self, draw: &mut Draw) -> Result<(), String> {
        let size = draw.size();

        let half_width = size.0 / 2.;
        let half_height = size.1 / 2.;

        let p1 = Point2D::new(
            self.points[0].x + half_width,
            self.points[0].y + half_height,
        );
        let p2 = Point2D::new(
            self.points[1].x + half_width,
            self.points[1].y + half_height,
        );
        let p3 = Point2D::new(
            self.points[2].x + half_width,
            self.points[2].y + half_height,
        );

        draw.triangle(p1.as_tuple(), p2.as_tuple(), p3.as_tuple())
            .color(self.color);

        Ok(())
    }
}

pub struct Triangle3D {
    pub points: [Point3D; 3],
    pub color: Color,
}

impl Triangle3D {
    pub fn new(
        (x1, y1, z1): (f32, f32, f32),
        (x2, y2, z2): (f32, f32, f32),
        (x3, y3, z3): (f32, f32, f32),
        color: Color,
    ) -> Self {
        let p1 = Point3D::new(x1, y1, z1);
        let p2 = Point3D::new(x2, y2, z2);
        let p3 = Point3D::new(x3, y3, z3);

        Self {
            points: [p1, p2, p3],
            color,
        }
    }

    pub fn rasterize(&self, camera: &Camera) -> Option<Triangle2D> {
        let moved_points: Vec<Point3D> = self
            .points
            .iter()
            .map(|p| p.move_point(camera).rotate_x(camera).rotate_y(camera))
            .collect();

        if moved_points.iter().any(|p| p.z < 0.) {
            return None;
        }

        let projected_points: Vec<Point2D> = moved_points
            .iter()
            .map(|p| p.rasterize(camera.near))
            .collect();

        Some(Triangle2D {
            points: projected_points.try_into().unwrap(),
            depth: -moved_points[0].z,
            color: self.color,
        })
    }
}
