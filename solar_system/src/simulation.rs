// Newton's law of universal gravitation
// F = G * ((m1 * m2) / r^2)

// Newtonian constant of gravitation
const G: f64 = 6.67430e-11f64;

// Astronomical unit in km
const AU: f32 = 149.6e6;

// TODO: this is a random value, fix this
const SCALE: f32 = 1. / 300000.;

use macroquad::prelude::*;

pub struct Body {
    mass: f64,
    radius: f64,
    pos: Vec3,
    initial_velocity: Vec3,
    current_velocity: Vec3,
    color: Color,
}

impl Body {
    pub fn new(
        mass: f64,
        radius: f64,
        pos: Vec3,
        initial_velocity: Vec3,
        color: Color,
    ) -> Body {
        Body {
            mass: mass,
            radius: radius,
            pos: pos,
            initial_velocity: initial_velocity,
            current_velocity: initial_velocity,
            color: color,
        }
    }

    pub fn from_config() -> Body {
        unimplemented!();
    }

    pub fn update(&self) {
        unimplemented!();
    }

    pub fn draw(&self) {
        let x = self.pos.x * SCALE + screen_width() / 2.0;
        let y = self.pos.y * SCALE + screen_height() / 2.0;

        // TODO: this is a random value, fix this
        let r = self.radius as f32 / 100.;

        draw_circle(
            x,
            y,
            r,
            self.color,
        );
    }
}

pub struct Simulation {
    bodies: Vec<Body>,
    time: i32,
    timestep: i32, // in seconds
}

// TODO: learn more about "Default"
impl Default for Simulation {
    fn default() -> Self {
        Self {
            bodies: Vec::new(),
            time: 0,
            timestep: 1,
        }
    }
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            bodies: Vec::new(),
            time: 0,
            timestep: 60 * 60 * 24,
        }
    }

    pub fn from_config() -> Simulation {
        unimplemented!();
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {

    }

    pub fn draw(&self) {
        for body in self.bodies.iter() {
            body.draw();
        }
    }
}
