use macroquad::prelude::*;

// Newtonian constant of gravitation
const G: f64 = 6.67430e-11_f64;

// Astronomical unit in meters
pub const AU: f64 = 149.6e6_f64 * 1000.;

const SCALE: f64 = 300. / AU;

// pub struct Orbit {
//     history: Vec<DVec3>
// }

// impl Orbit {
//     pub fn new() -> Orbit {
//         return Orbit {history: Vec::new()};
//     }

//     pub fn draw(&self) {
//         for pos in self.history {
//             draw_circle(pos.x, pos.y, 2., self.body.color)
//         }
//     }
// }

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
    orbit: Vec<Vec3>,
    first_angle: f64,       // in radian
}

impl Clone for Body {
    fn clone(&self) -> Self {
        Body {
            id: self.id,
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
                orbit: Vec::new(),
                first_angle: f64::NAN,
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

        // POC
        let current_angle = f64::atan2(self.pos.y, self.pos.x);
        unsafe {
            if self.id == 1 {
                static mut TOTAL_ANGLE: f64 = 0.;
                if !TOTAL_ANGLE.is_nan() {
                    TOTAL_ANGLE += current_angle;
                }
            
                println!("1st = {} ; current = {} ; total = {}", self.first_angle, current_angle, TOTAL_ANGLE);
                // if self.first_angle.is_nan() || (!TOTAL_ANGLE.is_nan() && !almost::equal_with(current_angle, self.first_angle, 0.)) {
                if !TOTAL_ANGLE.is_nan() && !(TOTAL_ANGLE < 0. && TOTAL_ANGLE + current_angle > 0.) {
                    self.orbit.push(Vec3::new(x, y, 0.));
                }
                else {
                    println!("Not adding with id : {}", self.id);
                    TOTAL_ANGLE = f64::NAN;
                }

                if self.first_angle.is_nan() {
                    self.first_angle = current_angle;
                }

                // TODO: step_by(N) => N should be according to distance to the origin
                for dot in self.orbit.iter().step_by(30) {
                    // TODO: orbit needs improvement
                    draw_circle(
                        dot.x,
                        dot.y,
                        4.,
                        Color::new(self.color.r, self.color.g, self.color.b, 0.5),
                    );
                }
            }
        }
        // END OF POC

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
