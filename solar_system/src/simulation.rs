use macroquad::prelude::*;

// Newtonian constant of gravitation
const G: f64 = 6.67430e-11_f64;

// Astronomical unit in meters
pub const AU: f64 = 149.6e6_f64 * 1000.;

// TODO: this is a random value, fix this
const SCALE: f64 = 250. / AU;

#[derive(Clone, Copy)]
pub struct Body {
    mass: f64,
    radius: f64,
    pos: DVec3,
    current_velocity: DVec3,
    color: Color,
}

impl Body {
    pub fn new(mass: f64, radius: f64, pos: DVec3, initial_velocity: DVec3, color: Color) -> Body {
        Body {
            mass: mass,
            radius: radius,
            pos: pos,
            current_velocity: initial_velocity,
            color: color,
        }
    }

    pub fn from_config() -> Body {
        unimplemented!();
    }

    pub fn attraction(&self, other: &Body) -> DVec3 {
        let distance_x = other.pos.x - self.pos.x;
        let distance_y = other.pos.y - self.pos.y;
        let distance = f64::sqrt(distance_x * distance_x + distance_y * distance_y);

        let force = G * (self.mass * other.mass) / (distance * distance);
        let theta = f64::atan2(distance_y, distance_x);
        let force_x = f64::cos(theta) * force;
        let force_y = f64::sin(theta) * force;

        return DVec3::new(force_x, force_y, 0.);
    }

    pub fn update(&mut self, sun: &Body, timestep: f64) {
        // DO NOT WORK
        if std::ptr::eq(self, sun) {
            return;
        }

        let total_force = self.attraction(sun);
        self.current_velocity += total_force / self.mass * timestep;
        self.pos += self.current_velocity * timestep;
    }

    pub fn draw(&self) {
        let offset = DVec3::new(
            (screen_width() / 2.) as f64,
            (screen_height() / 2.) as f64,
            0.,
        );
        let x: f32 = (self.pos.x * SCALE + offset.x) as f32;
        let y: f32 = (self.pos.y * SCALE + offset.y) as f32;

        // TODO: this is a random value, fix this
        let r = self.radius as f32 / 100.;

        draw_circle(x, y, r, self.color);
    }
}

pub struct Simulation {
    bodies: Vec<Body>,
    time: i32,
    timestep: i32, // in seconds
}

// TODO: learn more about "Default"
// impl Default for Simulation {
//     fn default() -> Self {
//         Self {
//             bodies: Vec::new(),
//             time: 0,
//             timestep: 1,
//         }
//     }
// }

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            bodies: Vec::new(),
            time: 0,
            timestep: 24 * 60 * 60,
        }
    }

    pub fn from_config() -> Simulation {
        unimplemented!();
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {
        let sun = self.bodies[0];
        for body in self.bodies.iter_mut() {
            // TODO: temporary, i.e, we're dealing with the sun
            if body.color == YELLOW {
                continue;
            }

            body.update(&sun, self.timestep as f64);
        }
        self.time += self.timestep;
    }

    pub fn draw(&self) {
        for body in self.bodies.iter() {
            // TODO: temporary, i.e, we're dealing with the sun
            // if body.color == YELLOW {
            //     continue;
            // }
            body.draw();
        }
    }
}
