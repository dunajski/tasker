extern crate clap;
use clap::*;
use std::{fs::File, io::prelude::*, io::BufReader, io::BufWriter};
use serde::{Deserialize, Serialize};
use toml::*;

#[derive(Deserialize, Debug)]
struct Taskito {
    text: String,
}

#[derive(Serialize, Debug)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Serialize, Debug)]
struct Keys {
    github: String,
    travis: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TaskList {
    title: String,
    task: Task,
}

#[derive(Deserialize, Debug)]
struct Task {
    subject: String,
    message: String,
    number: u16,
}


fn main() -> std::io::Result<()> {
    let matches = App::new("Task Saver")
        .version("1.0")
        .author("dnj")
        .about("It's my first App in Rust to maintain tasks")
        .arg(
            Arg::with_name("show")
                .short("s")
                .long("show")
                .help("Shows list of tasks"),
        )
        .get_matches();

    // matches.is_present("s")
    match matches.occurrences_of("show") {
        1 => println!("show tasks!"),
        _ => println!("I have nothing to do :<"),
    }

    let file = File::open("tasks.json")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("Co tam jest? {}", contents);

    let task: Taskito = serde_json::from_str(&contents).unwrap();
    println!("task = {:?}", task);

    let config = Config {
        ip: "127.0.0.1".to_string(),
        port: None,
        keys: Keys {
            github: "xxxxxxxxxxxxxxxxx".to_string(),
            travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
        },
    };

    let toml = toml::to_string(&config).unwrap();

    let mut file = File::create("config.toml")?;
    file.write(toml.as_bytes())?;

    println!("{:?}", config);

    ////////////////////////
    let file = File::open("tasks.toml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let task: TaskList = toml::from_str(&contents).unwrap();

    println!("{:?}", task);

    Ok(())
}
