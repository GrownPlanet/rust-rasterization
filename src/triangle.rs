use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
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

        let p1 = Point::new(
            self.points[0].x + half_width,
            self.points[0].y + half_height,
        );
        let p2 = Point::new(
            self.points[1].x + half_width,
            self.points[1].y + half_height,
        );
        let p3 = Point::new(
            self.points[2].x + half_width,
            self.points[2].y + half_height,
        );

        let tri_offset = Triangle2D {
            points: [p1, p2, p3],
        };

        // canvas.draw_line(p1_2, p2_2)?;
        // canvas.draw_line(p1_2, p3_2)?;
        // canvas.draw_line(p2_2, p3_2)?;

        // let min_x = tri_offset.points.iter().min_by_key(|p| p.x).unwrap().x;
        // let min_y = tri_offset.points.iter().min_by_key(|p| p.y).unwrap().y;
        // let max_x = tri_offset.points.iter().max_by_key(|p| p.x).unwrap().x;
        // let max_y = tri_offset.points.iter().max_by_key(|p| p.y).unwrap().y;

        // let dst = Rect::new(min_x, min_y, (max_x - min_x) as u32, (max_y - min_y) as u32);
        // println!("{:?}", dst);

        let size = canvas.output_size().unwrap();

        // let size = canvas.output_size().unwrap();

        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, size.0, size.1)
            .map_err(|e| e.to_string())?;

        // Create a red-green gradient
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..size.1 {
                for x in 0..size.0 {
                    let offset = y as usize * pitch + x as usize * 3;
                    if tri_offset.is_inside(Point::new(x as i32, y as i32)) {
                        buffer[offset] = 255;
                    }
                    // buffer[offset + 1] = y as u8;
                    // buffer[offset + 2] = 0;
                }
            }
        })?;

        canvas.copy(&texture, None, None)?;

        Ok(())
    }

    fn sign(points: [Point; 3]) -> i32 {
        let p1 = points[0];
        let p2 = points[1];
        let p3 = points[2];

        (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    }

    pub fn is_inside(&self, point: Point) -> bool {
        let d1 = Self::sign([point, self.points[0], self.points[1]]);
        let d2 = Self::sign([point, self.points[2], self.points[0]]);
        let d3 = Self::sign([point, self.points[1], self.points[2]]);

        let has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
        let has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

        !(has_neg && has_pos)
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
        let moved_points = self
            .points
            .iter()
            .map(|p| p.move_point(camera).rotate_x(camera).rotate_y(camera));

        if moved_points.clone().any(|p| p.z < 0) {
            return None;
        }

        let projected_points: Vec<Point> = moved_points.map(|p| p.rasterize(near)).collect();

        Some(Triangle2D {
            points: projected_points.try_into().unwrap(),
        })
    }
}
