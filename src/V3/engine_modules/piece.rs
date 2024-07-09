//! Module for enumeration of the chess pieces


use std::ops::{Index, IndexMut};


#[derive(Clone, Copy)]
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
