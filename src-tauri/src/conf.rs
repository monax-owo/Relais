use std::{
  fs::OpenOptions,
  io::{BufReader, Read},
  path::Path,
  sync::{atomic::AtomicBool, Mutex, MutexGuard},
};

use default::*;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::util::SourceAppState;

pub mod default {
  pub fn key() -> String {
    String::from("teststest")
  }
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct AppConfig {
  #[serde(default = "key")]
  key: String,
}

// todo:必要になったらBuilder patternにする(AppConfigBuilder)
impl AppConfig {
  pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
    let path = path.as_ref();
    let file = OpenOptions::new()
      .create(true)
      .read(true)
      .write(true)
      .open(path)?;
    let mut buf_reader = BufReader::new(&file);
    let mut buf = String::new();
    buf_reader.read_to_string(&mut buf)?;
    let config = toml::from_str::<Self>(&buf).unwrap();

    Ok(config)
  }
}

impl SourceAppState {
  pub fn new<P: AsRef<Path>>(config_path: P) -> anyhow::Result<Self> {
    Ok(Self {
      config: Mutex::new(AppConfig::new(config_path)?),
      windows: Mutex::new(Vec::new()),
      // TODO:ウィンドウごとにする
      overlay: AtomicBool::new(false),
    })
  }

  pub fn conf(&self) -> MutexGuard<'_, AppConfig> {
    self.config.lock().unwrap()
  }

  pub fn conf_mut(&mut self) -> MutexGuard<'_, AppConfig> {
    self.config.lock().unwrap()
  }
}
