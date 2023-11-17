use nannou::prelude::*;
pub fn start() {
    nannou::app(model).update(update).run();
}
struct Model {
    len: f32,
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());
    app.new_window()
        .fullscreen()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();
    Model { len: 800.0 }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    model.len = map_range(app.mouse.x, win.left(), win.right(), 30., 900.)
}

fn branch(draw: &Draw, length: f32, lines_left: i32) {
    let rotation = PI * 2.0 / 3.0; // 120 deg
    draw.line()
        .weight(2.)
        .color(WHITE)
        .start(pt2(0.0, 0.0))
        .end(pt2(0.0, length));
    let len = length / 2.;
    if len > 5. {
        let draw2 = draw.x_y(0.0, len).rotate(rotation);
        branch(&draw2, len, 3);
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
