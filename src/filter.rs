use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub struct PuzzleInfo<'a> {
    pub puzzle_id: &'a str,
    pub fen: &'a str,
    pub moves: &'a str,
    pub rating: &'a str,
    pub rating_deviation: &'a str,
    pub popularity: &'a str,
    pub nb_plays: &'a str,
    pub themes: &'a str,
    pub game_url: &'a str,
    pub opening_tags: &'a str,
}

impl<'a> PuzzleInfo<'a> {
    pub fn new(
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
    ) -> Self {
        PuzzleInfo {
            puzzle_id,
            fen,
            moves,
            rating,
            rating_deviation,
            popularity,
            nb_plays,
            themes,
            game_url,
            opening_tags,
        }
    }

    // FIXME
    pub fn rating_filter(
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

            // FIXME
            // if fields.len() < 10 {
            //     continue;
            // }

            let puzzle_info = PuzzleInfo::new(
                fields.first().unwrap_or(&""),
                fields.get(1).unwrap_or(&""),
                fields.get(2).unwrap_or(&""),
                fields.get(3).unwrap_or(&""),
                fields.get(4).unwrap_or(&""),
                fields.get(5).unwrap_or(&""),
                fields.get(6).unwrap_or(&""),
                fields.get(7).unwrap_or(&""),
                fields.get(8).unwrap_or(&""),
                fields.get(9).unwrap_or(&""),
            );

            // FIXME
            // We can add more option on rating filter
            // - Range of rating, target > x && target < y
            // - Specfic rating, target = 3000
            // - Avoid certain range of rating
            let rating_filter = fields.get(3).unwrap_or(&"").parse::<i32>().unwrap_or(0);

            // FIXME
            // Struct fields and tags is not same. Need to check again.
            if rating_filter > nums {
                writeln!(writer, "[Event \"{}\"]", puzzle_info.puzzle_id)?;
                writeln!(writer, "[FEN \"{}\"]", puzzle_info.fen)?;
                writeln!(writer, "[White \"{}\"]", puzzle_info.moves)?;
                writeln!(writer, "[White \"{}\"]", puzzle_info.rating)?;
                writeln!(writer, "[WhiteElo \"{}\"]", puzzle_info.rating_deviation)?;
                writeln!(writer, "[Popularity \"{}\"]", puzzle_info.popularity)?;
                writeln!(writer, "[BlackElo \"{}\"]", puzzle_info.nb_plays)?;
                writeln!(writer, "[Black \"{}\"]", puzzle_info.themes)?;
                writeln!(writer, "{} 1-0\n", puzzle_info.moves)?;
            }
        }

        Ok(())
    }
}
