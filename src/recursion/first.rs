use nannou::prelude::*;

pub fn start() {
    nannou::app(model).run();
}
struct Model;

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    app.new_window()
        .fullscreen()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();
    Model
}

fn draw_circle(draw: &Draw, x: f32, y: f32, mut r: f32) {
    let min_radius = 20.0;
    let random_hue = random_range(0., 255.);
    let color = Hsv::new(random_hue, 0.75, 1.0);
    let points = (0..=360).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * r + x;
        let y = radian.cos() * r + y;
        (pt2(x, y), color)
    });
    draw.polyline().weight(3.0).points_colored(points);
    if r > min_radius {
        r *= 0.6;
        draw_circle(draw, x + r, y, r);
        draw_circle(draw, x - r, y, r);
        draw_circle(draw, x, y + r, r);
        draw_circle(draw, x, y - r, r);
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw_circle(&draw, 0., 0., 360.);
    draw.to_frame(app, &frame).unwrap();
}
