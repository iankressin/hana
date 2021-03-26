use crate::meta_handler::MetaHandler;
use hana_server::HanaServer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Server;

impl Server {
  pub fn listen(path: String, running: &'static mut bool) -> Result<(), std::io::Error> {
    let path = Arc::new(path);
    let path_clone = Arc::clone(&path);
    let metadata = MetaHandler::get_metadata(&path).unwrap();
    let lock = Arc::new(RwLock::new(metadata));
    let c_lock = Arc::clone(&lock);

    let (tx, rx) = channel();

    let t = thread::spawn(move || {
        for received in rx {
          let mut _meta = lock.write().unwrap();
          println!("File received: {:?}", received);
          MetaHandler::push_metadata(&path, received).unwrap();
        }
      });

    while *running {
      let tx_pipe = Sender::clone(&tx);
      HanaServer::listen(&c_lock, tx_pipe, &path_clone, false).unwrap();
      *running = false;
    }

    println!("Closing server ...");

    Ok(())
  }
}
