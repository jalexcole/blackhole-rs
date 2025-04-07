use std::{alloc::System, time::{Instant, SystemTime}};

use rand::{Rng, rngs::ThreadRng};

use raylib::prelude::*;
use rayon::{iter::IntoParallelRefIterator, slice::{ParallelSlice, ParallelSliceMut}};

fn main() {
    let mut random = rand::rng();

    let mut pariticles = Vec::new();

    (0..10000).for_each(|_| {
        let p = Particle::random(&mut random);
        pariticles.push(p);
    });

    let camera = Camera3D::perspective(
        Vector3 {
            x: 100.0,
            y: 100.0,
            z: 100.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        45.0,
    );

    let (mut rl, thread) = raylib::init().size(1366, 768).title("Hello, World").build();
    let mut start_time = SystemTime::now();
    let mut stop_time = SystemTime::now();
    while !rl.window_should_close() {
        let elapsed_time = stop_time.duration_since(start_time).unwrap();
        let mut d = rl.begin_drawing(&thread);
        start_time = SystemTime::now();
        
        update(&mut pariticles, elapsed_time.as_secs_f64());
        d.clear_background(Color::BLACK);
        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_fps(12, 12);
        let mut e = d.begin_mode3D(camera);


        
        for p in pariticles.iter() {
            p.draw(&mut e);
            // e.draw_sphere(p.position, 0.1, Color::RED);
        }

        stop_time = SystemTime::now();
    }
}


fn update(particles: &mut [Particle], time_step: f64) {

    let mut gravity_forces = Vec::new();

    for p in particles.iter() {
        let mut grav_force = Vector3::new(0.0, 0.0, 0.0);

        for q in particles.as_parallel_slice() {
            if p != q {
                grav_force += gravity_func(p, q);
            }
        }

        gravity_forces.push(grav_force);
    }

    for p in particles.as_parallel_slice_mut() {
        let mut vel = p.velocity.clone();
        vel.x = vel.x * time_step as f32;
        vel.y = vel.y * time_step as f32;
        vel.z = vel.z * time_step as f32;

        p.position += vel;
    }
    
    for i in 0..particles.len() {
        particles[i].velocity += gravity_forces[i] * time_step as f32;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Particle {
    position: Vector3,
    velocity: Vector3,
    mass: f64,
}

impl Particle {
    pub fn random(random: &mut ThreadRng) -> Particle {
        Particle {
            position: Vector3::new(
                random.random::<f32>() * 10.0,
                random.random::<f32>() * 10.0,
                random.random::<f32>() * 10.0,
            ),
            velocity: Vector3::new(random.random(), random.random(), random.random()),
            mass: random.random::<f64>() * 100000.0,
        }
    }
}

impl Drawable for Particle {
    fn draw(&self, d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>) {
        d.draw_sphere(self.position, 0.1, Color::ORANGE);
    }
}


pub trait Drawable {
    fn draw(&self, d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>);
}

const G: f64 = 6.67408e-11;

/// Calculates the gravitational force between two particles using the formula
/// F = G * (m1 * m2) / r^2
/// 
/// Where G is the gravitational constant, m1 and m2 are the masses of the two
/// particles and r is the distance between them.
/// 
/// The result is the force felt by particle 1 due to particle 2.

pub fn gravity_func(p1: &Particle, p2: &Particle) -> Vector3 {
    

    let m1 = p1.mass as f32;
    let m2 = p2.mass as f32;

    let r = p1.position - p2.position;
    let r2 = r.length().powf(2.0);
    

    let force = G as f32 * m1 * m2 / r2;

   - ( p1.position - p2.position ) / (p1.position - p2.position).length() * force
}

