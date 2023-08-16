use std::env;
use std::process::Command;
use std::time::Instant;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please provide a command to execute and the number of executions as arguments.");
        return;
    }

    let command = &args[1];
    let num_executions: u32 = args[2].parse().unwrap_or(0);

    let file_name = format!("{}.csv", command);
    let mut file = File::create(file_name).expect("Failed to create file");

    for run_number in 1..=num_executions {
        let start = Instant::now();
        let output = Command::new("time")
            .arg("-p")
            .arg(command)
            .output()
            .expect("Failed to execute command");

        let duration = start.elapsed().as_millis();

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            println!("Command executed successfully:\n{}", result);
            let time = String::from_utf8_lossy(&output.stderr);
            println!("Output:\n{}", time);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("Command failed:\n{}", error);
        }

        let csv_line = format!("{},{:?},{}", run_number, duration, command);
        file.write_all(csv_line.as_bytes())
            .expect("Failed to write to file");
        file.write_all(b"\n").expect("Failed to write to file");
    }
}
