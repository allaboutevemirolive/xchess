use super::PuzzleInfo;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Default)]
enum RatingFilter {
    #[default]
    LowerLimit,
    UpperLimit,
    /// Rating filter that is between lower and upper limit
    BetweenLimits,
}

pub struct Rating {
    puzzle_info: PuzzleInfo,
}

impl Rating {
    pub fn new(puzzle_info: PuzzleInfo) -> Self {
        Rating {
            puzzle_info,
        }
    }

    pub fn rating_filter(
        &self,
        rating_filter: i32,
        writer: &mut BufWriter<File>,
    ) -> std::result::Result<(), std::io::Error> {
        let ratings = self.parse_rating().unwrap_or(0);
        let default_filter = Default::default();

        match default_filter {
            // We want rating that is more than rating_filter
            // rating_filter is a lower limit
            RatingFilter::LowerLimit => {
                if ratings > rating_filter {
                    writeln!(writer, "[Event \"{}\"]", self.puzzle_info.puzzle_id)?;
                    writeln!(writer, "[FEN \"{}\"]", self.puzzle_info.fen)?;
                    writeln!(writer, "[White \"{}\"]", self.puzzle_info.rating)?;
                    writeln!(
                        writer,
                        "[WhiteElo \"{}\"]",
                        self.puzzle_info.rating_deviation
                    )?;
                    writeln!(writer, "[Popularity \"{}\"]", self.puzzle_info.popularity)?;
                    writeln!(writer, "[BlackElo \"{}\"]", self.puzzle_info.nb_plays)?;
                    writeln!(writer, "[Black \"{}\"]", self.puzzle_info.themes)?;
                    writeln!(writer, "{} 1-0\n", self.puzzle_info.moves)?;
                }
            }

            // We want rating that is less than rating_filter
            // rating_filter is an upper limit
            RatingFilter::UpperLimit => {
                if ratings < rating_filter {
                    writeln!(writer, "[Event \"{}\"]", self.puzzle_info.puzzle_id)?;
                    writeln!(writer, "[FEN \"{}\"]", self.puzzle_info.fen)?;
                    writeln!(writer, "[White \"{}\"]", self.puzzle_info.rating)?;
                    writeln!(
                        writer,
                        "[WhiteElo \"{}\"]",
                        self.puzzle_info.rating_deviation
                    )?;
                    writeln!(writer, "[Popularity \"{}\"]", self.puzzle_info.popularity)?;
                    writeln!(writer, "[BlackElo \"{}\"]", self.puzzle_info.nb_plays)?;
                    writeln!(writer, "[Black \"{}\"]", self.puzzle_info.themes)?;
                    writeln!(writer, "{} 1-0\n", self.puzzle_info.moves)?;
                }
            }

            // FIXME
            RatingFilter::BetweenLimits => {}
        }

        Ok(())
    }

    pub fn parse_rating(&self) -> std::result::Result<i32, std::num::ParseIntError> {
        self.puzzle_info.rating.parse()
    }
}
