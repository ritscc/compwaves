use crate::{Model, params::ParamsData};
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
pub trait SceneInstance {
    fn key_pressed(&mut self, audio_handle: &OutputStreamHandle) {}
    fn key_released(&mut self, audio_handle: &OutputStreamHandle) {}
    fn invoke(&mut self);
    fn stop(&mut self);
    fn draw(&self, app: &App, model: &Model, draw: &Draw);
    fn update(&mut self, update: &Update);
    fn on_params_update(&mut self, data: ParamsData) {}
}

#[derive(Default)]
pub(crate) struct SceneManager(Vec<Scene>);

#[allow(unused)]
impl SceneManager {
    pub(crate) fn new(scenes: Vec<Scene>) -> Self {
        SceneManager(scenes)
    }

    pub(crate) fn add_scene(&mut self, scene: Scene) {
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

    pub(crate) fn get_by_key(&self, key: Key) -> Option<&Scene> {
        self.0.iter().find(|v| v.key.contains(&key))
    }

    pub(crate) fn get_mut_by_key(&mut self, key: Key) -> Option<&mut Scene> {
        self.0.iter_mut().find(|v| v.key.contains(&key))
    }

    pub(crate) fn get_by_sound(&self, sound: &str) -> Option<&Scene> {
        self.0.iter().find(|v| v.sound.contains(&sound))
    }

    pub(crate) fn get_mut_by_sound(&mut self, sound: &str) -> Option<&mut Scene> {
        self.0.iter_mut().find(|v| v.sound.contains(&sound))
    }
}

pub struct Scene {
    pub(crate) instance: Box<dyn SceneInstance>,
    pub(crate) key: Vec<Key>,
    pub(crate) sound: Vec<&'static str>,
    params_update_event_rx: Option<mpsc::Receiver<notify::Event>>,
}

impl Scene {
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
    pub(crate) instance: Box<dyn SceneInstance>,
    pub(crate) params_file_path: Option<PathBuf>,
    pub(crate) params_update_event_rx: Option<mpsc::Receiver<notify::Event>>,
    key: Vec<Key>,
    sound: Vec<&'static str>,
}

impl SceneBuilder {
    pub fn new<SI: SceneInstance + std::default::Default + 'static>() -> Self {
        SceneBuilder {
            instance: Box::new(SI::default()),
            key: Vec::new(),
            sound: Vec::new(),
            params_file_path: None,
            params_update_event_rx: None,
        }
    }

    pub fn key(mut self, key: Key) -> Self {
        self.key.push(key);
        self
    }

    pub fn sound(mut self, name: &'static str) -> Self {
        self.sound.push(name);
        self
    }

    pub fn param_file(mut self, path: impl AsRef<Path>) -> Self {
        self.params_file_path = Some(path.as_ref().into());
        self
    }

    pub fn build(self) -> Scene {
        Scene {
            instance: self.instance,
            key: self.key,
            sound: self.sound,
            params_update_event_rx: self.params_update_event_rx,
        }
    }
}
