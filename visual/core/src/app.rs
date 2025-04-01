use std::{
    path::{Path, PathBuf},
    sync::mpsc,
};

use crate::{
    Model, ParamsData, SceneBuilder, draw, event, osc::Osc, params::start_watch_file,
    scene::SceneManager, update,
};
use nannou::App as NannouApp;
use rodio::OutputStream;

pub struct App;

impl App {
    pub fn run(self, model: fn(app: &NannouApp) -> Model) {
        color_eyre::install().unwrap();

        nannou::app(model)
            .event(event)
            .update(update)
            .view(draw)
            .run();
    }
}

#[derive(Default)]
pub struct AppBuilder {
    base_path: Option<PathBuf>,
    scenes: Vec<SceneBuilder>,
}

impl AppBuilder {
    pub fn new() -> Self {
        AppBuilder::default()
    }

    pub fn base_path(mut self, base_path: impl AsRef<Path>) -> Self {
        self.base_path = Some(base_path.as_ref().into());
        self
    }

    pub fn scenes(mut self, scenes: Vec<SceneBuilder>) -> Self {
        self.scenes = scenes;
        self
    }

    pub fn build(self, app: &NannouApp) -> Model {
        app.new_window()
            .size(800, 600)
            .title("nannou OSC Visual")
            .build()
            .unwrap();

        let osc = Osc::listen("0.0.0.0:2020");

        let scenes = self
            .scenes
            .into_iter()
            .map(|mut scene| {
                if let (Some(params_file_path), Some(base_path)) =
                    (&scene.params_file_path, &self.base_path)
                {
                    let full_params_path = base_path.join("params").join(params_file_path);

                    let (tx, rx) = mpsc::channel();
                    start_watch_file(&full_params_path, tx);

                    let file_content = std::fs::read_to_string(&full_params_path).unwrap();
                    scene
                        .instance
                        .on_params_update(ParamsData::new(file_content));
                    scene.params_update_event_rx = Some(rx);
                }
                scene.build()
            })
            .collect();

        let scene_manager = SceneManager::new(scenes);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        Model {
            base_path: self.base_path.expect("please set the base_path"),
            osc,
            scene_manager,
            freqscope: [0; 1024],
            _audio_stream: _stream,
            audio_handle: stream_handle,
        }
    }
}
