use hana_server::drive_server::DriveServer;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;
use crate::meta_handler::MetaHandler;

pub struct Server;

impl Server {
    pub fn listen(path: String) -> std::io::Result<()> {
        let metadata = MetaHandler::get_metadata(&path).unwrap();

        let t = thread::spawn(move || {
            let lock = Arc::new(RwLock::new(metadata));

            let c_lock = Arc::clone(&lock);

            let (tx, rx) = channel();

            thread::spawn(move || {
                for received in rx {
                    let mut _meta = lock.write().unwrap();
                    println!("File received: {:?}", received);
                    MetaHandler::push_metadata(&path, received).unwrap();
                }
            });

            DriveServer::listen(&c_lock, tx).unwrap();
        });

        t.join().unwrap();

        Ok(())
    }
}
