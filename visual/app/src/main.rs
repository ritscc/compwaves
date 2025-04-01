mod scenes;

use core::App;
use core::AppConfig;
use core::nannou::event::Key;
use core::scene::SceneBuilder;
use scenes::hat::Hat;
use scenes::kick::Kick;
use scenes::snare::Snare;
use std::path::Path;
use std::path::PathBuf;

fn scenes() -> Vec<SceneBuilder> {
    vec![
        SceneBuilder::new::<Kick>().dirt_sound("bd").key(Key::B),
        SceneBuilder::new::<Snare>()
            .dirt_sound("sn")
            .key(Key::S)
            .audio_file(
                Path::new("superdirt-samples")
                    .join("sn")
                    .join("STATASA.wav"),
            )
            .audio_volume(0.05)
            .param_file("snare.toml"),
        SceneBuilder::new::<Hat>().dirt_sound("hc").key(Key::H),
    ]
}

fn config(nannou_app: &core::NannouApp) -> core::Model {
    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let params_base_path = cargo_manifest_dir.join("params");
    let audio_base_path = PathBuf::from(&cargo_manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("samples");

    AppConfig::new()
        .params_base_path(params_base_path)
        .audio_base_path(audio_base_path)
        .scenes(scenes())
        .build(nannou_app)
}

fn main() {
    App.run(config);
}
