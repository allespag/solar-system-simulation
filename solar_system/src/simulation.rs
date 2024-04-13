use macroquad::prelude::*;

// Newtonian constant of gravitation
const G: f64 = 6.67430e-11_f64;

// Astronomical unit in meters
pub const AU: f64 = 149.6e6_f64 * 1000.;

const SCALE: f64 = 300. / AU;

pub struct Orbit {
    total_theta: f64,
    first_turn_made: bool,
    history: Vec<Vec3>,
    color: Color,
}

impl Orbit {
    pub fn new(color: &Color) -> Orbit {
        let c = Color::new(color.r, color.g, color.b, 0.5);
        
        return Orbit {
            total_theta: 0.,
            first_turn_made: false,
            history: Vec::new(),
            color: c,
        };
    }

    pub fn update(&mut self, x: f64, y: f64, x_scaled: f32, y_scaled: f32) {
        if self.first_turn_made {
            return;
        }
        
        let current_theta = f64::atan2(y, x);

        if !(self.total_theta < 0. && self.total_theta + current_theta > 0.) {
            self.history.push(Vec3::new(x_scaled, y_scaled, 0.));
            self.total_theta += current_theta;

        }
        else {
            self.first_turn_made = true;
        }

    }

    pub fn draw(&self) {
        // TODO: step_by(N) => N should be according to distance to the origin
        for pos in self.history.iter().step_by(50) {
            draw_circle(
                pos.x,
                pos.y,
                4.,
                self.color,
            );
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    STAR,
    PLANET,
}

pub struct Body {
    id: u64,
    type_: BodyType,
    mass: f64,               // in kg
    radius: f64,             // in km
    pos: DVec3,              // in meters
    current_velocity: DVec3, // in m/s
    color: Color,
    orbit: Orbit,
}

impl Clone for Body {
    fn clone(&self) -> Self {
        Body {
            id: self.id,
            orbit: Orbit::new(&self.color),
            ..Self::new(
                self.type_,
                self.mass,
                self.radius,
                self.pos,
                self.current_velocity,
                self.color,
            )
        }
    }
}

impl Body {
    pub fn new(
        type_: BodyType,
        mass: f64,
        radius: f64,
        pos: DVec3,
        initial_velocity: DVec3,
        color: Color,
    ) -> Body {
        // Please note that I'm uncertain about the implementation provided here.
        static mut ID_COUNTER: u64 = 0;

        unsafe {
            let id = ID_COUNTER;
            ID_COUNTER += 1;

            Body {
                id: id,
                type_: type_,
                mass: mass,
                radius: radius,
                pos: pos,
                current_velocity: initial_velocity,
                color: color,
                orbit: Orbit::new(&color),
            }
        }
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

    pub fn update(&mut self, bodies: &Vec<Body>, timestep: f64) {
        let mut total_force = DVec3::ZERO;

        for body in bodies.iter() {
            if self.id != body.id {
                total_force += self.attraction(body);
            }
        }
        self.current_velocity += total_force / self.mass * timestep;
        self.pos += self.current_velocity * timestep;
    }

    pub fn draw(&mut self) {
        let offset = DVec3::new(
            (screen_width() / 2.) as f64,
            (screen_height() / 2.) as f64,
            0.,
        );
        let x = (self.pos.x * SCALE + offset.x) as f32;
        let y = (self.pos.y * SCALE + offset.y) as f32;
        let r = self.radius.log(1.6);

        self.orbit.update(self.pos.x, self.pos.y, x, y);
        self.orbit.draw();
        
        draw_circle(x, y, r as f32, self.color);
    }
}

pub struct Simulation {
    bodies: Vec<Body>,
    time: i32,     // in seconds
    timestep: i32, // in seconds
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            bodies: Vec::new(),
            time: 0,
            timestep: 60 * 60,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {
        let old_bodies = self.bodies.to_vec();

        for body in &mut self.bodies {
            if body.type_ == BodyType::STAR {
                continue;
            }
            body.update(&old_bodies, self.timestep as f64);
        }
        self.time += self.timestep;
    }

    pub fn draw(&mut self) {
        for body in self.bodies.iter_mut() {
            body.draw();
        }
    }
}
