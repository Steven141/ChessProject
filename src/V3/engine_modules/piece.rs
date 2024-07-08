//! Enumeration of the chess pieces


pub enum Piece {
    WP,
    WN,
    WB,
    WR,
    WQ,
    WK,
    BP,
    BN,
    BB,
    BR,
    BQ,
    BK,
    EP,
}


impl Piece {
    pub fn idx(&self) -> usize {
        match self {
            Piece::WP => 0,
            Piece::WN => 1,
            Piece::WB => 2,
            Piece::WR => 3,
            Piece::WQ => 4,
            Piece::WK => 5,
            Piece::BP => 6,
            Piece::BN => 7,
            Piece::BB => 8,
            Piece::BR => 9,
            Piece::BQ => 10,
            Piece::BK => 11,
            Piece::EP => 12,
        }
    }
}