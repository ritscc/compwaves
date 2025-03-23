mod model;
mod osc;
mod scene;

use model::Model;
use nannou::color::BLACK;
use nannou::event::WindowEvent::{KeyPressed, KeyReleased};
use nannou::event::{Key, Update};
use nannou::{App, Event, Frame};
use scene::hat::Hat;
use scene::kick::Kick;
use scene::snare::Snare;
use scene::{SceneTrigger, Scenes};

fn main() {
    dotenvy::dotenv().unwrap();

    nannou::app(Model::new)
        .event(event)
        .update(update)
        .view(draw)
        .run();
}

fn init_scenes() -> Scenes {
    let mut scenes = Scenes::new();
    scenes.add_scene("bd", Kick::new());
    scenes.add_scene("hc", Hat::new());
    scenes.add_scene("sn", Snare::new());
    scenes.add_scene(Key::S, Snare::new());

    scenes.stop_all();
    scenes
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

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        id: _id,
        simple: Some(window_event),
    } = event
    {
        match window_event {
            KeyPressed(key) => {
                let scene = model.scenes.0.get_mut(&SceneTrigger::KeyInput(key));
                if let Some(scene) = scene {
                    #[cfg(debug_assertions)]
                    scene.key_pressed(&model.audio_handle);

                    #[cfg(not(debug_assertions))]
                    scene.key_pressed();
                }
            }
            KeyReleased(key) => {
                let scene = model.scenes.0.get_mut(&SceneTrigger::KeyInput(key));
                if let Some(scene) = scene {
                    #[cfg(debug_assertions)]
                    scene.key_released(&model.audio_handle);

                    #[cfg(not(debug_assertions))]
                    scene.key_released();
                }
            }
            _ => {}
        }
    }
}

#[cfg(debug_assertions)]
pub fn play_sound(
    audio_handle: &rodio::OutputStreamHandle,
    file: impl AsRef<std::path::Path>,
    volume: f32,
) {
    use rodio::{Decoder, Source as _};
    use std::{fs::File, io::BufReader};

    let file = BufReader::new(File::open(file).unwrap());
    let source = Decoder::new(file).unwrap();
    let source = source.amplify(volume);

    audio_handle.play_raw(source.convert_samples()).unwrap();
}
