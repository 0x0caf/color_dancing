use nannou::prelude::*;
use rand::{thread_rng, Rng};

struct IntColor {
    r: i32,
    g: i32,
    b: i32,
}

impl IntColor {
    fn new(r: i32, g: i32, b: i32) -> IntColor {
        IntColor {
            r, g, b
        }
    }

    fn rand() -> IntColor {
        let mut rng = thread_rng();
        IntColor::new(
            rng.gen_range(0, 256),
            rng.gen_range(0, 256),
            rng.gen_range(0, 256),
        )
    }

    fn as_srgb(&self) -> Srgb {
        srgb((self.r as f32) / 256.0, (self.g as f32) / 256.0, (self.b as f32) / 256.0)
    }

    fn new_mix_color(&self) -> IntColor {
        let mut rng = thread_rng();
        IntColor::new(
            (rng.gen_range(0, 256) + self.r) / 2,
            (rng.gen_range(0, 256) + self.g) / 2,
            (rng.gen_range(0, 256) + self.b) / 2
        )
    }
}

enum AppState {
    Initialize,
    Clear,
    Update,
    NewOrbit,
    Reset,
    Wait
}

enum Shape {
    Square,
    Circle,
    IsoTriangle,
}
struct Model {
    app_state: AppState,
    radius: f32,
    distance_step: f32,
    current_distance: f32,
    orbit_step: f32,
    max_orbit_step: f32,
    max_distance: f32,
    current_color: IntColor,
    current_mix_color: IntColor,
    shape: Shape,
}

impl Model {
    fn new(app_state: AppState, mix_color: IntColor) -> Model {
        let radius = Model::rand_radius();
        let distance_step = radius * 0.5;
        Model {
            app_state,
            radius,
            distance_step,
            current_distance:  radius * 2.0,
            orbit_step: 0.0,
            max_orbit_step: Self::rand_max_orbit_step(),
            max_distance: 600.0,
            current_color: mix_color.new_mix_color(),
            current_mix_color: mix_color,
            shape: Self::rand_shape(),
        }
    }

    fn rand_max_orbit_step() -> f32 {
        let mut rng = thread_rng();
        rng.gen_range(80, 250) as f32
    }

    fn rand_radius() -> f32 {
        let mut rng = thread_rng();
        rng.gen_range(10, 40) as f32
    }

    fn rand_shape() -> Shape {
        let mut rng = thread_rng();
        let num = rng.gen_range(0, 3) as i32;
        match num {
            0 => Shape::Circle,
            1 => Shape::Square,
            2 => Shape::IsoTriangle,
            _ => Shape::Circle,
        }
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run()
}

fn model(_app: &App) -> Model {
    Model::new(AppState::Initialize, IntColor::rand())
}

fn update(app: &App, model: &mut Model, _update: Update) {
    match model.app_state {
        AppState::Clear => {
            model.app_state = AppState::Update;
        }
        AppState::Update => {
            model.orbit_step = model.orbit_step + 1.0;
            if model.orbit_step > model.max_orbit_step {
                model.app_state = AppState::Reset;
            }
        },
        AppState::Reset => {
            model.current_distance = model.current_distance + model.distance_step;
            if model.current_distance > model.max_distance {
                model.app_state = AppState::NewOrbit;
            } else {
                model.orbit_step = 0.0;
                model.current_color = model.current_mix_color.new_mix_color();
                model.max_orbit_step = Model::rand_max_orbit_step();
                model.radius = Model::rand_radius();
                model.distance_step = model.radius * 0.5;
                model.app_state = AppState::Update;
            }
        },
        AppState::NewOrbit => {
            model.orbit_step = 0.0;
            model.current_distance = model.radius * 2.0;
            model.current_mix_color = IntColor::rand();
            model.current_color = model.current_mix_color.new_mix_color();
            model.shape = Model::rand_shape();
            model.app_state = AppState::Wait;
        }
        AppState::Wait => {
            if app.keys.down.contains(&Key::Space) {
                model.app_state = AppState::Clear;
            }
        },
        _ => {
            model.app_state = AppState::Clear;
        }
    }

    if app.keys.down.contains(&Key::C) {
        model.app_state = AppState::NewOrbit;
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    let transparent = rgba(0.0, 0.0, 0.0, 0.0);
    match model.app_state {
        AppState::Clear => {
            draw.background().color(srgb(0.25, 0.25, 0.25));
        },
        AppState::Update => {
            let deg = (model.orbit_step/ model.max_orbit_step) * 360.0;
            let radian = deg_to_rad(deg);
            let pos_x = radian.sin() * model.current_distance;
            let pos_y = radian.cos() * model.current_distance;

            match model.shape {
                Shape::Circle => {
                    draw.ellipse().x_y(pos_x, pos_y).radius(model.radius).color(transparent).stroke(model.current_color.as_srgb());
                },
                Shape::IsoTriangle => {
                    draw.tri().rotate(radian).x_y(pos_x, pos_y).height(model.radius).width(model.radius).color(transparent).stroke_color(model.current_color.as_srgb());
                },
                Shape::Square => {
                    draw.rect().x_y(pos_x, pos_y).color(transparent).width(model.radius).height(model.radius).stroke_color(model.current_color.as_srgb());
                },
            }
        },
        _ => {

        }
    }
    draw.to_frame(app, &frame).unwrap();
}