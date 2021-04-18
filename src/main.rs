extern crate clap;
use clap::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, io::BufReader, vec::Vec};

#[derive(Deserialize, Serialize,  Debug)]
struct TaskList {
    title: String,
    task: Vec<Task>,
}

#[derive(Deserialize, Serialize, Debug)]
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
    let file = File::open("tasks.toml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut task: TaskList = toml::from_str(&contents).unwrap();

    match matches.occurrences_of("show") {
        1 => {
            for x in &task.task {
                println!("{:?}", x.subject);
                println!("{:?}", x.message);
                println!();
            }
        }
        _ => println!("I have nothing to do :<"),
    }

    // println!("{:?}", task);
    
    let new_task = Task {
        subject: "example title".to_string(),
        message: "example message".to_string(),
        number: 6,
    };
    
    task.task.push(new_task);

    println!("{:?}", task);

    let toml = toml::to_string(&task).unwrap();

    let mut file = File::create("tasks.toml")?;
    file.write(toml.as_bytes())?;
    Ok(())
}
