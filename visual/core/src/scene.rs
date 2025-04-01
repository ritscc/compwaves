use crate::{
    Model,
    params::ParamsData,
    sound::{AudioFile, DirtSound},
};
use nannou::{
    App, Draw,
    event::{Key, Update},
};
use rodio::OutputStreamHandle;
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
};

#[allow(unused)]
pub trait Scene {
    fn key_pressed(&mut self, audio: Option<(&AudioFile, &OutputStreamHandle)>) {}
    fn key_released(&mut self, audio: Option<(&AudioFile, &OutputStreamHandle)>) {}
    fn invoke(&mut self);
    fn stop(&mut self);
    fn draw(&self, app: &App, model: &Model, draw: &Draw);
    fn update(&mut self, update: &Update);
    fn on_params_update(&mut self, data: ParamsData) {}
}

#[derive(Default)]
pub(crate) struct SceneManager(Vec<SceneInstance>);

#[allow(unused)]
impl SceneManager {
    pub(crate) fn new(scenes: Vec<SceneInstance>) -> Self {
        SceneManager(scenes)
    }

    pub(crate) fn add_scene(&mut self, scene: SceneInstance) {
        self.0.push(scene);
    }

    pub(crate) fn invoke_all(&mut self) {
        for scene_instance in &mut self.0 {
            scene_instance.instance.invoke();
        }
    }

    pub(crate) fn stop_all(&mut self) {
        for scene in &mut self.0 {
            scene.instance.stop();
        }
    }

    pub(crate) fn update_all(&mut self, update: &Update) {
        for scene in &mut self.0 {
            scene.instance.update(update);
            scene.handle_params_update_event();
        }
    }

    pub(crate) fn draw_all(&self, app: &App, model: &Model, draw: &Draw) {
        for scene in &model.scene_manager.0 {
            scene.instance.draw(app, model, draw);
        }
    }

    pub(crate) fn get_by_key(&self, key: Key) -> Option<&SceneInstance> {
        self.0.iter().find(|v| v.key.contains(&key))
    }

    pub(crate) fn get_mut_by_key(&mut self, key: Key) -> Option<&mut SceneInstance> {
        self.0.iter_mut().find(|v| v.key.contains(&key))
    }

    pub(crate) fn get_by_dirt_sound(&self, sound: &DirtSound) -> Option<&SceneInstance> {
        self.0.iter().find(|v| v.dirt_sounds.contains(sound))
    }

    pub(crate) fn get_mut_by_dirt_sound(
        &mut self,
        sound: &DirtSound,
    ) -> Option<&mut SceneInstance> {
        self.0.iter_mut().find(|v| v.dirt_sounds.contains(sound))
    }
}

pub struct SceneInstance {
    pub(crate) instance: Box<dyn Scene>,
    pub(crate) key: Vec<Key>,
    pub(crate) dirt_sounds: Vec<DirtSound>,
    pub(crate) audio_file: Option<AudioFile>,
    params_update_event_rx: Option<mpsc::Receiver<notify::Event>>,
}

impl SceneInstance {
    fn handle_params_update_event(&mut self) {
        if let Some(params_update_event_rx) = &self.params_update_event_rx {
            if let Ok(event) = params_update_event_rx.try_recv() {
                if event.kind.is_modify() {
                    if let Some(first_path) = event.paths.first() {
                        let s = std::fs::read_to_string(first_path).unwrap();
                        self.instance.on_params_update(ParamsData::new(s));
                    }
                }
            }
        }
    }
}

pub struct SceneBuilder {
    pub(crate) instance: Box<dyn Scene>,
    pub(crate) params_file_path: Option<PathBuf>,
    pub(crate) params_update_event_rx: Option<mpsc::Receiver<notify::Event>>,
    keys: Vec<Key>,
    dirt_sound_names: Vec<&'static str>,
    audio_file_path: Option<PathBuf>,
    audio_volume: Option<f32>,
}

impl SceneBuilder {
    pub fn new<SI: Scene + std::default::Default + 'static>() -> Self {
        SceneBuilder {
            instance: Box::new(SI::default()),
            keys: Vec::new(),
            params_file_path: None,
            params_update_event_rx: None,
            dirt_sound_names: Vec::new(),
            audio_file_path: None,
            audio_volume: None,
        }
    }

    pub fn key(mut self, key: Key) -> Self {
        self.keys.push(key);
        self
    }

    pub fn dirt_sound(mut self, name: &'static str) -> Self {
        self.dirt_sound_names.push(name);
        self
    }

    pub fn audio_file(mut self, file_path: impl AsRef<Path>) -> Self {
        self.audio_file_path = Some(file_path.as_ref().into());

        self
    }

    pub fn audio_volume(mut self, volume: f32) -> Self {
        self.audio_volume = Some(volume);
        self
    }

    pub fn param_file(mut self, path: impl AsRef<Path>) -> Self {
        self.params_file_path = Some(path.as_ref().into());
        self
    }

    pub fn build(self, audio_base_path: impl AsRef<Path>) -> SceneInstance {
        let dirt_sounds = self
            .dirt_sound_names
            .into_iter()
            .map(DirtSound::new)
            .collect();

        let audio_file = self.audio_file_path.map(|f| {
            let mut audio_file = AudioFile::new(f);

            if let Some(audio_volume) = self.audio_volume {
                audio_file.volume = audio_volume;
            }

            audio_file.rebased_path(audio_base_path)
        });

        SceneInstance {
            instance: self.instance,
            key: self.keys,
            dirt_sounds,
            audio_file,
            params_update_event_rx: self.params_update_event_rx,
        }
    }
}
