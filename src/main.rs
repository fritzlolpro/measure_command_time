use serde_json::json;

use std::process::Command;
use std::result;
use std::time::Instant;

use std::fs::File;
use std::io::prelude::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(term_width = 0)]
struct Args {
    command: String,
    /// Number of executions
    #[arg(short = 'n', default_value_t = 1)]
    num_executions: i32,
    /// Output file name
    #[arg(short = 'o')]
    file_name: Option<String>,
    /// Custom fields to write into file
    #[arg(short = 'f')]
    custom_fields: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let command = args.command;
    let num_executions: i32 = args.num_executions;
    let file_name = args.file_name.unwrap_or(command.clone());

    let file_name = format!("{}.json", file_name);
    let mut file = File::create(file_name).expect("Failed to create file");

    // let custom_fields = args.custom_fields.unwrap_or_default().split(",").collect();
    let mut json_output: String = String::from("");
    for run_number in 1..=num_executions {
        let start = Instant::now();
        let output = Command::new(&command)
            .output()
            .expect("Failed to execute command");

        let duration = start.elapsed().as_millis();

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            println!("Command executed successfully:\n{}", result);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("Command failed:\n{}", error);
        }

        let mut result = json!({
            "run_number": run_number,
            "compile_duration": duration,
            "command": command
        });

        if let Some(ref custom_fields) = args.custom_fields {
            for field in custom_fields.split(",") {
                let field: Vec<&str> = field.split(":").collect();
                // Cant parse numbers in this piece of sgit will always result a string,
                // use javascript to parse json
                result
                    .as_object_mut()
                    .unwrap()
                    .insert(field[0].trim().to_string(), field[1].into());
            }
        }
        json_output.push_str(&String::from(serde_json::to_string_pretty(&result).unwrap()));
        if num_executions - run_number != 0 {
            // this piece of crap cant produce jsons need to handele comma
            json_output.push_str(&String::from(","));
        }
    }
    file.write_all(json_output.as_bytes()).expect("Failed to write to file");
    // file.write_all(b"\n").expect("Failed to write to file");
}
