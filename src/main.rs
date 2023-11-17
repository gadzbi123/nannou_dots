use std::ops::Add;

use nannou::{
    app,
    color::{encoding::Srgb, Alpha},
    draw::primitive::Line,
    prelude::*,
};
mod recursion;
fn main() {
    // nannou::app(model).update(update).simple_window(view).run();
    // nannou::app(model).run();
    // recursion::first::start();
    // recursion::second::start();
    // recursion::third::start();
    nannou::app(model).update(update).run();
}

#[derive(Clone, Copy)]
struct Circle {
    radius: f32,
    center: [f32; 2],
    color: Hsva,
}

impl Circle {
    fn new(win: Rect<f32>, r: f32) -> Self {
        Circle {
            radius: r,
            center: [
                random_range(win.left() / 2., win.right() / 2.),
                random_range(win.bottom() / 2., win.top() / 2.),
            ],
            color: Self::get_random_color(),
        }
    }
    fn get_random_color() -> Alpha<Hsv, f32> {
        hsva(random_range(0., 1.), 0.7, 0.7, 1.)
    }
    pub fn transparentize(&mut self) {
        self.color.alpha -= 0.02;
    }
    pub fn reset(&mut self, app: &App) {
        let win = app.window_rect();
        self.center = [
            random_range(win.left() / 2., win.right() / 2.),
            random_range(win.bottom() / 2., win.top() / 2.),
        ];
        self.radius = 10.;
        self.color = Self::get_random_color();
    }
}

struct Model {
    circles: Vec<Circle>,
    last_update: u128,
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());
    app.new_window()
        // .fullscreen()
        .size(1920, 900)
        .view(view)
        .key_pressed(generate_new_circle)
        .build()
        .unwrap();
    Model {
        circles: vec![
            Circle::new(app.window_rect(), 10.),
            Circle::new(app.window_rect(), 20.),
            Circle::new(app.window_rect(), 30.),
            Circle::new(app.window_rect(), 40.),
            Circle::new(app.window_rect(), 50.),
        ],
        last_update: 0,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    if update.since_start.as_millis() < 10 + model.last_update {
        return;
    }
    model.last_update = update.since_start.as_millis();
    for Circle {
        radius,
        center: _,
        color: _,
    } in model.circles.iter_mut()
    {
        *radius += 1.;
    }
    // it only transparentizes one circle per tick, need to filter all
    if let Some(circle) = model.circles.iter_mut().find(|x| x.radius > 250.) {
        circle.transparentize();
    }
    //same here
    if let Some(circle) = model.circles.iter_mut().find(|x| x.color.alpha < 0.01) {
        circle.reset(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    for circle in model.circles.iter() {
        let points = (0..=360).map(|i| {
            let x_axis = circle.center[0];
            let y_axis = circle.center[1];
            let rad = deg_to_rad(i as f32);
            let point = (
                rad.sin() * circle.radius + x_axis,
                rad.cos() * circle.radius + y_axis,
            );
            point
        });
        draw.polyline()
            .weight(5.)
            .points(points)
            .color(circle.color);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn generate_new_circle(app: &App, model: &mut Model, key: Key) {
    if key == Key::Space {
        model.circles.push(Circle::new(app.window_rect(), 10.))
    }
}
