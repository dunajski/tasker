extern crate clap;
use clap::*;
use std::{fs::File, io::prelude::*, io::BufReader};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Task {
    text: String,
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
    // println!("Co tam jest? {}", contents);

    let task: Task = serde_json::from_str(&contents).unwrap();
    println!("task = {:?}", task);
    Ok(())
}
