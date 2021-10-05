extern crate todo;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enought args");
    }

    let command = args[1].as_str();
    let all_todos = todo::get_all().unwrap();

    match command {
        "get" => {
            if args.len() == 3 {
                let id = args[2].parse::<i32>().unwrap();
                let t = todo::get_by_id(id).unwrap();
                println!("id: {}\nmessage: {}\n", t.id, t.message);

            } else {
                for t in all_todos {
                    println!("id: {}\nmessage: {}\n", t.id, t.message);
                }
            }
        }
        
        "add" => {
            if args.len() < 3 {
                panic!("not enought args");
            }

            let mut last_id = 0;
            for t in all_todos {
                if t.id > last_id {
                    last_id = t.id;
                }
            }

            let message = args[2].clone();
            todo::add(todo::ToDo{
                id: last_id+1,
                message: message,
            }).unwrap();
        }

        "del" => {
            if args.len() < 3 {
                panic!("not enought args");
            }

            let id = args[2].parse::<i32>().unwrap();
            todo::delete_by_id(id).unwrap();
        }

        _ => {
            panic!("command is not defined");
        }
    }


    // let todos = todo::get_all().unwrap();
    // for t in todos {
    //     println!("{:?}", t);
    // }

    // todo::add(todo::ToDo{
    //     id: 134,
    //     message: String::from("hello"),
    // }).unwrap()
}