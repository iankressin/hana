#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod meta_handler;
mod server;

use hana_client::drive_client::DriveClient;

fn main() {
  static mut flag_server: bool = false;

  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;

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
              || unsafe {
                flag_server = true;
                match server::Server::listen(path, &mut flag_server) {
                    Ok(()) => {
                        flag_server = false;
                        Ok(())
                    },
                  Err(err) => Err(err.into()),
                }
              },
              callback,
              error,
            ),

            StopServer { callback, error } => tauri::execute_promise(
              _webview,
              move || {
                  unsafe {
                    flag_server = false; 
                    println!("FLAG_SERVER: {}", flag_server);
                  }
                Ok(())
              },
              callback,
              error,
            ),

            SendFiles {
              path,
              files,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                DriveClient::send(files, &path);
                Ok(())
              },
              callback,
              error,
            ),

            HasDirs {
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                  match meta_handler::MetaHandler::has_dirs() {
                    Ok(dirs) => Ok(dirs),
                    Err(err) => Err(err.into()),
                  }
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
