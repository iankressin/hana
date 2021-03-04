#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod meta_handler;
mod server;

use hana_client::drive_client::DriveClient;
use anyhow::Error;
use std::sync::mpsc::channel;
use std::sync::RwLock;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      let (tx, rx) = channel();
      // let mut serverRunning = RwLock::new(false);

      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            Init {
              folder,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || match meta_handler::MetaHandler::new_dir(&folder) {
                Ok(new_path) => Ok(new_path),
                Err(err) => Err(err.into()),
              },
              callback,
              error,
            ),

            GetFolders { callback, error } => tauri::execute_promise(
              _webview,
              move || match meta_handler::MetaHandler::get_dirs_record_as_vec() {
                Ok(dirs) => Ok(dirs),
                Err(err) => Err(err.into()),
              },
              callback,
              error,
            ),

            GetMetadata {
              path,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || match meta_handler::MetaHandler::get_metadata(&path) {
                Ok(dirs) => Ok(dirs),
                Err(err) => Err(err.into()),
              },
              callback,
              error,
            ),

            Sync {
              path,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || match meta_handler::MetaHandler::update(&path) {
                Ok(()) => match meta_handler::MetaHandler::get_metadata(&path) {
                  Ok(metadata) => Ok(metadata),
                  Err(err) => Err(err.into()),
                },
                Err(err) => Err(err.into()),
              },
              callback,
              error,
            ),

            RunServer {
              path,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                let _ = &tx.clone();

                match server::Server::listen(path, rx, &tx) {
                  Ok(()) => Ok(()),
                  Err(err) => Err(err.into()),
                }
              },
              callback,
              error,
            ),

            StopServer { callback, error } => tauri::execute_promise(
              _webview,
              move || match &tx.send(()) {
                Ok(()) => Ok(()),
                Err(err) => {
                    println!("Error: {}", err);
                    Err(Error::new(std::io::Error::new(
                      std::io::ErrorKind::Interrupted,
                      "Something went wrong!",
                    )))
                },
              },
              callback,
              error,
            ),

            StopServer { callback, error } => tauri::execute_promise(
              _webview,
              move || match &tx.send(()) {
                Ok(()) => Ok(()),
                Err(err) => {
                    println!("Error: {}", err);
                    Err(Error::new(std::io::Error::new(
                      std::io::ErrorKind::Interrupted,
                      "Something went wrong!",
                    )))
                },
              },
              callback,
              error,
            ),

            SendFiles { path, files, callback, error } => tauri::execute_promise(
              _webview,
              move || {
                DriveClient::send(files, &path);
                Ok(())
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
