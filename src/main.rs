pub mod filter;
pub mod flag;
pub mod writer;

use filter::PuzzleInfo;
use flag::Flags;
use std::env;
use writer::FileWriter;

// FIXME
// Add 'HELP_TEXT'

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process_args()?;
    Ok(())
}

pub fn process_args() -> Result<(), Box<dyn std::error::Error>> {
    // FIXME
    let mut flags = Flags::new();
    flags.processing_args(env::args().collect());

    // DEBUG
    // let flags = Flags {
    //     input: "./input.csv".into(),
    //     output: "./output.csv".into(),
    //     rating_filter: 1000,
    // };

    // FIXME
    // Set default output is same as the input's folder
    let mut f_writer = FileWriter::new(&flags.input, &flags.output);

    PuzzleInfo::rating_filter(
        &mut f_writer.input,
        &mut f_writer.output,
        flags
            .rating_filter
            .try_into()
            .expect("Failed to get rating filter"),
    )?;

    Ok(())
}
