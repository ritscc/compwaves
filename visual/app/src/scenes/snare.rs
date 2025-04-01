use core::Model;
use core::nannou::prelude::*;
use core::scene::Scene;
use core::sound::AudioFile;
use core::sound::OutputStreamHandle;
use core::sound::SoundPlayable as _;
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct Params {
    size: f32,
}

#[derive(Default)]
pub struct Snare {
    is_active: bool,
    progress: f64,
    key_counter: u32,
    params: Params,
}

impl Scene for Snare {
    fn invoke(&mut self) {
        self.is_active = true;
    }

    fn stop(&mut self) {
        self.is_active = false;
        self.progress = 0.;
    }

    fn update(&mut self, update: &Update) {
        if !self.is_active {
            return;
        }

        let delta_seconds = update.since_last.secs();
        self.progress += delta_seconds;

        if self.progress > 0.1 {
            self.stop();
        }
    }

    fn draw(&self, app: &App, _model: &Model, draw: &Draw) {
        if !self.is_active {
            return;
        }

        let win_rect = app.window_rect();
        draw.ellipse()
            .xy(win_rect.xy())
            .radius(self.params.size)
            .color(WHITE);
    }

    fn key_pressed(&mut self, audio: Option<(&AudioFile, &OutputStreamHandle)>) {
        if self.key_counter == 0 {
            audio.play().unwrap();
            self.invoke();
        }

        self.key_counter += 1;
    }

    fn key_released(&mut self, _audio: Option<(&AudioFile, &OutputStreamHandle)>) {
        self.key_counter = 0;
    }

    fn on_params_update(&mut self, data: core::ParamsData) {
        if let Ok(data) = data.get::<Params>() {
            self.params.size = data.size;
        }
    }
}
