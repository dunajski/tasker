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

        println!("Add description");
        let message = lines.next().unwrap().unwrap();

        let new_task = Task {
            subject: subject,
            message: message,
        };

        self.task.push(new_task);
    }

    fn list(&mut self) -> () {
        let mut z = 1u16;
        for x in &self.task {
            println!("No: {:?}", z);
            println!("Subject:{:?}", x.subject);
            println!("Description:{:?}", x.message);
            println!();
            z += 1;
        }
    }

    fn remove(&mut self, number_of_task: u16) -> () {
        let number_of_task = number_of_task.into();
        if self.task.len() < number_of_task {
            println!("There is no task with {} number.", number_of_task);
            return;
        } else {
            self.task.remove(number_of_task - 1); // -1 because indexing from 0
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
        .arg(
            Arg::with_name("remove")
                .short("r")
                .long("remove")
                .help("Remove task with given number")
                .value_name("number of task")
                .takes_value(true),
        )
        .get_matches();

    // Open file to read actual tasks
    let file = File::open("tasks.toml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // ... and serialize content to TaskList type variable
    let mut tasks: TaskList = toml::from_str(&contents).unwrap();

    // check invoked arguments
    if matches.is_present("list") {
        tasks.list();
    }

    if matches.is_present("save") {
        tasks.save();
    }

    if matches.is_present("remove") {
        let number = matches.value_of("remove").unwrap();
        tasks.remove(number.parse::<u16>().unwrap());
    }

    // convert back tasks to toml string
    let toml = toml::to_string(&tasks).unwrap();

    // and save to file
    let mut file = File::create("tasks.toml")?;
    file.write(toml.as_bytes())?;
    Ok(())
}
