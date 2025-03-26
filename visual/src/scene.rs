pub mod hat;
pub mod kick;
pub mod snare;
use rodio::OutputStreamHandle;
use std::collections::HashMap;

use crate::Model;
use nannou::{
    App, Draw,
    event::{Key, Update},
};

#[derive(Eq, Hash, PartialEq)]
pub enum SceneTrigger {
    KeyInput(Key),
    SoundName(String),
}

impl From<&str> for SceneTrigger {
    fn from(value: &str) -> Self {
        Self::SoundName(value.to_owned())
    }
}

impl From<Key> for SceneTrigger {
    fn from(value: Key) -> Self {
        Self::KeyInput(value)
    }
}

#[allow(unused)]
pub trait Scene {
    fn key_pressed(&mut self, audio_handle: &OutputStreamHandle) {}
    fn key_released(&mut self, audio_handle: &OutputStreamHandle) {}
    fn invoke(&mut self);
    fn stop(&mut self);
    fn draw(&self, app: &App, model: &Model, draw: &Draw);
    fn update(&mut self, update: &Update);
}

pub struct Scenes(pub HashMap<SceneTrigger, Box<dyn Scene>>);

#[allow(unused)]
impl Scenes {
    pub fn new() -> Self {
        Scenes(HashMap::new())
    }

    pub fn add_scene<T>(&mut self, trigger: impl Into<SceneTrigger>, scene: T)
    where
        T: Scene + 'static,
    {
        self.0.insert(trigger.into(), Box::new(scene));
    }

    pub fn start_all(&mut self) {
        for scene in &mut self.0.values_mut() {
            scene.invoke();
        }
    }

    pub fn stop_all(&mut self) {
        for scene in &mut self.0.values_mut() {
            scene.stop();
        }
    }

    pub fn update_all(&mut self, update: &Update) {
        for scene in &mut self.0.values_mut() {
            scene.update(update);
        }
    }

    pub fn draw_all(&self, app: &App, model: &Model, draw: &Draw) {
        for scene in model.scenes.0.values() {
            scene.draw(app, model, draw);
        }
    }
}
