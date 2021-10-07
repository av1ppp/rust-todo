extern crate todo;

use clap::{App, Arg, SubCommand};
use std::env;

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    /*
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
            todo::add(todo::ToDo {
                id: last_id + 1,
                message: message,
            })
            .unwrap();
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
    */
}
