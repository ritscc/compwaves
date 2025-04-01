use core::Model;
use core::SceneInstance;
use core::nannou::prelude::*;

#[derive(Default)]
pub struct Kick {
    is_active: bool,
    progress: f64,
}

impl SceneInstance for Kick {
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

    fn draw(&self, _app: &App, model: &Model, draw: &Draw) {
        if !self.is_active {
            return;
        }

        let points = (0..50).map(|i| {
            let x = i as f32;
            let point = pt2(x - 20., x.sin() * model.freqscope()[i] as f32 / 4.) * 20.0;
            (point, WHITE)
        });
        draw.polyline().weight(3.0).points_colored(points);
    }
}
