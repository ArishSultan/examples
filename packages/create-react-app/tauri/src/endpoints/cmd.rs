use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  Init {},
  #[cfg(any(feature = "all-api", feature = "readTextFile"))]
  ReadTextFile {
    path: String,
    callback: String,
    error: String,
  },
  #[cfg(any(feature = "all-api", feature = "readBinaryFile"))]
  ReadBinaryFile {
    path: String,
    callback: String,
    error: String,
  },
  #[cfg(any(feature = "all-api", feature = "writeFile"))]
  WriteFile {
    file: String,
    contents: String,
    callback: String,
    error: String,
  },
  #[cfg(any(feature = "all-api", feature = "listFiles"))]
  ListFiles {
    path: String,
    callback: String,
    error: String,
  },
  #[cfg(any(feature = "all-api", feature = "listDirs"))]
  ListDirs {
    path: String,
    callback: String,
    error: String,
  },
  #[cfg(any(feature = "all-api", feature = "setTitle"))]
  SetTitle {
    title: String,
  },
  #[cfg(any(feature = "all-api", feature = "execute"))]
  Execute {
    command: String,
    args: Vec<String>,
    callback: String,
    error: String,
  },
  #[cfg(any(feature = "all-api", feature = "open"))]
  Open {
    uri: String,
  },
  ValidateSalt {
    salt: String,
    callback: String,
    error: String,
  },
  #[cfg(any(feature = "all-api", feature = "event"))]
  Listen {
    event: String,
    handler: String,
    once: bool,
  },
  #[cfg(any(feature = "all-api", feature = "event"))]
  Emit {
    event: String,
    payload: String,
  },
  #[cfg(not(any(feature = "dev-server", feature = "embedded-server")))]
  LoadAsset {
    asset: String,
    asset_type: String,
    callback: String,
    error: String,
  },
}
