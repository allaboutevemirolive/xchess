use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub struct FileWriter {
    pub input: BufReader<File>,
    pub output: BufWriter<File>,
}

impl FileWriter {
    pub fn new(input_path: &OsString, output_path: &OsString) -> Self {
        let input_file = File::open(input_path).expect("Failed to open input file");

        // Convert OsString to PathBuf
        let output_path_buf = PathBuf::from(output_path);

        // Ensure the output directory exists
        if let Some(parent_dir) = output_path_buf.parent() {
            if !parent_dir.exists() {
                eprintln!("Error: Output directory does not exist: {:?}", parent_dir);
                std::process::exit(1);
            }
        }

        // FIXME
        let output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&output_path_buf)
            .unwrap_or_else(|_| panic!("Failed to create or open output file: {:?}", output_path));

        let input_reader = BufReader::new(input_file);
        let output_writer = BufWriter::new(output_file);

        Self {
            input: input_reader,
            output: output_writer,
        }
    }
}
