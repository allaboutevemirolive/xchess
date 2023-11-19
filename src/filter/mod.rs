pub mod op;
pub mod rating;
pub mod theme;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use self::op::OpeningTags;
use self::rating::Rating;
use self::theme::Themes;

/*

Puzzle ID: 00YeV
FEN: r2qr1k1/ppp2ppp/2nbp3/5b2/3P4/2P1BN2/P1PQBPPP/3R1RK1 b - - 7 11
Moves: d8f6 e3g5 f6g6 f3h4 h7h6 h4g6
Rating: 1385
Rating Deviation: 75
Popularity: 80
Number of Plays: 159
Themes: crushing long middlegame trappedPiece
Game URL: https://lichess.org/wqxB0iyn/black#22
Opening Tags: Scandinavian_Defense Scandinavian_Defense_Marshall_Variation


Do filter for this:

Rating: 1385
Rating Deviation: 75
Popularity: 80
Number of Plays: 159
Themes: crushing long middlegame trappedPiece
Opening Tags: Scandinavian_Defense Scandinavian_Defense_Marshall_Variation


*/

#[derive(Default)]
enum RatingFilter {
    #[default]
    LowerLimit,
    UpperLimit,
    /// Rating filter that is between lower and upper limit
    BetweenLimits,
}

#[derive(Debug)]
pub struct PuzzleInfo {
    puzzle_id: String,
    fen: String,
    moves: String,
    rating: String,
    rating_deviation: String,
    popularity: String,
    nb_plays: String,
    themes: String,
    game_url: String,
    opening_tags: String,
}

impl PuzzleInfo {
    pub fn new(fields: &[&str]) -> Self {
        PuzzleInfo {
            puzzle_id: fields.get(0).map_or(String::new(), |&s| s.to_string()),
            fen: fields.get(1).map_or(String::new(), |&s| s.to_string()),
            moves: fields.get(2).map_or(String::new(), |&s| s.to_string()),
            rating: fields.get(3).map_or(String::new(), |&s| s.to_string()),
            rating_deviation: fields.get(4).map_or(String::new(), |&s| s.to_string()),
            popularity: fields.get(5).map_or(String::new(), |&s| s.to_string()),
            nb_plays: fields.get(6).map_or(String::new(), |&s| s.to_string()),
            themes: fields.get(7).map_or(String::new(), |&s| s.to_string()),
            game_url: fields.get(8).map_or(String::new(), |&s| s.to_string()),
            opening_tags: fields.get(9).map_or(String::new(), |&s| s.to_string()),
        }
    }

    pub fn puzzle_iterator(
        reader: &mut BufReader<File>,
        writer: &mut BufWriter<File>,
        rating_filter: i32,
    ) -> std::result::Result<(), std::io::Error> {
        let mut unique_themes: HashSet<String> = HashSet::new();

        let mut unique_opening_tags: HashSet<String> = HashSet::new();

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };

            let fields: Vec<&str> = line.split(',').collect();
            let puzzle_info = PuzzleInfo::new(&fields);

            // let rating_info = Rating::new(puzzle_info);
            // rating_info.rating_filter(rating_filter, writer)?;

            let opening_tags = OpeningTags::new(puzzle_info);
            opening_tags.get_unique_opening_tags(&mut unique_opening_tags);

            // let theme_info = Themes::new(puzzle_info);
            // theme_info.get_unique_themes(&mut unique_themes);

            // puzzle_info.puzzle_info_output(writer)?;
        }

        // Themes::print_unique_themes(unique_themes, writer)?;

        OpeningTags::print_unique_opening_tags(unique_opening_tags, writer)?;

        Ok(())
    }

    pub fn puzzle_info_output(
        &self,
        writer: &mut BufWriter<File>,
    ) -> std::result::Result<(), std::io::Error> {
        writeln!(writer, "Puzzle ID: {}", self.puzzle_id)?;
        writeln!(writer, "FEN: {}", self.fen)?;
        writeln!(writer, "Moves: {}", self.moves)?;
        writeln!(writer, "Rating: {}", self.rating)?;
        writeln!(writer, "Rating Deviation: {}", self.rating_deviation)?;
        writeln!(writer, "Popularity: {}", self.popularity)?;
        writeln!(writer, "Number of Plays: {}", self.nb_plays)?;
        writeln!(writer, "Themes: {}", self.themes)?;
        writeln!(writer, "Game URL: {}", self.game_url)?;
        writeln!(writer, "Opening Tags: {}", self.opening_tags)?;
        writeln!(writer)?;

        Ok(())
    }

    pub fn is_same_puzzle(&self, other: &PuzzleInfo) -> bool {
        self.puzzle_id == other.puzzle_id
    }

    // pub fn unique_themes(&self, unique_themes: &mut HashSet<String>) {
    //     let themes: Vec<&str> = self.themes.split_whitespace().collect();

    //     for theme in themes {
    //         unique_themes.insert(theme.to_string());
    //     }
    // }

    // pub fn print_unique_themes(
    //     mut unique_themes: HashSet<String>,
    //     writer: &mut BufWriter<File>,
    // ) -> std::result::Result<(), std::io::Error> {
    //     let mut themes_vec: Vec<String> = unique_themes.drain().collect();
    //     themes_vec.sort_unstable();

    //     for unique in themes_vec {
    //         writeln!(writer, "{}", unique)?;
    //     }

    //     Ok(())
    // }
}
