extern crate clap;
use clap::*;
use std::{fs::File, io::prelude::*, io::BufReader, vec::Vec,};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct TaskList {
    title: String,
    task: Vec<Task>,
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

    let file = File::open("tasks.toml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let task: TaskList = toml::from_str(&contents).unwrap();

    println!("{:?}", task);

    Ok(())
}
