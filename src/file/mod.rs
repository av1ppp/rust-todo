use dirs::home_dir;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};

pub fn get_path() -> Result<String, String> {
    match home_dir() {
        Some(mut home) => {
            home.push(".todos");
            match home.to_str() {
                Some(filepath) => {
                    Ok(String::from(filepath))
                }
                None => {
                    Err(String::from("home dir is not defined"))
                }
            }
        }
        None => {
            Err(String::from("home dir is not defined"))
        }
    }
}

pub fn get_file_data() -> Result<String, String> {
    let filepath = get_path()?;

    if Path::new(filepath.as_str()).exists() {
        match fs::File::open(filepath) {
            Ok(mut v) => {
                let mut buf = String::new();
                match v.read_to_string(&mut buf) {
                    Ok(_) => Ok(buf),
                    Err(err) => Err(err.to_string())
                }
            }
            Err(err) => Err(err.to_string())
        }

    } else {
        match fs::File::create(filepath) {
            Ok(mut v) => {
                let default_value = String::from("{ \"todos\": [] }");

                match v.write_all(default_value.as_bytes()) {
                    Ok(()) => Ok(default_value),
                    Err(err) => Err(err.to_string())
                }
            }
            Err(err) => Err(err.to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileJSON {
    pub todos: Vec<super::ToDo>,
}

pub fn save_to_file(todos: Vec<super::ToDo>) -> Result<(), String> {
    let filepath = get_path()?;
    let json = FileJSON{todos: todos};

    let data = match serde_json::to_string(&json) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string())
    };

    match fs::write(filepath, data) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}