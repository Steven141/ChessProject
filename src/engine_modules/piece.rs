//! Module for enumeration of the chess pieces


use pyo3::prelude::*;
use std::ops::{
    Index,
    IndexMut,
};


#[pyclass(module = "ChessProject", get_all, set_all)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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


/*
Used for indexing operations (container[index]) in immutable contexts.

container[index] is actually syntactic sugar for *container.index(index)

Allows let value = v[index] if the type of value implements Copy
*/
impl<T> Index<Piece> for [T] {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        match index {
            Piece::WP => &self[0],
            Piece::WN => &self[1],
            Piece::WB => &self[2],
            Piece::WR => &self[3],
            Piece::WQ => &self[4],
            Piece::WK => &self[5],
            Piece::BP => &self[6],
            Piece::BN => &self[7],
            Piece::BB => &self[8],
            Piece::BR => &self[9],
            Piece::BQ => &self[10],
            Piece::BK => &self[11],
            Piece::EP => &self[12],
        }
    }
}


/*
Used for indexing operations (container[index]) in mutable contexts.

container[index] is actually syntactic sugar for *container.index_mut(index)

Allows v[index] = value
*/
impl<T> IndexMut<Piece> for [T] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        match index {
            Piece::WP => &mut self[0],
            Piece::WN => &mut self[1],
            Piece::WB => &mut self[2],
            Piece::WR => &mut self[3],
            Piece::WQ => &mut self[4],
            Piece::WK => &mut self[5],
            Piece::BP => &mut self[6],
            Piece::BN => &mut self[7],
            Piece::BB => &mut self[8],
            Piece::BR => &mut self[9],
            Piece::BQ => &mut self[10],
            Piece::BK => &mut self[11],
            Piece::EP => &mut self[12],
        }
    }
}


/// Allows for equality between enum and char
impl PartialEq<char> for Piece {
    fn eq(&self, other: &char) -> bool {
        match self {
            Piece::WP => *other == 'P',
            Piece::WN => *other == 'N',
            Piece::WB => *other == 'B',
            Piece::WR => *other == 'R',
            Piece::WQ => *other == 'Q',
            Piece::WK => *other == 'K',
            Piece::BP => *other == 'p',
            Piece::BN => *other == 'n',
            Piece::BB => *other == 'b',
            Piece::BR => *other == 'r',
            Piece::BQ => *other == 'q',
            Piece::BK => *other == 'k',
            _ => panic!("CANNOT EQUATE THIS VALUE"),
        }
    }
}


/// Specific groups of enums that are useful
impl Piece {
    pub fn whitePiecesNoKing() -> [Piece; 5] {
        [
            Piece::WP,
            Piece::WN,
            Piece::WB,
            Piece::WR,
            Piece::WQ,
        ]
    }


    pub fn blackPiecesNoKing() -> [Piece; 5] {
        [
            Piece::BP,
            Piece::BN,
            Piece::BB,
            Piece::BR,
            Piece::BQ,
        ]
    }


    pub fn whitePiecesWithEnemyKing() -> [Piece; 7] {
        [
            Piece::WP,
            Piece::WN,
            Piece::WB,
            Piece::WR,
            Piece::WQ,
            Piece::WK,
            Piece::BK,
        ]
    }


    pub fn blackPiecesWithEnemyKing() -> [Piece; 7] {
        [
            Piece::BP,
            Piece::BN,
            Piece::BB,
            Piece::BR,
            Piece::BQ,
            Piece::WK,
            Piece::BK,
        ]
    }


    pub fn allPieces() -> [Piece; 12] {
        [
            Piece::WP,
            Piece::WN,
            Piece::WB,
            Piece::WR,
            Piece::WQ,
            Piece::WK,
            Piece::BP,
            Piece::BN,
            Piece::BB,
            Piece::BR,
            Piece::BQ,
            Piece::BK,
        ]
    }
}
