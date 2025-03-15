mod model;
mod osc;
mod scene;

use model::Model;
use nannou::color::BLACK;
use nannou::event::Update;
use nannou::{App, Frame};
use osc::Osc;
use scene::Scenes;
use scene::hat::Hat;
use scene::kick::Kick;
use scene::snare::Snare;

fn main() {
    nannou::app(model).update(update).view(draw).run();
}

fn init_scenes() -> Scenes {
    let mut scenes = Scenes::new();
    scenes.add_scene("bd", Kick::new());
    scenes.add_scene("hc", Hat::new());
    scenes.add_scene("sn", Snare::new());

    scenes.stop_all();
    scenes
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 600)
        .title("nannou OSC Visual")
        .build()
        .unwrap();

    let osc = Osc::listen("0.0.0.0:2020");

    let scenes = init_scenes();

    Model {
        osc,
        scenes,
        freqscope: [0; 1024],
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.scenes.update_all(&update);
    model.handle_osc();
}

fn draw(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.scenes.draw_all(app, model, &draw);

    draw.to_frame(app, &frame).unwrap();
}
