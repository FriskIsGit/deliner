use std::{env};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Deliner utility. Creates a trimmed copy with empty lines/new lines removed.");
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let input_path = &args[1];
    let Ok(file) = File::open(input_path) else {
        eprintln!("Error opening file");
        exit(1);
    };

    let reader = BufReader::new(file);
    let output_path = assemble_output_path(input_path);
    println!("Output path: {}", output_path.display());
    let output_file = match File::create(&output_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating output file: {}", e);
            exit(1);
        }
    };

    let mut writer = BufWriter::new(output_file);

    for maybeLine in reader.lines() {
        let Ok(line) = maybeLine else {
            eprintln!("Error reading line {}", maybeLine.unwrap_err());
            exit(1);
        };
        if line.trim().is_empty() {
            continue
        }
        println!("{line}");
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
