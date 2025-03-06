use std::{env};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

const PRINT_LINES_MAX: usize = 1000;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Deliner utility.");
        println!("Overwrites given file removing all empty lines.");
        println!("Alternatively if --copy is used. The changes will be made to a separate file.");
        let executable = &args[0];
        println!("Usage: {} <path>", executable);
        println!("       {} <path> --copy", executable);
        return;
    }

    let input_path = &args[1];
    let Ok(input_file) = File::open(input_path) else {
        eprintln!("Error opening file");
        exit(1);
    };

    let copy = if args.len() > 2 && args[2] == "--copy" { true } else { false };

    let output_file;
    if copy {
        let output_path = assemble_output_path(input_path);
        println!("Output path: {}", output_path.display());
        match File::create(&output_path) {
            Ok(file) => output_file = file,
            Err(e) => {
                eprintln!("Error creating output file: {}", e);
                exit(1);
            }
        };
    } else {
        match OpenOptions::new().write(true).open(input_path) {
            Ok(file) => output_file = file,
            Err(e) => {
                eprintln!("Error opening input file: {}", e);
                exit(1);
            }
        };
    };

    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for (i, maybeLine) in reader.lines().into_iter().enumerate() {
        let Ok(line) = maybeLine else {
            eprintln!("Error reading line {}", maybeLine.unwrap_err());
            exit(1);
        };
        if line.trim().is_empty() {
            continue
        }
        if i < PRINT_LINES_MAX {
            println!("{:4}|{line}", i+1);
        }

        writeln!(writer, "{line}").expect("Failed to write");
    }

    writer.flush().expect("Final flush failed!");
}

// Function to create the output file path
fn assemble_output_path(input_path: &str) -> PathBuf {
    let in_path = Path::new(input_path);
    let prefix = match in_path.file_stem() {
        Some(stem) => stem.to_str().unwrap(),
        None => "",
    };

    let extension = match in_path.extension() {
        Some(ext) => {
            let ext = ext.to_str().unwrap();
            format!(".{ext}")
        },
        None => "".to_owned(),
    };

    let mut output_path = in_path.to_path_buf();
    let cleaned_name = format!("{prefix}_delined{extension}");
    output_path.set_file_name(cleaned_name);
    return output_path
}
