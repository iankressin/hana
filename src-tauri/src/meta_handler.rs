// TODO: Consider to move MetaHandler to its own crate
// TODO: Make $HOME static
use std::fs;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io;
use sha1::{Digest, Sha1};
use regex::Regex;
use drive_client::types::Metadata;
use serde::{Deserialize, Serialize};
use std::env;
use anyhow::{ Result as AnyResult, Error };

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
struct MetaHandlerError<'a> {
  message: &'a str,
}

impl<'a> MetaHandlerError<'a> {
  fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for MetaHandlerError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}


pub struct MetaHandler;

impl MetaHandler {
    pub fn new_dir(folder_path: &str) -> Result<(), Error> {
        let (name, path) = MetaHandler::get_folder_and_path(folder_path);
        match MetaHandler::register_dir(name, path) {
            Ok(_) => match MetaHandler::create_metadata(path) {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::new(err))
            
            },
            Err(err) => Err(Error::new(err))
        }
    }

    fn register_dir(name: &str, path: &str) -> Result<(), std::io::Error> {
        let mut record = MetaHandler::get_dirs_record()?;
        record.insert(name.to_owned(), path.to_owned());

        MetaHandler::set_dirs_record(&record)?;
        Ok(())
    }


    pub fn get_dirs_record() -> Result<HashMap<String, String>, std::io::Error> {
        let home = env::var("HOME").unwrap();
        let bytes = fs::read(&format!("{}/.hana/records/folders.json", home)).unwrap();
        let json = String::from_utf8_lossy(&bytes);
        let records: HashMap<String, String> = serde_json::from_str(&json).unwrap();
    
        Ok(records)
        
    }

    fn set_dirs_record(record: &HashMap<String, String>) -> Result<(), std::io::Error> {
        let home = env::var("HOME").unwrap();
        let json = serde_json::to_string(record).unwrap();
        fs::write(format!("{}/.hana/records/folders.json", home), &json).unwrap();

        Ok(())
    }

    fn create_metadata(folder_path: &str) -> Result<(), std::io::Error> {
        println!("Folder where metadata is being created: {}/.hana", folder_path);
        fs::create_dir(format!("{}/.hana/", folder_path))?;
        let mut file = fs::File::create(format!("{}/.hana/metadata.json", folder_path))?;
        let json = serde_json::to_string(&MetaHandler::get_folder_metada(folder_path).unwrap())?;

        file.write_all(&json.as_bytes()).unwrap();

        Ok(())
    }

    pub fn get_folder_metada(folder_path: &str) -> std::io::Result<Vec<Metadata>> {
        let mut meta = Vec::new();
        for entry in fs::read_dir(folder_path)? {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    let (name, extension) = MetaHandler::get_file_name_and_extension(
                        &entry.file_name().to_str().unwrap().to_string(),
                    );

                    let name_extension = {
                        if extension == "" {
                            name.clone()
                        } else {
                            format!("{}.{}", &name, &extension)
                        }
                    };

                    if !metadata.is_dir() {
                        let mut buf = [0u8; 20];
                        println!("File entry ==>>> {:?}", entry);
                        MetaHandler::hash_files(&entry.path().to_str().unwrap(), &mut buf);
                        meta.push(Metadata {
                            name_extension,
                            name,
                            extension,
                            size: metadata.len() as u32,
                            hash: hex::encode(buf),
                        })
                    }
                }
            }
        }

        Ok(meta)
    }

    pub fn get_file_name_and_extension(file: &String) -> (String, String) {
        // Looks for a dot at the begining or no dot at all
        let re = Regex::new(r"^\.|^[^.]*$").unwrap();

        if re.is_match(file) {
            (file.to_owned(), String::from(""))
        } else {
            let words = file.split(".").collect::<Vec<&str>>();
            let name = words[..words.len() - 1]
                .into_iter()
                .map(|i| i.to_string())
                .collect();
            let extension = words.last().unwrap().to_string();

            (name, extension)
        }
    }

    fn hash_files(path: &str, buf: &mut [u8]) {
        let mut file = fs::File::open(&path).unwrap();
        let mut hasher = Sha1::new();
        let n = io::copy(&mut file, &mut hasher).unwrap();

        buf.copy_from_slice(&hasher.finalize())
    }

    fn get_folder_and_path(path: &str) -> (&str, &str) {
    
        let splited_path = path.split("/");
        let mut s: Vec<&str> = splited_path.collect();
        let folder_name = s.remove(s.len() - 1);
        
        ( folder_name, path )
    }
}
