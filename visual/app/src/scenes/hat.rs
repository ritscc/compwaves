use core::Model;
use core::nannou::prelude::*;
use core::scene::Scene;

#[derive(Default)]
pub struct Hat {
    is_active: bool,
    progress: f64,
}

impl Scene for Hat {
    fn invoke(&mut self) {
        self.is_active = true;
        self.progress = 0.;
    }

    fn stop(&mut self) {
        self.is_active = false;
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

        let line_y = win_rect.h() * 0.3;
        let line_width = 2.0;
        let line_color = WHITE;

        draw.line()
            .start(pt2(win_rect.left(), win_rect.top() - line_y))
            .end(pt2(win_rect.right(), win_rect.top() - line_y))
            .weight(line_width)
            .color(line_color);

        draw.line()
            .start(pt2(win_rect.left(), win_rect.bottom() + line_y))
            .end(pt2(win_rect.right(), win_rect.bottom() + line_y))
            .weight(line_width)
            .color(line_color);
    }
}
