//! Common Go specific definitions without dependencies to any file format
use std::fmt::{Debug, Formatter, Display};
use std::fmt;
use crate::go::GoMove::PlaceStone;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerColor {
    Black,
    White,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BoardCoordinate {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub enum GoMove {
    PlaceStone { player: PlayerColor, coordinate: BoardCoordinate },
    Pass { player: PlayerColor },
}

impl GoMove {
    pub fn place_stone(player: PlayerColor, (x, y): (u8, u8)) -> GoMove {
        PlaceStone { player, coordinate: BoardCoordinate::new(x, y) }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Handicap {
    stones: u8
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Score {
    /// Value of score in fixed point, multiplied by 10. Eg. 6.5 is represented as 65
    value: i16
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(unused)]
pub enum GameResult {
    Jigo,
    Count(PlayerColor, Option<Score>),
    Resign(PlayerColor),
    Time(PlayerColor),
    Forfeit(PlayerColor),
}

impl PlayerColor {
    pub fn pick<T>(&self, black_option: T, white_option: T) -> T {
        match self {
            PlayerColor::Black => black_option,
            PlayerColor::White => white_option
        }
    }
}

impl Handicap {
    pub fn from(stones: u8) -> Option<Handicap> {
        if stones >= 2 {
            Some(Handicap { stones })
        } else {
            None
        }
    }

    // TODO: support various board sizes and handicaps greater than 9
    pub fn handicap_points(&self) -> Vec<BoardCoordinate> {
        let mut stones = self.stones;

        let mut result = vec![];

        let board_size = 19;
        let max = board_size - 1;
        let mid = max / 2;

        if stones == 5 || stones == 7 {
            result.push(BoardCoordinate::new(mid, mid));
            stones -= 1;
        }

        let candidates: Vec<(u8, u8)> = vec![
            (3, max - 3), (max - 3, 3), (max - 3, max - 3), (3, 3), (3, mid), (max - 3, mid), (mid, 3), (mid, max - 3), (mid, mid)
        ];

        for &(x, y) in candidates.iter().take(stones as usize) {
            result.push(BoardCoordinate::new(x, y));
        }

        result
    }
}

impl BoardCoordinate {
    #[allow(unused)]
    pub fn new(x: u8, y: u8) -> BoardCoordinate {
        BoardCoordinate { x, y }
    }
}

impl Score {
    pub fn new(value: f32) -> Score {
        Score { value: (value * 10.0) as i16 }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value = (self.value as f32) / 10.0;
        f.write_fmt(format_args!("{}", value))
    }
}

impl Display for Handicap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.stones))
    }
}

impl Debug for BoardCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = (b'a' + self.x - 1) as char;
        f.write_fmt(format_args!("{}{}", c, self.y))
    }
}

impl Debug for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatting_score() {
        assert_eq!(Score::new(0.0).to_string(), "0");
        assert_eq!(Score::new(6.5).to_string(), "6.5");
        assert_eq!(Score::new(-6.5).to_string(), "-6.5");
        assert_eq!(Score::new(0.5).to_string(), "0.5");
        assert_eq!(Score::new(-0.5).to_string(), "-0.5");
        assert_eq!(Score::new(6.0).to_string(), "6");
    }

    #[test]
    fn handicap_placement() {
        fn handicap_points(stones: u8) -> Vec<(u8, u8)> {
            Handicap::from(stones).unwrap().handicap_points().iter().map(|p| (p.x, p.y)).collect()
        }

        assert_eq!(handicap_points(2), vec![(3, 15), (15, 3)]);
        assert_eq!(handicap_points(3), vec![(3, 15), (15, 3), (15, 15)]);
        assert_eq!(handicap_points(4), vec![(3, 15), (15, 3), (15, 15), (3, 3)]);
        assert_eq!(handicap_points(5), vec![(9, 9), (3, 15), (15, 3), (15, 15), (3, 3)]);
        assert_eq!(handicap_points(6), vec![(3, 15), (15, 3), (15, 15), (3, 3), (3, 9), (15, 9)]);
        assert_eq!(handicap_points(7), vec![(9, 9), (3, 15), (15, 3), (15, 15), (3, 3), (3, 9), (15, 9)]);
        assert_eq!(handicap_points(8), vec![(3, 15), (15, 3), (15, 15), (3, 3), (3, 9), (15, 9), (9, 3), (9, 15)]);
        assert_eq!(handicap_points(9), vec![(3, 15), (15, 3), (15, 15), (3, 3), (3, 9), (15, 9), (9, 3), (9, 15), (9, 9)]);
    }
}
