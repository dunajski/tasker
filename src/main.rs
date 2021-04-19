extern crate clap;
use clap::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io, io::prelude::*, io::BufReader, vec::Vec};

impl TaskList {
    fn save(&mut self) -> () {
        let stdin = io::stdin();
        let stdin = stdin.lock();
        let mut lines = stdin.lines();

        println!("Add title to your task");
        let subject = lines.next().unwrap().unwrap();

        println!("Add message to your task");
        let message = lines.next().unwrap().unwrap();

        println!("Add number to your task");
        let number = lines.next().unwrap().unwrap().parse::<u16>().unwrap();

        let new_task = Task {
            subject: subject,
            message: message,
            number: number,
        };

        self.task.push(new_task);
    }

    fn list(&mut self) -> () {
        for x in &self.task {
            println!("No: {:?}", x.number);
            println!("Subject:{:?}", x.subject);
            println!("Description:{:?}", x.message);
            println!();
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
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
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Lists all tasks"),
        )
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .help("Save task to list"),
        )
        .get_matches();

    // matches.is_present("s")
    let file = File::open("tasks.toml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut task: TaskList = toml::from_str(&contents).unwrap();

    if matches.is_present("list") {
        task.list();
    }

    if matches.is_present("save") {
        task.save();
    }

    let toml = toml::to_string(&task).unwrap();

    let mut file = File::create("tasks.toml")?;
    file.write(toml.as_bytes())?;
    Ok(())
}
