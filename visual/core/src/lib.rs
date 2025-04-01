pub use app::{App, AppBuilder};
pub use model::Model;
pub use nannou::{self, App as NannouApp};
pub use params::ParamsData;

mod app;
mod model;
mod osc;
mod params;
pub mod scene;
pub mod sound;

use nannou::color::BLACK;
use nannou::event::Update;
use nannou::event::WindowEvent::{KeyPressed, KeyReleased};
use nannou::{Event, Frame};

fn update(_app: &NannouApp, model: &mut Model, update: Update) {
    model.scene_manager.update_all(&update);
    model
        .osc
        .handle_event(&mut model.freqscope, &mut model.scene_manager);
}

fn draw(app: &NannouApp, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.scene_manager.draw_all(app, model, &draw);

    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &NannouApp, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        id: _id,
        simple: Some(window_event),
    } = event
    {
        match window_event {
            KeyPressed(key) => {
                if let Some(scene) = model.scene_manager.get_mut_by_key(key) {
                    scene
                        .instance
                        .key_pressed(scene.audio_file.as_ref().map(|f| (f, &model.audio_handle)));
                }
            }
            KeyReleased(key) => {
                if let Some(scene) = model.scene_manager.get_mut_by_key(key) {
                    scene
                        .instance
                        .key_released(scene.audio_file.as_ref().map(|f| (f, &model.audio_handle)));
                }
            }
            _ => {}
        }
    }
}
