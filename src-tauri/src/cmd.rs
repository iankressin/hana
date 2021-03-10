use hana_types::Metadata;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  Init {
    folder: String,
    callback: String,
    error: String,
  },

  GetFolders {
    callback: String,
    error: String,
  },

  GetMetadata {
    path: String,
    callback: String,
    error: String,
  },

  Sync {
    path: String,
    callback: String,
    error: String,
  },

  RunServer {
    path: String,
    callback: String,
    error: String,
  },

  StopServer {
    callback: String,
    error: String,
  },

  SendFiles {
    path: String,
    files: Vec<Metadata>,
    callback: String,
    error: String,
  },

  HasDirs {
      callback: String,
      error: String,
  },

  OpenFile {
      path: String,
  }
}
