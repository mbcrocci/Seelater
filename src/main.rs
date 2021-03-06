extern crate clap;
use clap::{App, SubCommand, Arg};

extern crate dirs;

use std::fs;

extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Later {
    pub strings: Vec<String>
}

fn main() {

    let path: String;
    let data: String;

    if let Some(home_dir) = dirs::home_dir() {
        path = format!("{}/.seelater.json", home_dir.display());
        data = fs::read_to_string(&path).expect("error!!");

    } else {
        return;
    }

    let mut later: Later = serde_json::from_str(&data).unwrap();

    let matches = App::new("Seelater")
        .version("1.0")
        .author("Maurizio Crocci <mbcrocci@gmail.com>")
        .subcommand(SubCommand::with_name("add")
                    .arg(Arg::with_name("item")
                         .required(true)))

        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("next")
                    .arg(Arg::with_name("index")
                         .short("i")
                         .takes_value(true)
                         .value_name("n")))
        .get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let item = add_matches.value_of("item").unwrap();
            later.strings.push(item.to_string());
            println!("Adding {}...", item);
        },
        ("list", _) => {
            for (i, item) in later.strings.iter().enumerate() {
                println!("[{}] - {}", i, item);
            }
        },
        ("next", Some(next_matches)) => {

            if later.strings.len() < 1 {
                println!("The List is empty!");
                return;
            }

            if let Some(index) = next_matches.value_of("index") {
                let i: usize = index.to_string().parse().unwrap();

                if later.strings.len() < i {
                    println!("Index out of bounds!");
                    return;
                }

                let n = later.strings.remove(i);
                println!("See: [{}] - {}", i, n);

            } else {
                let n = later.strings.remove(0);
                println!("See: {}", n);
            }
        },
        _ => unreachable!()
    };

    let save = serde_json::to_string(&later).unwrap();

    fs::write(path, save).expect("errro!!!");
}
