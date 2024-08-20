//! Module holding Zobrist Hashing info and random number generation.


use pyo3::prelude::*;
use crate::{
    castle_rights::CastleRights,
    piece::Piece
};


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct Zobrist {
    random_seed: u32,
    pub piece_keys: [[u64; 64]; 12],
    pub enpassant_keys: [u64; 64],
    pub castle_keys: [u64; 16],
    pub side_key: u64,
}


#[pymethods]
impl Zobrist {
    #[new]
    pub fn new() -> Self {
        let mut z: Zobrist = Zobrist {
            random_seed: 1804289383,
            piece_keys: [[0; 64]; 12],
            enpassant_keys: [0; 64],
            castle_keys: [0; 16],
            side_key: 0,
        };
        z.initRandomKeys();
        z
    }


    /// Generate the random Zorbist keys
    fn initRandomKeys(&mut self) {
        for piece in Piece::allPieces() {
            for sq in 0..64 {
                self.piece_keys[piece][sq] = self.getRandomU64();
            }
        }
        for sq in 0..64 {
            self.enpassant_keys[sq] = self.getRandomU64();
        }
        for idx in 0..16 {
            self.castle_keys[idx] = self.getRandomU64();
        }
        self.side_key = self.getRandomU64();
    }


    // generate 32-bit pseudo legal numbers
    fn getRandomU32(&mut self) -> u32 {
        let mut num: u32 = self.random_seed;
        num ^= num << 13;
        num ^= num >> 17;
        num ^= num << 5;
        self.random_seed = num;
        num
    }


    // generate 64-bit pseudo legal numbers
    fn getRandomU64(&mut self) -> u64 {
        let n1: u64 = self.getRandomU32() as u64 & 0xFFFF;
        let n2: u64 = self.getRandomU32() as u64 & 0xFFFF;
        let n3: u64 = self.getRandomU32() as u64 & 0xFFFF;
        let n4: u64 = self.getRandomU32() as u64 & 0xFFFF;
        n1 | (n2 << 16) | (n3 << 32) | (n4 << 48)
    }


    /// Generate a entire hashkey based on a game state
    pub fn generateHashKey(
        &self,
        bitboards: [u64; 13],
        castle_rights: [bool; 4],
        whites_turn: bool,
    ) -> u64 {
        let mut final_key: u64 = 0;
        for piece in Piece::allPieces() {
            let mut bitboard: u64 = bitboards[piece];
            let mut bitboard_ls1b: u64 = get_ls1b!(bitboard); // selects single 1 bit
            while bitboard_ls1b != 0 {
                let idx: u32 = bitboard_ls1b.leading_zeros();
                final_key ^= self.piece_keys[piece][idx as usize];
                pop_bits!(bitboard, bitboard_ls1b);
                bitboard_ls1b = get_ls1b!(bitboard);
            }
        }
        // encode enpassant column as single square
        if bitboards[Piece::EP] != 0 {
            let col: usize = bitboards[Piece::EP].leading_zeros() as usize;
            let row: usize = if whites_turn {2} else {5};
            final_key ^= self.enpassant_keys[row * 8 + col];
        }
        // encode castle rights as 4bit uint
        final_key ^= self.castle_keys[
            ((castle_rights[CastleRights::CBQ] as usize) << 3)
            | ((castle_rights[CastleRights::CBK] as usize) << 2)
            | ((castle_rights[CastleRights::CWQ] as usize) << 1)
            | (castle_rights[CastleRights::CWK] as usize)
        ];
        // hash the side only if blacks turn
        if !whites_turn {
            final_key ^= self.side_key;
        }
        final_key
    }
}