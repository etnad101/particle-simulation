extern crate sdl2;

mod particle;
mod vector2d;

use particle::Particle;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BACKGROUND_COLOR: Color = Color::BLACK;
const PARTICLE_COLOR: Color = Color::YELLOW;
const PARTICLE_SIZE: i32 = 5;
const PARTICLE_COUNT: i32 = 100;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("particle simulation", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut particles: Vec<Particle> = Vec::new();

    for _ in 0..PARTICLE_COUNT {
        let x = rand::thread_rng().gen_range(0..WIDTH) as i32;
        let y = rand::thread_rng().gen_range(0..HEIGHT) as i32;

        let particle = Particle::new(x, y, PARTICLE_SIZE);
        particles.push(particle)
    }

    'running: loop {
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        canvas.set_draw_color(PARTICLE_COLOR);
        for p in &mut particles {
            p.draw(&mut canvas)?;
            p.update();
        }

        canvas.present();
    }
    return Ok(());
}
