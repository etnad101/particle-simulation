use crate::vector2d::Vector2d;

use rand::Rng;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32) -> Result<(), String> {
    let mut x = radius;
    let mut y = 0;

    let mut re = x * x + y * y - radius * radius;
    while x >= y {
        canvas.draw_point(Point::new(center.x() + x, center.y() + y))?;
        canvas.draw_point(Point::new(center.x() + y, center.y() + x))?;

        canvas.draw_point(Point::new(center.x() - x, center.y() + y))?;
        canvas.draw_point(Point::new(center.x() - y, center.y() + x))?;

        canvas.draw_point(Point::new(center.x() - x, center.y() - y))?;
        canvas.draw_point(Point::new(center.x() - y, center.y() - x))?;

        canvas.draw_point(Point::new(center.x() + x, center.y() - y))?;
        canvas.draw_point(Point::new(center.x() + y, center.y() - x))?;

        if 2 * (re + 2 * y + 1) + 1 - 2 * x > 0 {
            re += 1 - 2 * x;
            x -= 1;
        }
        re += 2 * y + 1;
        y += 1;
    }

    Ok(())
}

pub struct Particle {
    pub pos: Vector2d<i32>,
    pub velocity: Vector2d<f32>,
    pub size: i32,
}

impl Particle {
    pub fn new(x: i32, y: i32, size: i32) -> Self {
        Particle {
            pos: Vector2d::new(x, y),
            velocity: Vector2d::new(0.0, 0.0),
            size,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let center = Point::new(self.pos.x, self.pos.y);
        draw_circle(canvas, center, self.size)?;
        Ok(())
    }

    pub fn update(&mut self) {
        let x = rand::thread_rng().gen_range(-1..=1) as i32;
        let y = rand::thread_rng().gen_range(-1..=1) as i32;

        if self.pos.x + x <= 800 && self.pos.x + x >= 0 {
            self.pos.x += x;
        }
        if self.pos.y + y <= 600 && self.pos.y + y >= 0 {
            self.pos.y += y;
        }
    }
}
