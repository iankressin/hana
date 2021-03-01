#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod meta_handler;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            Init {
              folder,
              callback,
              error,
            } => tauri::execute_promise(_webview, move || {
                match meta_handler::MetaHandler::new_dir(&folder) {
                    Ok(_) => {
                        println!("Everything went down as expecter");
                        Ok(())
                    } 
                    Err(err) => {
                        println!("Err: {}", err);
                        Err(err.into())
                    } 

                }
            }, callback, error),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
