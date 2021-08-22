use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::env;
use std::str;

struct TodoItem {
    msg: String
}

static LOG_FILE_PATH: &str = "./logs.txt";

fn main() {
    let file: File = get_log_file().unwrap();

    let args: Vec<String> = env::args().collect();
    assert_ne!(args.len(), 1);

    let cmd: &str = &args[1].to_string();

    match cmd {
        "add" => {
            assert_ne!(args.len(), 2);
            let msg: &str = &args[2].to_string();
            add_log(file, msg).unwrap();
        }

        "get" => {
            let logs = get_logs(file);
            for l in logs {
                println!("Log: {}", l.msg);
            }
        }

        _ => panic!("Unknown command")
    }
}

/// Add message to logs
fn add_log(mut file: File, msg: &str) -> std::io::Result<()> {
    let msg_ = String::from("\n")+msg;
    file.write_all(msg_.as_bytes())
}

/// Returns all log messages
fn get_logs(file: File) -> Vec<TodoItem> {
    let mut v: Vec<TodoItem> = vec![];
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.len() > 0 {
            v.push(TodoItem{msg: String::from(line)});
        }
    }
    v
}

/// Returns a log file
fn get_log_file() -> std::io::Result<File> {
    OpenOptions::new()
        .append(true)
        .write(true)
        .read(true)
        .create(true)
        .open(LOG_FILE_PATH)
}
