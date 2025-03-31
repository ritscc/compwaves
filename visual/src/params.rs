use notify::{RecursiveMode, Watcher};
use serde::de::DeserializeOwned;
use std::{ffi::OsStr, path::Path, sync::mpsc, thread};

pub struct ParamsData(String);

impl ParamsData {
    pub fn new(s: String) -> Self {
        ParamsData(s)
    }

    pub fn get<P>(self) -> color_eyre::Result<P>
    where
        P: DeserializeOwned,
    {
        Ok(toml::from_str::<P>(&self.0)?)
    }
}

pub fn start_watch_file(
    file_path: impl AsRef<OsStr>,
    update_event_tx: mpsc::Sender<notify::Event>,
) {
    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
    let mut watcher = notify::recommended_watcher(tx).expect("Paramater file not found");

    watcher
        .watch(Path::new(&file_path), RecursiveMode::Recursive)
        .unwrap();

    println!(
        "Starat watching file: {}",
        file_path.as_ref().to_string_lossy()
    );

    thread::spawn(move || {
        let _ = watcher;

        loop {
            match rx.recv() {
                Ok(event) => match event {
                    Ok(event) => update_event_tx.send(event).unwrap(),
                    Err(e) => println!("event error: {:?}", e),
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}
