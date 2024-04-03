use macroquad::prelude::*;

mod simulation;

fn window_conf() -> Conf {
    Conf {
        window_title: "Solar System".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let sun = simulation::Body::new(
        1988500e24f64,
        149.57, // in million km
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        YELLOW,
    );

    let mut simulation = simulation::Simulation::new();
    simulation.add_body(sun);

    loop {
        clear_background(BLACK);

        simulation.update();
        simulation.draw();

        // From https://github.com/not-fl3/macroquad/issues/380
        let minimum_frame_time = 1. / 60.; // 60 FPS
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
        draw_text(format!("FPS: {}", get_fps()).as_str(), screen_width() - 60., 10., 16., WHITE);

        next_frame().await;
    }
}
