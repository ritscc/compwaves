pub use rodio::OutputStreamHandle;

use color_eyre::Result;
use rodio::{Decoder, Source as _};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

#[derive(PartialEq)]
pub struct DirtSound {
    name: String,
}

impl DirtSound {
    pub(crate) fn new(name: &str) -> Self {
        DirtSound {
            name: name.to_owned(),
        }
    }
}

pub struct AudioFile {
    path: PathBuf,
    pub volume: f32,
}

impl AudioFile {
    pub(crate) fn new(file_path: impl AsRef<Path>) -> Self {
        AudioFile {
            path: file_path.as_ref().into(),
            volume: 1.0,
        }
    }
}

impl PartialEq for AudioFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl AudioFile {
    pub fn play(&self, audio_handle: &rodio::OutputStreamHandle) -> Result<()> {
        let file = BufReader::new(File::open(&self.path)?);
        let source = Decoder::new(file)?;
        let source = source.amplify(self.volume);

        audio_handle.play_raw(source.convert_samples())?;

        Ok(())
    }

    pub(crate) fn rebased_path(self, base_path: impl AsRef<Path>) -> Self {
        AudioFile {
            path: base_path.as_ref().join(&self.path),
            volume: self.volume,
        }
    }
}

pub trait SoundPlayable {
    fn play(&self) -> Result<()>;
}

impl SoundPlayable for Option<(&AudioFile, &OutputStreamHandle)> {
    fn play(&self) -> Result<()> {
        if let Some((audio_file, audio_handle)) = self {
            let file = BufReader::new(File::open(&audio_file.path)?);
            let source = Decoder::new(file)?;
            let source = source.amplify(audio_file.volume);

            audio_handle.play_raw(source.convert_samples())?;
        }

        Ok(())
    }
}
