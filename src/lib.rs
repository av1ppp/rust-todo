use std::fs;
use std::io::prelude::*;
use std::path::Path;
use dirs::home_dir;
use serde_json;

use serde::{Deserialize, Serialize};

fn get_filepath() -> Result<String, String> {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub id: i32,
    pub message: String,
}

/// Добавление ToDo записи
pub fn add(todo: ToDo) -> Result<(), String> {
    let mut todos = get_all()?;
    todos.push(todo);
    save_to_file(todos)
}

/// Получение ToDo записи по id
pub fn get_by_id(id: i32) -> Result<ToDo, String> {
    let todos = get_all()?;
    for t in todos {
        if t.id == id {
            return Ok(t)
        }
    }
    Err(String::from("todo is not defined"))
}

fn get_file_data() -> Result<String, String> {
    let filepath = get_filepath()?;

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

fn save_to_file(todos: Vec<ToDo>) -> Result<(), String> {
    let filepath = get_filepath()?;
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

#[derive(Serialize, Deserialize, Debug)]
struct FileJSON {
    todos: Vec<ToDo>,
}

/// Получение всех записей ToDo
pub fn get_all() -> Result<Vec<ToDo>, String> {
    let file_data = get_file_data()?;

    let json: FileJSON = match serde_json::from_str(file_data.as_str()) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string())
    };

    Ok(json.todos)
}

/// Удаление ToDo по id
pub fn delete_by_id(id: i32) -> Result<(), String> {
    let mut todos = get_all()?;
    for i in 0..todos.len() {
        if todos[i].id == id {
            todos.remove(i);
            save_to_file(todos)?;
            return Ok(());
        }
    };
    Err(String::from("todo is not defined"))
}
