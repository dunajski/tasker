extern crate clap;
use clap::*;

fn main() {
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
        _ => println!("I have to do nothing :<"),
    }
}
