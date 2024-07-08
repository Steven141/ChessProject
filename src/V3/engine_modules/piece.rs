//! Enumeration of the chess pieces

use core::panic;


pub enum Piece {
    wP,
    wN,
    wB,
    wR,
    wQ,
    wK,
    bP,
    bN,
    bB,
    bR,
    bQ,
    bK,
    EP,
}


impl Piece {
    pub fn idx(&self) -> usize {
        match self {
            Piece::wP => 0,
            Piece::wN => 1,
            Piece::wB => 2,
            Piece::wR => 3,
            Piece::wQ => 4,
            Piece::wK => 5,
            Piece::bP => 6,
            Piece::bN => 7,
            Piece::bB => 8,
            Piece::bR => 9,
            Piece::bQ => 10,
            Piece::bK => 11,
            Piece::EP => 12,
        }
    }
}