mod scene;

use core::App;
use core::AppBuilder;
use core::SceneBuilder;
use core::nannou::event::Key;
use scene::hat::Hat;
use scene::kick::Kick;
use scene::snare::Snare;

fn scenes() -> Vec<SceneBuilder> {
    vec![
        SceneBuilder::new::<Kick>().sound("bd").key(Key::B),
        SceneBuilder::new::<Snare>()
            .sound("sn")
            .key(Key::S)
            .param_file("snare.toml"),
        SceneBuilder::new::<Hat>().sound("hc").key(Key::H),
    ]
}

fn main() {
    App.run(app_builder);
}

fn app_builder(nannou_app: &core::NannouApp) -> core::Model {
    AppBuilder::new()
        .base_path(env!("CARGO_MANIFEST_DIR"))
        .scenes(scenes())
        .build(nannou_app)
}
