use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub id: i32,
    pub message: String,
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

/// Добавление ToDo записи
pub fn add(todo: ToDo) -> Result<(), String> {
    let mut todos = get_all()?;
    todos.push(todo);
    super::file::save_to_file(todos)
}


/// Получение всех записей ToDo
pub fn get_all() -> Result<Vec<ToDo>, String> {
    let file_data = super::file::get_file_data()?;

    let json: super::file::FileJSON = match serde_json::from_str(file_data.as_str()) {
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
            super::file::save_to_file(todos)?;
            return Ok(());
        }
    };
    Err(String::from("todo is not defined"))
}
