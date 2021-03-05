use crate::meta_handler::MetaHandler;
use hana_server::HanaServer;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Server;

impl Server {
  pub fn listen(
    path: String,
    rx_stop: Receiver<()>,
    _: &Sender<()>,
  ) -> Result<(), std::io::Error> {
    let metadata = MetaHandler::get_metadata(&path).unwrap();
    let lock = Arc::new(RwLock::new(metadata));
    let path= Arc::new(path);
    let path_clone = Arc::clone(&path);
    let c_lock = Arc::clone(&lock);
    let (tx, rx) = channel();

    let t = thread::spawn(move || {
      for received in rx {
        let mut _meta = lock.write().unwrap();
        println!("File received: {:?}", received);
        MetaHandler::push_metadata(&path, received).unwrap();
      }
    });

    HanaServer::listen(&c_lock, tx, &path_clone).unwrap();

    t.join().unwrap();

    Ok(())
  }
}
