use macroquad::prelude::*;


pub struct Background {
    stars: Vec<Vec2>
}

impl Background {
    pub fn new(stars_count: i8) -> Background {
        let mut stars = Vec::new();
        for _ in 0..stars_count {
            stars.push(Vec2::new(rand::gen_range(0., screen_width()), rand::gen_range(0., screen_height())));
        }

        return Background {stars: stars}
    }

    pub fn draw(&self) {
        for star in &self.stars {
            draw_circle(star.x, star.y, 1., WHITE);
        }
    }

}