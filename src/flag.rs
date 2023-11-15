use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub struct Flags {
    pub input: OsString,
    pub output: OsString,
    pub rating_filter: u32,
}

impl Flags {
    // FIXME
    // Set 'proper' default output
    pub fn new() -> Self {
        Default::default()
    }

    pub fn processing_args(&mut self, args: Vec<String>) {
        for arg in args.iter() {
            // Command-line argument "--input=<value>"
            // Example: --input=/home/nemesis/Documents/Github/Focus/game/xchess/csv/input.csv
            if arg.starts_with("--input=") {
                self.input = arg.trim_start_matches("--input=").into();
            }
            // Command-line argument "--output=<value>"
            // Example: --output=/home/nemesis/Documents/Github/Focus/game/xchess/csv/output.csv
            else if arg.starts_with("--output=") {
                self.output = arg.trim_start_matches("--output=").into();
            }
            // Command-line argument "--rating=<value>"
            // Example: --rating=2000
            else if arg.starts_with("--rating=") {
                if let Ok(rating) = arg.trim_start_matches("--rating=").parse() {
                    self.rating_filter = rating;
                }
            }
        }

        // If --output argument is not provided, set output file to 
        // be in the same folder as the input file with the name "output.csv"
        if self.output.is_empty() {
            let input_path = Path::new(&self.input);
            let output_path = input_path
                .parent()
                .map(|parent| parent.join("output.csv"))
                .unwrap_or_else(|| PathBuf::from("output.csv"));

            self.output = output_path.into_os_string();
        }
    }
}
