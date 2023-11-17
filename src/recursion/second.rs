use nannou::prelude::*;
pub fn start() {
    nannou::app(model).update(update).run()
}
struct Model {
    theta: f32,
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());
    app.new_window()
        .fullscreen()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();
    Model { theta: PI / 4. }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    model.theta = map_range(app.mouse.x, win.left(), win.right(), 0., PI)
}

fn branch(draw: &Draw, mut length: f32, theta: f32) {
    draw.line()
        .weight(2.)
        .color(WHITE)
        .start(pt2(0.0, 0.0))
        .end(pt2(0.0, length));
    let draw = draw.x_y(0.0, length);
    length *= 0.66;
    if length > 2. {
        let draw2 = draw.rotate(theta);
        branch(&draw2, length, theta);
        let draw3 = draw.rotate(-theta);
        branch(&draw3, length, theta);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let draw = draw.x_y(0.0, app.window_rect().bottom());
    branch(&draw, 250., model.theta);

    draw.to_frame(app, &frame).unwrap();
}
