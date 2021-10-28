extern crate todo;

use clap::{App, SubCommand, Arg};

fn main() {
    let matches = App::new("TODO App")
        .version("0.1.1")
        .author("Avi Parampampam <aviparampampam@gmail.com>")
        .subcommand(
            SubCommand::with_name("get")
        )
        .subcommand(
            SubCommand::with_name("add")
            .arg(
                Arg::with_name("content")
                    .required(true)
            )
        )
        .subcommand(
            SubCommand::with_name("del")
            .arg(
                Arg::with_name("id")
                .multiple(true)
                .required(true)
            )
        )
        .get_matches();

    
    let all_todos = todo::get_all().unwrap();

    if let Some(_matches) = matches.subcommand_matches("get") {
        // TODO: Print current todo
        for t in all_todos {
            println!("id: {}\nmessage: {}\n", t.id, t.message);
        }
        return
    }

    if let Some(_matches) = matches.subcommand_matches("add") {
        let content = matches.value_of("content").unwrap();

        let mut last_id = 0;
        for t in all_todos {
            if t.id > last_id {
                last_id = t.id;
            }
        }

        todo::add(todo::ToDo {
            id: last_id + 1,
            message: content.to_string(),
        }).unwrap();
        return
    } 

    if let Some(_matches) = matches.subcommand_matches("del") {
        let id_str = matches.value_of("id").unwrap();
        let id = id_str.parse::<i32>().unwrap();
        todo::delete_by_id(id).unwrap();
        return
    } 
}
