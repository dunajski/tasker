extern crate clap;
use clap::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, io::BufReader, vec::Vec};

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
                .takes_value(true)
                .number_of_values(3)
                .value_names(&["SUBJECT", "MESSAGE", "NUMBER"])
                .help("Save task to list"),
        )
        .get_matches();

    // matches.is_present("s")
    let file = File::open("tasks.toml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut task: TaskList = toml::from_str(&contents).unwrap();

    match matches.occurrences_of("list") {
        1 => {
            for x in &task.task {
                println!("{:?}", x.subject);
                println!("{:?}", x.message);
                println!();
            }
        }
        _ => println!("I have nothing to do :<"),
    }

    if matches.is_present("save") {
        let vals: Vec<&str> = matches.values_of("save").unwrap().collect();

        println!("vals{:?}", vals);
        // println!("{:?}", task);

        let new_task2 = Task {
            subject: vals[0].to_string(),
            message: vals[1].to_string(),
            number: vals[2].parse::<u16>().unwrap(),
        };

        println!("TASK 2 {:?}", new_task2);
        task.task.push(new_task2);
    }

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
