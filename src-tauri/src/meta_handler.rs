// TODO: Consider to move MetaHandler to its own crate
// TODO: Make $HOME static
use anyhow::Error;
use hana_types::Metadata;
use regex::Regex;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

pub struct MetaHandler;

impl MetaHandler {
  pub fn new_dir(folder_path: &str) -> Result<(String, String), Error> {
    let (name, path) = MetaHandler::get_folder_and_path(folder_path);
    match MetaHandler::register_dir(name, path) {
      Ok(_) => match MetaHandler::create_metadata(path) {
        Ok(_) => Ok((name.to_owned(), path.to_owned())),
        Err(err) => Err(Error::new(err)),
      },
      Err(err) => Err(Error::new(err)),
    }
  }

  fn register_dir(name: &str, path: &str) -> Result<(), std::io::Error> {
    let mut record = MetaHandler::get_dirs_record().unwrap();
    record.insert(name.to_owned(), path.to_owned());

    MetaHandler::set_dirs_record(&record)?;
    Ok(())
  }

  pub fn get_dirs_record() -> Result<HashMap<String, String>, Error> {
    let home = env::var("HOME").unwrap();
    let bytes = fs::read(&format!("{}/.hana/records/folders.json", home)).unwrap();
    let json = String::from_utf8_lossy(&bytes);
    let records: HashMap<String, String> = serde_json::from_str(&json).unwrap();

    Ok(records)
  }

  pub fn update(path: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(&MetaHandler::get_folder_metada(path).unwrap())?;
    fs::write(&format!("{}/.hana/metadata.json", path), &json).unwrap();

    Ok(())
  }

  pub fn get_dirs_record_as_vec() -> Result<Vec<(String, String)>, Error> {
    let dirs = MetaHandler::get_dirs_record().unwrap();
    let dirs = dirs.into_iter().map(|(name, path)| (name, path)).collect();

    Ok(dirs)
  }

  pub fn get_metadata(path: &str) -> Result<Vec<Metadata>, Error> {
    let bytes = fs::read(&format!("{}/.hana/metadata.json", path)).unwrap();
    let json = String::from_utf8_lossy(&bytes);
    let metadata: Vec<Metadata> = serde_json::from_str(&json).unwrap();

    Ok(metadata)
  }

  pub fn push_metadata(path: &str, meta: Metadata) -> Result<(), std::io::Error> {
    let mut metadata = MetaHandler::get_metadata(path).unwrap();
    metadata.push(meta.clone());

    let json = serde_json::to_string(&metadata).unwrap();
    fs::write(&format!("{}/.hana/metadata.json", path), &json).unwrap();

    Ok(())
  }

  fn set_dirs_record(record: &HashMap<String, String>) -> Result<(), std::io::Error> {
    let home = env::var("HOME").unwrap();
    let json = serde_json::to_string(record).unwrap();
    fs::write(format!("{}/.hana/records/folders.json", home), &json).unwrap();

    Ok(())
  }

  fn create_metadata(folder_path: &str) -> Result<(), std::io::Error> {
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
    io::copy(&mut file, &mut hasher).unwrap();

    buf.copy_from_slice(&hasher.finalize())
  }

  fn get_folder_and_path(path: &str) -> (&str, &str) {
    let splited_path = path.split("/");
    let mut s: Vec<&str> = splited_path.collect();
    let folder_name = s.remove(s.len() - 1);

    (folder_name, path)
  }
}
