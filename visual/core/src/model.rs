use crate::{osc::Osc, scene::SceneManager};
use rodio::{OutputStream, OutputStreamHandle};

pub struct Model {
    pub(crate) osc: Osc,
    pub(crate) scene_manager: SceneManager,
    pub(crate) freqscope: [i32; 1024],
    pub(crate) audio_handle: OutputStreamHandle,
    pub(crate) _audio_stream: OutputStream,
}

impl Model {
    pub fn freqscope(&self) -> [i32; 1024] {
        self.freqscope
    }
}
