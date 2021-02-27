// TODO: Consider to move MetaHandler to its own crate
use std::fs;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io;
use sha1::{Digest, Sha1};
use regex::Regex;
use drive_client::types::Metadata;
use serde::{Deserialize, Serialize};
use std::env;

// #[derive(Serialize, Deserialize, Debug)]
// struct FolderRecords {
//     records: HashMap<String, String>
// }

pub struct MetaHandler;

impl MetaHandler {
    pub fn new_dir(folder_path: &str) -> Result<(), std::io::Error> {
        let (name, path) = MetaHandler::get_folder_and_path(folder_path);
        MetaHandler::register_dir(name, path).unwrap();
        MetaHandler::create_metadata(folder_path).unwrap();

        Ok(())
    }

    fn register_dir(name: &str, path: &str) -> Result<(), std::io::Error> {
        let home = env::var("HOME").unwrap();
        println!("{}", &format!("{}/.hana/records/folders.json", home));
        let bytes = fs::read(&format!("{}/.hana/records/folders.json", home)).unwrap();
        let json = String::from_utf8_lossy(&bytes);
        let mut record: HashMap<String, String> = serde_json::from_str(&json).unwrap();

        record.insert(name.to_owned(), path.to_owned());
        Ok(())
    }

    pub fn get_all_dirs() {}

    pub fn get_folder_metadata() {}

    fn create_metadata(folder_path: &str) -> Result<(), std::io::Error> {
        fs::create_dir(folder_path)?;
        let mut file = fs::File::create(format!("{}.hana/metadata.json", folder_path))?;
        let json = serde_json::to_string(&MetaHandler::get_folder_metada(folder_path).unwrap())?;

        file.write_all(&json.as_bytes()).unwrap();

        Ok(())
    }

    pub fn get_folder_metada(folder_path: &str) -> std::io::Result<Vec<Metadata>> {
        let mut meta = Vec::new();
        for entry in fs::read_dir(format!("{}./", folder_path))? {
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
        let folder_name = s.remove(s.len() - 2);
        
        ( folder_name, path )
    }
}
