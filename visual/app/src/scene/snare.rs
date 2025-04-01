use core::Model;
use core::OutputStreamHandle;
use core::SceneInstance;
use core::nannou::prelude::*;
use core::play_sound;
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

impl SceneInstance for Snare {
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

    fn key_pressed(&mut self, audio_handle: &OutputStreamHandle) {
        use std::path::Path;

        if self.key_counter == 0 {
            let audio_path = Path::new("superdirt-samples")
                .join("sn")
                .join("STATASA.wav");

            // if let Err(e) = play_sound(audio_handle, audio_path, 0.05) {
            //     eprintln!("{e}");
            // }
            self.invoke();
        }

        self.key_counter += 1;
    }

    fn key_released(&mut self, _audio_handle: &OutputStreamHandle) {
        self.key_counter = 0;
    }

    fn on_params_update(&mut self, data: core::ParamsData) {
        if let Ok(data) = data.get::<Params>() {
            self.params.size = data.size;
        }
    }
}
