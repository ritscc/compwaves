pub use app::{App, AppBuilder};
pub use model::Model;
pub use nannou::{self, App as NannouApp};
pub use params::ParamsData;
pub use rodio::OutputStreamHandle;
pub use scene::SceneBuilder;
pub use scene::SceneInstance;

mod app;
mod model;
mod osc;
mod params;
mod scene;

use color_eyre::Result;
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
                    scene.instance.key_pressed(&model.audio_handle);
                }
            }
            KeyReleased(key) => {
                if let Some(scene) = model.scene_manager.get_mut_by_key(key) {
                    scene.instance.key_released(&model.audio_handle);
                }
            }
            _ => {}
        }
    }
}

pub fn play_sound(
    audio_handle: &rodio::OutputStreamHandle,
    path: impl AsRef<std::path::Path>,
    volume: f32,
) -> Result<()> {
    use rodio::{Decoder, Source as _};
    use std::{fs::File, io::BufReader, path::PathBuf};

    let mut cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cargo_manifest_dir.pop();

    let file_path = cargo_manifest_dir.join("samples").join(path);

    let file = BufReader::new(File::open(file_path)?);
    let source = Decoder::new(file)?;
    let source = source.amplify(volume);

    audio_handle.play_raw(source.convert_samples())?;

    Ok(())
}
