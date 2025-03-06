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

    let reader = BufReader::new(input_file);
    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();

    let open_result = if copy {
        let output_path = assemble_output_path(input_path);
        println!("Output path: {}", output_path.display());
        File::create(output_path)
    } else {
        File::create(input_path)
    };

    let Ok(output_file) = open_result else {
        eprintln!("Error creating output file: {}", open_result.unwrap_err());
        exit(1);
    };

    let mut writer = BufWriter::new(output_file);
    for (i, line) in lines.iter().enumerate() {
        if i < PRINT_LINES_MAX {
            println!("{:4} | {line}", i+1);
        }

        writeln!(writer, "{line}").expect("Failed to write");
    }
    if lines.len() > PRINT_LINES_MAX {
        println!("... ({} lines remaining)", lines.len() - PRINT_LINES_MAX);
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
