use nannou::geom::Quad;
use nannou::{app, color::encoding::Srgb, draw::primitive::Line, prelude::*};
mod recursion;
fn main() {
    // nannou::app(model).update(update).simple_window(view).run();
    // nannou::app(model).run();
    // recursion::first::start();
    // recursion::second::start();
    // recursion::third::start();
    nannou::app(model).run();
}
struct Model {
    len: f32,
    rotation: f32,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    app.new_window()
        .fullscreen()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();
    Model {
        rotation: PI * 2.0 / 3.0, // 120 deg
        len: 900.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    model.len = map_range(app.mouse.x, win.left(), win.right(), 30., 900.)
}

fn branch(draw: &Draw, length: f32, lines_left: i32) {
    let color = hsv(
        random_range(0., 1.),
        random_range(0.1, 1.),
        random_range(0.3, 1.),
    );
    // dbg!(color);
    let rotation = PI * 2.0 / 3.0;
    draw.line()
        .weight(4.)
        .color(color)
        .start(pt2(0.0, 0.0))
        .end(pt2(0.0, length));
    let len = length / 3.;
    if len > 5. {
        let draw2 = draw.x_y(0.0, len).rotate(rotation);
        branch(&draw2, len, 3);
        let draw3 = draw.x_y(0.0, 2.0 * len).rotate(rotation);
        branch(&draw3, len, 3);
    }
    if lines_left > 1 {
        branch(
            &draw.x_y(0.0, length).rotate(rotation),
            length,
            lines_left - 1,
        );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    let draw = draw
        .x_y(win.left() / 2., win.bottom() / 2.)
        .rotate(-PI / 2.);
    branch(&draw, model.len, 3);

    draw.to_frame(app, &frame).unwrap();
}
