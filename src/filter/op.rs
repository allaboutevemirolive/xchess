use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};

use super::PuzzleInfo;

pub struct OpeningTags {
    puzzle_info: PuzzleInfo,
}

impl OpeningTags {
    pub fn new(puzzle_info: PuzzleInfo) -> Self {
        OpeningTags { puzzle_info }
    }

    pub fn get_unique_opening_tags(&self, unique_op_tags: &mut HashSet<String>) {
        let op_tags: Vec<&str> = self.puzzle_info.opening_tags.split_whitespace().collect();

        for op_tag in op_tags {
            unique_op_tags.insert(op_tag.to_string());
        }
    }

    pub fn print_unique_opening_tags(
        mut unique_opening_tags: HashSet<String>,
        writer: &mut BufWriter<File>,
    ) -> std::result::Result<(), std::io::Error> {
        let mut op_tags_vec: Vec<String> = unique_opening_tags.drain().collect();
        op_tags_vec.sort_unstable();

        for op_tag in op_tags_vec {
            writeln!(writer, "{}", op_tag)?;
        }

        Ok(())
    }
}
