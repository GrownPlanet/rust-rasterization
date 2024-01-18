// use std::cmp;

use notan::draw::{Draw, DrawShapes};
use notan::prelude::Color;

use crate::camera::Camera;
use crate::point::{Point2D, Point3D};

pub struct Triangle2D {
    pub points: [Point2D; 3],
}

impl Triangle2D {
    pub fn _new((x1, y1): (i32, i32), (x2, y2): (i32, i32), (x3, y3): (i32, i32)) -> Self {
        let p1 = Point2D::new(x1, y1);
        let p2 = Point2D::new(x2, y2);
        let p3 = Point2D::new(x3, y3);

        Self {
            points: [p1, p2, p3],
        }
    }

    pub fn draw(
        &self,
        draw: &mut Draw,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), String> {
        let half_width = screen_width as i32 / 2;
        let half_height = screen_height as i32 / 2;

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
            .color(Color::from_rgb(0., 0., 0.));

        Ok(())
    }

    // fn sign(points: [Point2D; 3]) -> i32 {
    //     let p1 = points[0];
    //     let p2 = points[1];
    //     let p3 = points[2];

    //     (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    // }

    // fn is_inside(&self, point: Point2D) -> bool {
    //     let d1 = Self::sign([point, self.points[0], self.points[1]]);
    //     let d2 = Self::sign([point, self.points[2], self.points[0]]);
    //     let d3 = Self::sign([point, self.points[1], self.points[2]]);

    //     let has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
    //     let has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

    //     !(has_neg && has_pos)
    // }
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

    pub fn rasterize(&self, camera: &Camera) -> Option<Triangle2D> {
        let moved_points = self
            .points
            .iter()
            .map(|p| p.move_point(camera).rotate_x(camera).rotate_y(camera));

        if moved_points.clone().any(|p| p.z < 0) {
            return None;
        }

        let projected_points: Vec<Point2D> =
            moved_points.map(|p| p.rasterize(camera.near)).collect();

        Some(Triangle2D {
            points: projected_points.try_into().unwrap(),
        })
    }
}
