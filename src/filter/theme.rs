use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};

use super::PuzzleInfo;

pub struct Themes {
    puzzle_info: PuzzleInfo,
}

impl Themes {
    pub fn new(puzzle_info: PuzzleInfo) -> Self {
        Themes { puzzle_info }
    }

    pub fn get_unique_themes(&self, unique_themes: &mut HashSet<String>) {
        let themes: Vec<&str> = self.puzzle_info.themes.split_whitespace().collect();

        for theme in themes {
            unique_themes.insert(theme.to_string());
        }
    }

    pub fn print_unique_themes(
        mut unique_themes: HashSet<String>,
        writer: &mut BufWriter<File>,
    ) -> std::result::Result<(), std::io::Error> {
        let mut themes_vec: Vec<String> = unique_themes.drain().collect();
        themes_vec.sort_unstable();

        for unique in themes_vec {
            writeln!(writer, "{}", unique)?;
        }

        Ok(())
    }
}
