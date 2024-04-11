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
        simulation::BodyType::STAR,
        1.9885e30_f64,
        10_000., // wrong value, in km
        DVec3::ZERO,
        DVec3::ZERO,
        YELLOW,
    );
    simulation.add_body(sun);

    let mercury = simulation::Body::new(
        simulation::BodyType::PLANET,
        0.330e24_f64,
        4_879. / 2., // in km
        DVec3::new(57.9e6_f64 * 1000., 0., 0.),
        DVec3::new(0., 47.4 * 1000., 0.),
        DARKGRAY,
    );
    simulation.add_body(mercury);

    let venus = simulation::Body::new(
        simulation::BodyType::PLANET,
        4.87e24_f64,
        12_104. / 2., // in km
        DVec3::new(108.2e6_f64 * 1000., 0., 0.),
        DVec3::new(0., 35.02 * 1000., 0.),
        WHITE,
    );
    simulation.add_body(venus);

    let earth = simulation::Body::new(
        simulation::BodyType::PLANET,
        5.87e24_f64,
        12_756. / 2., // in km
        DVec3::new(149.6e6_f64 * 1000., 0., 0.),
        DVec3::new(0., 29.783 * 1000., 0.),
        BLUE,
    );
    simulation.add_body(earth);

    let mars = simulation::Body::new(
        simulation::BodyType::PLANET,
        0.642e24_f64,
        6_792. / 2., // in km
        DVec3::new(228.0e6_f64 * 1000., 0., 0.),
        DVec3::new(0., 24.1 * 1000., 0.),
        RED,
    );
    simulation.add_body(mars);

    let jupiter = simulation::Body::new(
        simulation::BodyType::PLANET,
        1898e24_f64,
        142_984. / 2., // in km
        DVec3::new(778.5e6_f64 * 1000., 0., 0.),
        DVec3::new(0., 13.1 * 1000., 0.),
        BROWN,
    );
    simulation.add_body(jupiter);

    let saturn = simulation::Body::new(
        simulation::BodyType::PLANET,
        568e24_f64,
        120_536. / 2., // in km
        DVec3::new(1432.0e6_f64 * 1000., 0., 0.),
        DVec3::new(0., 9.7 * 1000., 0.),
        Color::from_hex(0xFAE5BF),
    );
    simulation.add_body(saturn);

    let uranus = simulation::Body::new(
        simulation::BodyType::PLANET,
        86.8e24_f64,
        51_118. / 2., // in km
        DVec3::new(2867.0e6_f64 * 1000., 0., 0.),
        DVec3::new(0., 6.8 * 1000., 0.),
        Color::from_hex(0xACE5EE),
    );
    simulation.add_body(uranus);

    let neptune = simulation::Body::new(
        simulation::BodyType::PLANET,
        102.0e24_f64,
        49_528. / 2., // in km
        DVec3::new(4515.0e6_f64 * 1000., 0., 0.),
        DVec3::new(0., 5.4 * 1000., 0.),
        Color::from_hex(0x4B70DD),
    );
    simulation.add_body(neptune);

    let pluto = simulation::Body::new(
        simulation::BodyType::PLANET,
        0.0130e24_f64,
        2_376. / 2., // in km
        DVec3::new(5906.4e6_f64 * 1000., 0., 0.),
        DVec3::new(0., 4.7 * 1000., 0.),
        BEIGE,
    );
    simulation.add_body(pluto);

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
