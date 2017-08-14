use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

#[derive(Debug)]
struct Process {
    name: String,
    command: String
}

fn main() {
    let procfile = load_procfile().unwrap();
    let processes = split_procfile(&procfile);

    for p in processes.into_iter() {
        print!("{:?}", p);
    }

    // Start processes
    // If one process dies, kill all the rest.
    // Handle Ctrl-C
}

fn load_procfile() -> Result<String> {
    let mut file = File::open("Procfile")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn split_procfile(procfile: &str) -> Vec<Process> {
    let lines = procfile.split("\n");

    // FIXME: Problem now is that l can be an empty string.
    lines.map(|l| {
        println!("{:?}", l);
        let line_data = l.split(":").collect::<Vec<&str>>();
        let (process_name, command) = (line_data[0], line_data[1]);

        Process {
            name: String::from(process_name),
            command: String::from(command)
        }
    }).collect()
}
