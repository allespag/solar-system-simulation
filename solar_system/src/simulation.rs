// Newton's law of universal gravitation
// F = G * ((m1 * m2) / r^2)

// Newtonian constant of gravitation
const G: f64 = 6.67430e-11f64;

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
        current_velocity: Vec3,
        color: Color,
    ) -> Body {
        Body {
            mass: mass,
            radius: radius,
            pos: pos,
            initial_velocity: initial_velocity,
            current_velocity: current_velocity,
            color: color,
        }
    }

    pub fn from_config() -> Body {
        unimplemented!();
    }

    pub fn draw(&self) {
        draw_circle(
            self.pos.x + screen_width() / 2.0,
            self.pos.y + screen_height() / 2.0,
            self.radius as f32,
            self.color,
        );
    }
}

pub struct Simulation {
    bodies: Vec<Body>,
    time: i32,
    timestep: i32,
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
            timestep: 60 * 60,
        }
    }

    pub fn from_config() -> Simulation {
        unimplemented!();
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {
        self.bodies[0].pos.x += 1.;
    }

    pub fn draw(&self) {
        for body in self.bodies.iter() {
            body.draw();
        }
    }
}
