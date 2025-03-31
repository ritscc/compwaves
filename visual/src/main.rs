mod model;
mod osc;
mod params;
mod scene;

use color_eyre::Result;
use model::Model;
use nannou::color::BLACK;
use nannou::event::WindowEvent::{KeyPressed, KeyReleased};
use nannou::event::{Key, Update};
use nannou::{App, Event, Frame};
use scene::SceneBuilder;
use scene::hat::Hat;
use scene::kick::Kick;
use scene::snare::Snare;

fn init_scene_builders() -> Vec<SceneBuilder> {
    vec![
        SceneBuilder::new::<Kick>().sound("bd").key(Key::B),
        SceneBuilder::new::<Snare>()
            .sound("sn")
            .key(Key::S)
            .param_file("snare.toml"),
        SceneBuilder::new::<Hat>().sound("hc").key(Key::H),
    ]
}

fn main() -> Result<()> {
    color_eyre::install()?;

    nannou::app(Model::new)
        .event(event)
        .update(update)
        .view(draw)
        .run();

    Ok(())
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.scenes.update_all(&update);
    model
        .osc
        .handle_event(&mut model.freqscope, &mut model.scenes);
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
                if let Some(scene) = model.scenes.get_mut_by_key(key) {
                    scene.instance.key_pressed(&model.audio_handle);
                }
            }
            KeyReleased(key) => {
                if let Some(scene) = model.scenes.get_mut_by_key(key) {
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
