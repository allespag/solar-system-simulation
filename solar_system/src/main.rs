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
    let mut simulation = simulation::Simulation::new();
    
    let sun = simulation::Body::new(
        1.9885e30_f64,
        10_000., // wrong value, in km
        DVec3::ZERO,
        DVec3::ZERO,
        YELLOW,
    );
    simulation.add_body(sun);

    let mercury = simulation::Body::new(
        0.330e24_f64,
        4_879. / 2., // in km
        DVec3::new(57.9e6_f64 * 1000., 0., 0.),
        DVec3::new(0., -47.4 * 1000., 0.),
        DARKGRAY,
    );
    simulation.add_body(mercury);

    let venus = simulation::Body::new(
        4.87e24_f64,
        12_104. / 2., // in km,
        DVec3::new(108.2e6_f64 * 1000., 0., 0.),
        DVec3::new(0., -35.02 * 1000., 0.),
        WHITE,
    );
    simulation.add_body(venus);

    const DESIRED_FPS: f32 = 60.;

    loop {
        clear_background(BLACK);

        simulation.update();
        simulation.draw();

        // From https://github.com/not-fl3/macroquad/issues/380
        let minimum_frame_time = 1. / DESIRED_FPS;
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
        draw_text(format!("FPS: {}", get_fps()).as_str(), screen_width() - 60., 10., 16., WHITE);

        next_frame().await;
    }
}
