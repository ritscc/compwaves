use nannou::App;
use rodio::{OutputStream, OutputStreamHandle};

use crate::{init_scenes, osc::Osc, scene::Scenes};
pub struct Model {
    pub osc: Osc,
    pub scenes: Scenes,
    pub freqscope: [i32; 1024],
    pub audio_handle: OutputStreamHandle,
    _audio_stream: OutputStream,
}

impl Model {
    pub fn new(app: &App) -> Self {
        app.new_window()
            .size(800, 600)
            .title("nannou OSC Visual")
            .build()
            .unwrap();

        let osc = Osc::listen("0.0.0.0:2020");
        let scenes = init_scenes();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        Model {
            osc,
            scenes,
            freqscope: [0; 1024],
            _audio_stream: _stream,
            audio_handle: stream_handle,
        }
    }
}
