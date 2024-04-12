use macroquad::prelude::*;

struct Star {
    pos: Vec2,
    alpha: f32,
}

impl Star {
    pub fn new(pos: Vec2, alpha: f32) -> Star {
        return Star {pos: pos, alpha: alpha};
    }

    pub fn from_random() -> Star {
        let pos = Vec2::new(
            rand::gen_range(0., screen_width()),
            rand::gen_range(0., screen_height())
        );
        let alpha = rand::gen_range(0., 1.);

        return Star::new(pos, alpha);
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 1., Color::new(1., 1., 1., self.alpha));
    }
}
pub struct Background {
    stars: Vec<Star>,
}

impl Background {
    pub fn new(stars_count: i8) -> Background {
        let mut stars = Vec::new();
        for _ in 0..stars_count {
            stars.push(Star::from_random());
        }

        return Background { stars: stars };
    }

    pub fn draw(&self) {
        for star in &self.stars {
            star.draw();
        }
    }
}
