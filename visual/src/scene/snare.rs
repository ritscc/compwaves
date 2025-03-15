use super::Scene;
use crate::Model;
use nannou::prelude::*;

pub struct Snare {
    is_active: bool,
    progress: f64,
}

impl Snare {
    pub fn new() -> Self {
        Snare {
            is_active: false,
            progress: 0.0,
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
}
