use crate::{osc::Osc, scene::Scenes};

pub struct Model {
    pub osc: Osc,
    pub scenes: Scenes,
    pub freqscope: [i32; 1024],
}
