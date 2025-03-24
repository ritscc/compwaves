use super::Scene;
use crate::Model;
#[cfg(debug_assertions)]
use crate::play_sound;
use nannou::prelude::*;
use rodio::OutputStreamHandle;

pub struct Snare {
    is_active: bool,
    progress: f64,
    key_counter: u32,
}

impl Snare {
    pub fn new() -> Self {
        Snare {
            is_active: false,
            progress: 0.0,
            key_counter: 0,
        }
    }
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
        draw.ellipse().xy(win_rect.xy()).radius(100.).color(WHITE);
    }

    #[cfg(debug_assertions)]
    fn key_pressed(&mut self, audio_handle: &OutputStreamHandle) {
        use std::path::Path;

        if self.key_counter == 0 {
            let audio_path = Path::new("superdirt-samples")
                .join("sn")
                .join("STATASA.wav");

            if let Err(e) = play_sound(audio_handle, audio_path, 0.05) {
                eprintln!("{e}");
            }
            self.invoke();
        }

        self.key_counter += 1;
    }

    #[cfg(debug_assertions)]
    fn key_released(&mut self, _audio_handle: &OutputStreamHandle) {
        self.key_counter = 0;
    }
}
