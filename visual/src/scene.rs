pub mod hat;
pub mod kick;
pub mod snare;

use crate::{
    Model,
    params::{ParamsData, start_watch_file},
};
use nannou::{
    App, Draw,
    event::{Key, Update},
};
use rodio::OutputStreamHandle;
use std::{path::Path, sync::mpsc};

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
pub struct SceneManager(Vec<Scene>);

#[allow(unused)]
impl SceneManager {
    pub fn new(scenes: Vec<Scene>) -> Self {
        SceneManager(scenes)
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.0.push(scene);
    }

    pub fn invoke_all(&mut self) {
        for scene_instance in &mut self.0 {
            scene_instance.instance.invoke();
        }
    }

    pub fn stop_all(&mut self) {
        for scene in &mut self.0 {
            scene.instance.stop();
        }
    }

    pub fn update_all(&mut self, update: &Update) {
        for scene in &mut self.0 {
            scene.instance.update(update);
            scene.handle_params_update_event();
        }
    }

    pub fn draw_all(&self, app: &App, model: &Model, draw: &Draw) {
        for scene in &model.scenes.0 {
            scene.instance.draw(app, model, draw);
        }
    }

    pub fn get_by_key(&self, key: Key) -> Option<&Scene> {
        self.0.iter().find(|v| v.key.contains(&key))
    }

    pub fn get_mut_by_key(&mut self, key: Key) -> Option<&mut Scene> {
        self.0.iter_mut().find(|v| v.key.contains(&key))
    }

    pub fn get_by_sound(&self, sound: &str) -> Option<&Scene> {
        self.0.iter().find(|v| v.sound.contains(&sound))
    }

    pub fn get_mut_by_sound(&mut self, sound: &str) -> Option<&mut Scene> {
        self.0.iter_mut().find(|v| v.sound.contains(&sound))
    }
}

pub struct Scene {
    pub instance: Box<dyn SceneInstance>,
    pub key: Vec<Key>,
    pub sound: Vec<&'static str>,
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
    instance: Box<dyn SceneInstance>,
    key: Vec<Key>,
    sound: Vec<&'static str>,
    params_update_event_rx: Option<mpsc::Receiver<notify::Event>>,
}

impl SceneBuilder {
    pub fn new<SI: SceneInstance + std::default::Default + 'static>() -> Self {
        SceneBuilder {
            instance: Box::new(SI::default()),
            key: Vec::new(),
            sound: Vec::new(),
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
        let (tx, rx) = mpsc::channel();

        let params_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("params")
            .join(path);

        start_watch_file(&params_file_path, tx);

        let s = std::fs::read_to_string(params_file_path).unwrap();
        self.instance.on_params_update(ParamsData::new(s));

        self.params_update_event_rx = Some(rx);
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
