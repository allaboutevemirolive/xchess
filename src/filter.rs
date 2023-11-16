use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Debug)]
pub struct PuzzleInfo<'a> {
    puzzle_id: &'a str,
    fen: &'a str,
    moves: &'a str,
    rating: &'a str,
    rating_deviation: &'a str,
    popularity: &'a str,
    nb_plays: &'a str,
    themes: &'a str,
    game_url: &'a str,
    opening_tags: &'a str,
}

impl<'a> PuzzleInfo<'a> {
    pub fn new(fields: &'a [&'a str]) -> Self {
        PuzzleInfo {
            puzzle_id: fields.get(0).unwrap_or(&""),
            fen: fields.get(1).unwrap_or(&""),
            moves: fields.get(2).unwrap_or(&""),
            rating: fields.get(3).unwrap_or(&""),
            rating_deviation: fields.get(4).unwrap_or(&""),
            popularity: fields.get(5).unwrap_or(&""),
            nb_plays: fields.get(6).unwrap_or(&""),
            // TODO: Make filter based on themes
            themes: fields.get(7).unwrap_or(&""),
            game_url: fields.get(8).unwrap_or(&""),
            opening_tags: fields.get(9).unwrap_or(&""),
        }
    }

    pub fn puzzle_iterator(
        reader: &mut BufReader<File>,
        writer: &mut BufWriter<File>,
        nums: i32,
    ) -> Result<(), std::io::Error> {
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };

            let fields: Vec<&str> = line.split(',').collect();
            let puzzle_info = PuzzleInfo::new(&fields);

            // TODO
            puzzle_info.pgn_rating_filter(nums, writer)?;
        }

        Ok(())
    }

    /// Customized rating filter for PGN output
    fn pgn_rating_filter(
        &self,
        nums: i32,
        writer: &mut BufWriter<File>,
    ) -> Result<(), std::io::Error> {
        let rating_filter = self.parse_rating().unwrap_or(0);

        if rating_filter > nums {
            writeln!(writer, "[Event \"{}\"]", self.puzzle_id)?;
            writeln!(writer, "[FEN \"{}\"]", self.fen)?;
            writeln!(writer, "[White \"{}\"]", self.rating)?;
            writeln!(writer, "[WhiteElo \"{}\"]", self.rating_deviation)?;
            writeln!(writer, "[Popularity \"{}\"]", self.popularity)?;
            writeln!(writer, "[BlackElo \"{}\"]", self.nb_plays)?;
            writeln!(writer, "[Black \"{}\"]", self.themes)?;
            writeln!(writer, "{} 1-0\n", self.moves)?;
        }

        Ok(())
    }

    pub fn parse_rating(&self) -> Result<i32, std::num::ParseIntError> {
        self.rating.parse()
    }

    // FIXME
    pub fn is_same_puzzle(&self, other: &PuzzleInfo<'a>) -> bool {
        self.puzzle_id == other.puzzle_id
    }
    
}
