use macroquad::prelude::*;

// Newtonian constant of gravitation
const G: f64 = 6.67430e-11_f64;

// Astronomical unit in meters
pub const AU: f64 = 149.6e6_f64 * 1000.;

// TODO: this is a random value, fix this
const SCALE: f64 = 150. / AU;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    STAR,
    PLANET,
}

pub struct Body {
    type_: BodyType,
    mass: f64,
    radius: f64,
    pos: DVec3,
    current_velocity: DVec3,
    color: Color,
    orbit: Vec<Vec3>,
}

impl Clone for Body {
    fn clone(&self) -> Self {
        Body {
            orbit: self.orbit.clone(),
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
        Body {
            type_: type_,
            mass: mass,
            radius: radius,
            pos: pos,
            current_velocity: initial_velocity,
            color: color,
            orbit: Vec::new(),
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
            // TODO: temporary, making sure we're not on the same body
            if self.mass == body.mass {
                continue;
            }
            total_force += self.attraction(body);
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
        let x: f32 = (self.pos.x * SCALE + offset.x) as f32;
        let y: f32 = (self.pos.y * SCALE + offset.y) as f32;

        // TODO: this is a random value, fix this
        let r = self.radius as f32 / 500.;

        // TODO: max len in function of the distance to the sun ?
        if self.orbit.len() > 100 {
            self.orbit.remove(0);
        }
        self.orbit.push(Vec3::new(x, y, 0.));
        for o in self.orbit.iter() {
            draw_circle(o.x, o.y, 1., LIGHTGRAY);
        }

        draw_circle(x, y, r, self.color);
    }
}

pub struct Simulation {
    bodies: Vec<Body>,
    time: i32,
    timestep: i32, // in seconds
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            bodies: Vec::new(),
            time: 0,
            timestep: 24 * 60 * 60,
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
