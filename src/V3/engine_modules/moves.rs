//! Module holding information about all the moves


use pyo3::prelude::*;
use std::str::from_utf8;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::{
    castle_rights::CastleRights,
    special_bitboards::SpecialBitBoards,
    piece::Piece,
    zobrist::Zobrist,
};


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct Moves {
    pub castle_rooks: [usize; 4], // squares from top-left to bottom-right
    pub masks: SpecialBitBoards,
    pub checkmate: bool,
    pub stalemate: bool,
}


#[pymethods]
impl Moves {
    #[new]
    pub fn new() -> Self {
        Moves {
            castle_rooks: [0, 7, 56, 63],
            masks: SpecialBitBoards::new(),
            checkmate: false,
            stalemate: false,
        }
    }


    pub fn getValidMoves(&mut self, z: &mut Zobrist, bitboards: [u64; 13], castle_rights: [bool; 4], hash_key: u64, whites_turn: bool, depth: u32) -> String {
        let mut moves: String = self.getPossibleMoves(bitboards, castle_rights, whites_turn);
        if depth == 0 {
            // TODO: look to replace shuffling with sorting
            let mut move_groups: Vec<&str> = moves.as_bytes().chunks(4).map(|chunk| from_utf8(chunk).unwrap()).collect();
            move_groups.shuffle(&mut thread_rng());
            moves = move_groups.join("");
        }
        let mut valid_moves: String = String::new();
        for i in (0..moves.len()).step_by(4) {
            let (bitboards_t, _) = self.getUpdatedBitboards(z, &moves[i..i+4], bitboards, hash_key, whites_turn);
            if self.isValidMove(bitboards_t, whites_turn) {
                valid_moves += &moves[i..i+4];
            }
        }
        if valid_moves.len() == 0 {
            if self.isKingAttacked(bitboards, whites_turn) {
                self.checkmate = true;
            } else {
                self.stalemate = true;
            }
        } else {
            self.checkmate = false;
            self.stalemate = false;
        }
        valid_moves
    }


    pub fn makeMove(&self, z: &mut Zobrist, mut bitboard: u64, mut hash_key: u64, move_str: &str, p_type: Piece) -> (u64, u64) {
        let start_sq: u32; let end_sq: u32;
        let start_bitboard: u64; let end_bitboard: u64;
        if move_str.chars().nth(3).unwrap().is_numeric() { // regular move
            let (r1, c1, r2, c2) = move_to_u32s!(move_str);
            start_sq = r1 * 8 + c1;
            end_sq = r2 * 8 + c2;
            if get_bit!(bitboard, end_sq) == 1 {
                hash_key ^= z.piece_keys[p_type][end_sq as usize]; // remove taken piece from hash
            }
            if get_bit!(bitboard, start_sq) == 1 {
                hash_key ^= z.piece_keys[p_type][start_sq as usize]; // remove source piece from hash
                hash_key ^= z.piece_keys[p_type][end_sq as usize]; // add target piece to hash
                pop_bit!(bitboard, start_sq);
                set_bit!(bitboard, end_sq);
            } else {
                pop_bit!(bitboard, end_sq);
            }
        } else if move_str.chars().nth(3).unwrap() == 'P' { // pawn promo
            let whites_turn: bool = move_str.chars().nth(2).unwrap().is_uppercase();
            let (c1, c2, _, _) = move_to_u32s!(move_str);
            let (r1, r2) = if whites_turn {(1, 0)} else {(6, 7)};
            if whites_turn { // white promo
                start_bitboard = self.masks.file_masks[c1 as usize] & self.masks.rank_masks[1];
                end_bitboard = self.masks.file_masks[c2 as usize] & self.masks.rank_masks[0];
            } else { // black promo
                start_bitboard = self.masks.file_masks[c1 as usize] & self.masks.rank_masks[6];
                end_bitboard = self.masks.file_masks[c2 as usize] & self.masks.rank_masks[7];
            }
            start_sq = start_bitboard.leading_zeros();
            end_sq = end_bitboard.leading_zeros();
            if get_bit!(bitboard, end_sq) == 1 {
                hash_key ^= z.piece_keys[p_type][end_sq as usize]; // remove taken piece from hash
            }
            if get_bit!(bitboard, start_sq) == 1 {
                hash_key ^= z.piece_keys[p_type][start_sq as usize]; // remove source piece from hash
            }
            if p_type == move_str.chars().nth(2).unwrap() {
                hash_key ^= z.piece_keys[p_type][end_sq as usize]; // add promoted piece to hash
                set_bit!(bitboard, end_sq);
            } else {
                pop_bit!(bitboard, start_sq);
                pop_bit!(bitboard, end_sq);
            }
        } else if move_str.chars().nth(3).unwrap() == 'E' { // enpassant
            let whites_turn: bool = move_str.chars().nth(2).unwrap() == 'w';
            let (c1, c2, _, _) = move_to_u32s!(move_str);
            let (r1, r2) = if whites_turn {(3, 2)} else {(4, 5)};
            if whites_turn { // white
                start_bitboard = self.masks.file_masks[c1 as usize] & self.masks.rank_masks[3];
                end_bitboard = self.masks.file_masks[c2 as usize] & self.masks.rank_masks[2];
                end_sq = end_bitboard.leading_zeros();
                if get_bit!(bitboard, end_sq + 8) == 1 {
                    hash_key ^= z.piece_keys[p_type][(r1 * 8 + c2) as usize] // remove taken piece from hash
                }
                pop_bits!(bitboard, self.masks.file_masks[c2 as usize] & self.masks.rank_masks[3]);
            } else { // black
                start_bitboard = self.masks.file_masks[c1 as usize] & self.masks.rank_masks[4];
                end_bitboard = self.masks.file_masks[c2 as usize] & self.masks.rank_masks[5];
                end_sq = end_bitboard.leading_zeros();
                if get_bit!(bitboard, end_sq - 8) == 1 {
                    hash_key ^= z.piece_keys[p_type][(r1 * 8 + c2) as usize] // remove taken piece from hash
                }
                pop_bits!(bitboard, self.masks.file_masks[c2 as usize] & self.masks.rank_masks[4]);
            }
            start_sq = start_bitboard.leading_zeros();
            if get_bit!(bitboard, start_sq) == 1 {
                hash_key ^= z.piece_keys[p_type][start_sq as usize]; // remove source piece from hash
                hash_key ^= z.piece_keys[p_type][end_sq as usize]; // add target piece to hash
                pop_bit!(bitboard, start_sq);
                set_bit!(bitboard, end_sq);
            }
        } else {
            panic!("INVALID MOVE TYPE");
        }
        (bitboard, hash_key)
    }


    pub fn makeMoveCastle(&self, z: &mut Zobrist, mut rook: u64, king: u64, mut hash_key: u64, move_str: &str, p_type: Piece) -> (u64, u64) {
        let (r1, c1, _, _) = move_to_u32s!(move_str);
        let start_sq: u32 = r1 * 8 + c1;
        if get_bit!(king, start_sq) == 1 && ((move_str == "0402") || (move_str == "0406") || (move_str == "7472") || (move_str == "7476")) {
            if p_type == Piece::WR { // white
                match move_str {
                    "7476" => { // king side
                        hash_key ^= z.piece_keys[p_type][self.castle_rooks[3]];
                        hash_key ^= z.piece_keys[p_type][self.castle_rooks[3] - 2];
                        pop_bit!(rook, self.castle_rooks[3]);
                        set_bit!(rook, self.castle_rooks[3] - 2);
                    },
                    "7472" => { // queen side
                        hash_key ^= z.piece_keys[p_type][self.castle_rooks[2]];
                        hash_key ^= z.piece_keys[p_type][self.castle_rooks[2] + 3];
                        pop_bit!(rook, self.castle_rooks[2]);
                        set_bit!(rook, self.castle_rooks[2] + 3);
                    },
                    _ => (),
                }
            } else { // black
                match move_str {
                    "0406" => { // king side
                        hash_key ^= z.piece_keys[p_type][self.castle_rooks[1]];
                        hash_key ^= z.piece_keys[p_type][self.castle_rooks[1] - 2];
                        pop_bit!(rook, self.castle_rooks[1]);
                        set_bit!(rook, self.castle_rooks[1] - 2);
                    },
                    "0402" => { // queen side
                        hash_key ^= z.piece_keys[p_type][self.castle_rooks[0]];
                        hash_key ^= z.piece_keys[p_type][self.castle_rooks[0] + 3];
                        pop_bit!(rook, self.castle_rooks[0]);
                        set_bit!(rook, self.castle_rooks[0] + 3);
                    },
                    _ => (),
                }
            }
        }
        (rook, hash_key)
    }


    pub fn makeMoveEP(&self, z: &mut Zobrist, ep: u64, bitboard: u64, mut hash_key: u64, move_str: &str, whites_turn: bool) -> (u64, u64) {
        // remove current enpassant status from hash
        if ep != 0 {
            let col: usize = ep.leading_zeros() as usize;
            let row: usize = if whites_turn {2} else {5};
            hash_key ^= z.enpassant_keys[row * 8 + col];
        }
        let mut ep_t: u64 = 0;
        if move_str.chars().nth(3).unwrap().is_numeric() {
            let (r1, c1, r2, _) = move_to_u32s!(move_str);
            if (r1 as i32 - r2 as i32).abs() == 2 && get_bit!(bitboard, r1 * 8 + c1) == 1 {
                ep_t = self.masks.file_masks[c1 as usize];
                let col: usize = ep_t.leading_zeros() as usize;
                let row: usize = if !whites_turn {2} else {5};
                hash_key ^= z.enpassant_keys[row * 8 + col]; // add next move enpassant status to hash
            }
        }
        (ep_t, hash_key)
    }


    pub fn getPossibleMoves(&mut self, bitboards: [u64; 13], castle_rights: [bool; 4], whites_turn: bool) -> String {
        if whites_turn {self.possibleMovesW(bitboards, castle_rights)}
        else {self.possibleMovesB(bitboards, castle_rights)}
    }


    pub fn possibleMovesW(&mut self, bitboards: [u64; 13], castle_rights: [bool; 4]) -> String {
        self.masks.not_allied_pieces = !or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BK], bitboards); // avoid illegal bK capture
        self.masks.enemy_pieces = or_array_elems!([Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ], bitboards); // avoid illegal bK capture
        self.masks.occupied = or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK], bitboards);
        self.masks.empty = !self.masks.occupied;
        self.possibleWP(bitboards[Piece::WP], bitboards[Piece::BP], bitboards[Piece::EP])
            + &self.possibleB(bitboards[Piece::WB])
            + &self.possibleQ(bitboards[Piece::WQ])
            + &self.possibleR(bitboards[Piece::WR])
            + &self.possibleN(bitboards[Piece::WN])
            + &self.possibleK(bitboards[Piece::WK])
            + &self.possibleCastleW(bitboards, castle_rights)
    }


    pub fn possibleMovesB(&mut self, bitboards: [u64; 13], castle_rights: [bool; 4]) -> String {
        self.masks.not_allied_pieces = !or_array_elems!([Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK, Piece::WK], bitboards); // avoid illegal wK capture
        self.masks.enemy_pieces = or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ], bitboards); // avoid illegal bK capture
        self.masks.occupied = or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK], bitboards);
        self.masks.empty = !self.masks.occupied;
        self.possibleBP(bitboards[Piece::WP], bitboards[Piece::BP], bitboards[Piece::EP])
            + &self.possibleB(bitboards[Piece::BB])
            + &self.possibleQ(bitboards[Piece::BQ])
            + &self.possibleR(bitboards[Piece::BR])
            + &self.possibleN(bitboards[Piece::BN])
            + &self.possibleK(bitboards[Piece::BK])
            + &self.possibleCastleB(bitboards, castle_rights)
    }


    fn possibleWP(&self, wP: u64, bP: u64, EP: u64) -> String {
        // standard moves and captures
        let mut move_list: String = String::new(); // r1,c1,r2,c2
        let mut moves: u64 = (wP << 7) & self.masks.enemy_pieces & !self.masks.rank_masks[0] & !self.masks.file_masks[0]; // right capture
        let mut possible_move: u64 = get_ls1b!(moves); // selects single possible move
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) + 1, (idx % 8) - 1, idx / 8, idx % 8);
            pop_bits!(moves, possible_move); // remove current move from moves
            possible_move = get_ls1b!(moves); // get next possible move
        }

        moves = (wP << 9) & self.masks.enemy_pieces & !self.masks.rank_masks[0] & !self.masks.file_masks[7]; // left capture
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) + 1, (idx % 8) + 1, idx / 8, idx % 8);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (wP << 8) & self.masks.empty & !self.masks.rank_masks[0]; // move forward 1
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) + 1, idx % 8, idx / 8, idx % 8);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (wP << 16) & self.masks.empty & (self.masks.empty << 8) & self.masks.rank_masks[4]; // move forward 2
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) + 2, idx % 8, idx / 8, idx % 8);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        // pawn promotion, move_list -> c1,c2,promo type,'P'
        moves = (wP << 7) & self.masks.enemy_pieces & self.masks.rank_masks[0] & !self.masks.file_masks[0]; // promo by right capture
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) - 1; let c2 = idx % 8;
            move_list += &format!("{}{}QP{}{}RP{}{}BP{}{}NP", c1, c2, c1, c2, c1, c2, c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (wP << 9) & self.masks.enemy_pieces & self.masks.rank_masks[0] & !self.masks.file_masks[7]; // promo by left capture
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) + 1; let c2 = idx % 8;
            move_list += &format!("{}{}QP{}{}RP{}{}BP{}{}NP", c1, c2, c1, c2, c1, c2, c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (wP << 8) & self.masks.empty & self.masks.rank_masks[0]; // promo by move forward 1
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = idx % 8; let c2 = idx % 8;
            move_list += &format!("{}{}QP{}{}RP{}{}BP{}{}NP", c1, c2, c1, c2, c1, c2, c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        // enpassant, move_list -> c1,c2,'wE'
        moves = (wP >> 1) & bP & self.masks.rank_masks[3] & !self.masks.file_masks[0] & EP; // enpassant right
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) - 1; let c2 = idx % 8;
            move_list += &format!("{}{}wE", c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (wP << 1) & bP & self.masks.rank_masks[3] & !self.masks.file_masks[7] & EP; // enpassant left
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) + 1; let c2 = idx % 8;
            move_list += &format!("{}{}wE", c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }
        move_list
    }


    fn possibleBP(&self, wP: u64, bP: u64, EP: u64) -> String {
        // standard moves and captures
        let mut move_list: String = String::new(); // r1,c1,r2,c2
        let mut moves: u64 = (bP >> 7) & self.masks.enemy_pieces & !self.masks.rank_masks[7] & !self.masks.file_masks[7]; // right capture
        let mut possible_move: u64 = get_ls1b!(moves); // selects single possible move
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) - 1, (idx % 8) + 1, idx / 8, idx % 8);
            pop_bits!(moves, possible_move); // remove current move from moves
            possible_move = get_ls1b!(moves); // get next possible move
        }

        moves = (bP >> 9) & self.masks.enemy_pieces & !self.masks.rank_masks[7] & !self.masks.file_masks[0]; // left capture
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) - 1, (idx % 8) - 1, idx / 8, idx % 8);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (bP >> 8) & self.masks.empty & !self.masks.rank_masks[7]; // move forward 1
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) - 1, idx % 8, idx / 8, idx % 8);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (bP >> 16) & self.masks.empty & (self.masks.empty >> 8) & self.masks.rank_masks[3]; // move forward 2
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) - 2, idx % 8, idx / 8, idx % 8);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        // pawn promotion, move_list -> c1,c2,promo type,'P'
        moves = (bP >> 7) & self.masks.enemy_pieces & self.masks.rank_masks[7] & !self.masks.file_masks[7]; // promo by right capture
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) + 1; let c2 = idx % 8;
            move_list += &format!("{}{}qP{}{}rP{}{}bP{}{}nP", c1, c2, c1, c2, c1, c2, c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (bP >> 9) & self.masks.enemy_pieces & self.masks.rank_masks[7] & !self.masks.file_masks[0]; // promo by left capture
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) - 1; let c2 = idx % 8;
            move_list += &format!("{}{}qP{}{}rP{}{}bP{}{}nP", c1, c2, c1, c2, c1, c2, c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (bP >> 8) & self.masks.empty & self.masks.rank_masks[7]; // promo by move forward 1
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = idx % 8; let c2 = idx % 8;
            move_list += &format!("{}{}qP{}{}rP{}{}bP{}{}nP", c1, c2, c1, c2, c1, c2, c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        // enpassant, move_list -> c1,c2,'wE'
        moves = (bP << 1) & wP & self.masks.rank_masks[4] & !self.masks.file_masks[7] & EP; // enpassant right
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) + 1; let c2 = idx % 8;
            move_list += &format!("{}{}bE", c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }

        moves = (bP >> 1) & wP & self.masks.rank_masks[4] & !self.masks.file_masks[0] & EP; // enpassant left
        possible_move = get_ls1b!(moves);
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) - 1; let c2 = idx % 8;
            move_list += &format!("{}{}bE", c1, c2);
            pop_bits!(moves, possible_move);
            possible_move = get_ls1b!(moves);
        }
        move_list
    }


    fn possibleB(&self, mut B: u64) -> String {
        let mut move_list: String = String::new();
        let mut bishop: u64 = get_ls1b!(B);
        while bishop != 0 {
            let bishop_idx: usize = bishop.leading_zeros() as usize;
            let mut moves: u64 = self.possibleDiagAndAntiDiagMoves(bishop_idx) & self.masks.not_allied_pieces;
            let mut possible_move: u64 = get_ls1b!(moves); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", bishop_idx / 8, bishop_idx % 8, move_idx / 8, move_idx % 8);
                pop_bits!(moves, possible_move); // remove current possible move
                possible_move = get_ls1b!(moves);
            }

            pop_bits!(B, bishop); // remove current bishop
            bishop = get_ls1b!(B);
        }
        move_list
    }


    fn possibleQ(&self, mut Q: u64) -> String {
        let mut move_list: String = String::new();
        let mut queen: u64 = get_ls1b!(Q);
        while queen != 0 {
            let queen_idx: usize = queen.leading_zeros() as usize;
            let mut moves: u64 = (self.possibleDiagAndAntiDiagMoves(queen_idx) | self.possibleHAndVMoves(queen_idx)) & self.masks.not_allied_pieces;
            let mut possible_move: u64 = get_ls1b!(moves); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", queen_idx / 8, queen_idx % 8, move_idx / 8, move_idx % 8);
                pop_bits!(moves, possible_move); // remove current possible move
                possible_move = get_ls1b!(moves);
            }

            pop_bits!(Q, queen); // remove current queen
            queen = get_ls1b!(Q);
        }
        move_list
    }


    fn possibleR(&self, mut R: u64) -> String {
        let mut move_list: String = String::new();
        let mut rook: u64 = get_ls1b!(R);
        while rook != 0 {
            let rook_idx: usize = rook.leading_zeros() as usize;
            let mut moves: u64 = self.possibleHAndVMoves(rook_idx) & self.masks.not_allied_pieces;
            let mut possible_move: u64 = get_ls1b!(moves); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", rook_idx / 8, rook_idx % 8, move_idx / 8, move_idx % 8);
                pop_bits!(moves, possible_move); // remove current possible move
                possible_move = get_ls1b!(moves);
            }

            pop_bits!(R, rook); // remove current rook
            rook = get_ls1b!(R);
        }
        move_list
    }


    fn possibleN(&self, mut N: u64) -> String {
        let mut move_list: String = String::new();
        let mut knight: u64 = get_ls1b!(N);
        let knight_span_c6_idx: usize = 18;
        while knight != 0 {
            let knight_idx: usize = knight.leading_zeros() as usize;

            // allign the knight_span_c6 mask
            let mut moves: u64;
            if knight_idx <= knight_span_c6_idx {
                moves = self.masks.knight_span_c6 << (knight_span_c6_idx - knight_idx);
            } else {
                moves = self.masks.knight_span_c6 >> (knight_idx - knight_span_c6_idx);
            }

            // remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4 {
                pop_bits!(moves, !(!self.masks.file_gh & self.masks.not_allied_pieces));
            } else {
                pop_bits!(moves, !(!self.masks.file_ab & self.masks.not_allied_pieces));
            }
            let mut possible_move: u64 = get_ls1b!(moves); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", knight_idx / 8, knight_idx % 8, move_idx / 8, move_idx % 8);
                pop_bits!(moves, possible_move); // remove current possible move
                possible_move = get_ls1b!(moves);
            }

            pop_bits!(N, knight); // remove current knight
            knight = get_ls1b!(N);
        }
        move_list
    }


    fn possibleK(&self, mut K: u64) -> String {
        let mut move_list: String = String::new();
        let mut king: u64 = get_ls1b!(K);
        let king_span_c7_idx: usize = 10;
        while king != 0 {
            let king_idx: usize = king.leading_zeros() as usize;

            // allign the king_span_c7 mask
            let mut moves: u64;
            if king_idx <= king_span_c7_idx {
                moves = self.masks.king_span_c7 << (king_span_c7_idx - king_idx);
            } else {
                moves = self.masks.king_span_c7 >> (king_idx - king_span_c7_idx);
            }

            // remove moves sliding off board or allied pieces
            if king_idx % 8 < 4 {
                pop_bits!(moves, !(!self.masks.file_gh & self.masks.not_allied_pieces));
            } else {
                pop_bits!(moves, !(!self.masks.file_ab & self.masks.not_allied_pieces));
            }
            let mut possible_move: u64 = get_ls1b!(moves); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", king_idx / 8, king_idx % 8, move_idx / 8, move_idx % 8);
                pop_bits!(moves, possible_move); // remove current possible move
                possible_move = get_ls1b!(moves);
            }

            pop_bits!(K, king); // remove current king
            king = get_ls1b!(K);
        }
        move_list
    }


    fn possibleCastleW(&mut self, bitboards: [u64; 13], castle_rights: [bool; 4]) -> String {
        let unsafe_w: u64 = self.unsafeForWhite(bitboards);
        let mut move_list: String = String::new(); // king move r1c1r2c1
        if unsafe_w & bitboards[Piece::WK] == 0 {
            if castle_rights[CastleRights::CWK] && get_bit!(bitboards[Piece::WR], self.castle_rooks[3]) == 1 {
                if ((self.masks.occupied | unsafe_w) & ((1 << 1) | (1 << 2))) == 0 {
                    move_list += "7476";
                }
            }
            if castle_rights[CastleRights::CWQ] && get_bit!(bitboards[Piece::WR], self.castle_rooks[2]) == 1 {
                if ((self.masks.occupied | (unsafe_w & !(1 << 6))) & ((1 << 4) | (1 << 5) | (1 << 6))) == 0 {
                    move_list += "7472";
                }
            }
        }
        move_list
    }


    fn possibleCastleB(&mut self, bitboards: [u64; 13], castle_rights: [bool; 4]) -> String {
        let unsafe_b = self.unsafeForBlack(bitboards);
        let mut move_list: String = String::new(); // king move r1c1r2c1
        if unsafe_b & bitboards[Piece::BK] == 0 {
            if castle_rights[CastleRights::CBK] && get_bit!(bitboards[Piece::BR], self.castle_rooks[1]) == 1 {
                if ((self.masks.occupied | unsafe_b) & ((1 << 58) | (1 << 57))) == 0 {
                    move_list += "0406";
                }
            }
            if castle_rights[CastleRights::CBQ] && get_bit!(bitboards[Piece::BR], self.castle_rooks[0]) == 1 {
                if ((self.masks.occupied | (unsafe_b & !(1 << 62))) & ((1 << 62) | (1 << 61) | (1 << 60))) == 0 {
                    move_list += "0402";
                }
            }
        }
        move_list
    }


    fn possibleHAndVMoves(&self, piece_idx: usize) -> u64 {
        // piece_idx = 0 -> top left of board -> 1000...000
        let binary_idx: u64 = 1 << (64 - 1 - piece_idx);
        let rank_mask = self.masks.rank_masks[piece_idx / 8];
        let file_mask = self.masks.file_masks[piece_idx % 8];
        let possible_h = (wrap_op!(self.masks.occupied, wrap_op!(binary_idx, 2, '*'), '-')) ^ (wrap_op!(self.masks.occupied.reverse_bits(), wrap_op!(binary_idx.reverse_bits(), 2, '*'), '-')).reverse_bits();
        let possible_v = (wrap_op!((self.masks.occupied & file_mask), wrap_op!(binary_idx, 2, '*'), '-')) ^ (wrap_op!((self.masks.occupied & file_mask).reverse_bits(), wrap_op!(binary_idx.reverse_bits(), 2, '*'), '-')).reverse_bits();
        (possible_h & rank_mask) | (possible_v & file_mask)
    }


    fn possibleDiagAndAntiDiagMoves(&self, piece_idx: usize) -> u64 {
        // piece_idx = 0 -> top left of board -> 1000...000
        let binary_idx: u64 = 1 << (64 - 1 - piece_idx);
        let diag_mask = self.masks.diagonal_masks[(piece_idx / 8) + (piece_idx % 8)];
        let a_diag_mask = self.masks.anti_diagonal_masks[7 + (piece_idx / 8) - (piece_idx % 8)];
        let possible_d = (wrap_op!((self.masks.occupied & diag_mask), wrap_op!(binary_idx, 2, '*'), '-')) ^ (wrap_op!((self.masks.occupied & diag_mask).reverse_bits(), wrap_op!(binary_idx.reverse_bits(), 2, '*'), '-')).reverse_bits();
        let possible_ad = (wrap_op!((self.masks.occupied & a_diag_mask), wrap_op!(binary_idx, 2, '*'), '-')) ^ (wrap_op!((self.masks.occupied & a_diag_mask).reverse_bits(), wrap_op!(binary_idx.reverse_bits(), 2, '*'), '-')).reverse_bits();
        (possible_d & diag_mask) | (possible_ad & a_diag_mask)
    }


    pub fn unsafeForBlack(&mut self, mut bitboards: [u64; 13]) -> u64 {
        self.masks.occupied = or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK], bitboards);
        // pawn threats
        let mut unsafe_b: u64 = (bitboards[Piece::WP] << 7) & !self.masks.file_masks[0]; // pawn right capture
        set_bits!(unsafe_b, (bitboards[Piece::WP] << 9) & !self.masks.file_masks[7]); // pawn left capture

        // knight threat
        let mut knight: u64 = get_ls1b!(bitboards[Piece::WN]);
        let knight_span_c6_idx: usize = 18;
        while knight != 0 {
            let knight_idx: usize = knight.leading_zeros() as usize;
            // allign the knight_span_c6 mask
            let mut moves: u64;
            if knight_idx <= knight_span_c6_idx {
                moves = self.masks.knight_span_c6 << (knight_span_c6_idx - knight_idx);
            } else {
                moves = self.masks.knight_span_c6 >> (knight_idx - knight_span_c6_idx);
            }
            // remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4 {
                pop_bits!(moves, self.masks.file_gh);
            } else {
                pop_bits!(moves, self.masks.file_ab);
            }
            set_bits!(unsafe_b, moves);
            pop_bits!(bitboards[Piece::WN], knight); // remove current knight
            knight = get_ls1b!(bitboards[Piece::WN]);
        }

        // bishop / queen threats (diagonals)
        let mut wQB: u64 = bitboards[Piece::WQ] | bitboards[Piece::WB];
        let mut b_or_q: u64 = get_ls1b!(wQB);
        while b_or_q != 0 {
            let b_or_q_idx: usize = b_or_q.leading_zeros() as usize;
            let moves: u64 = self.possibleDiagAndAntiDiagMoves(b_or_q_idx);
            set_bits!(unsafe_b, moves);
            pop_bits!(wQB, b_or_q); // remove current bishop or queen
            b_or_q = get_ls1b!(wQB);
        }

        // rook / queen threats (hor and vert)
        let mut wQR: u64 = bitboards[Piece::WQ] | bitboards[Piece::WR];
        let mut r_or_q: u64 = get_ls1b!(wQR);
        while r_or_q != 0 {
            let r_or_q_idx: usize = r_or_q.leading_zeros() as usize;
            let moves: u64 = self.possibleHAndVMoves(r_or_q_idx);
            set_bits!(unsafe_b, moves);
            pop_bits!(wQR, r_or_q); // remove current rook or queen
            r_or_q = get_ls1b!(wQR);
        }

        // king threats
        let mut king: u64 = get_ls1b!(bitboards[Piece::WK]);
        let king_span_c7_idx: usize = 10;
        while king != 0 {
            let king_idx: usize = king.leading_zeros() as usize;
            // allign the king_span_c7 mask
            let mut moves: u64;
            if king_idx <= king_span_c7_idx {
                moves = self.masks.king_span_c7 << (king_span_c7_idx - king_idx);
            } else {
                moves = self.masks.king_span_c7 >> (king_idx - king_span_c7_idx);
            }
            // remove moves sliding off board or allied pieces
            if king_idx % 8 < 4 {
                pop_bits!(moves, self.masks.file_gh);
            } else {
                pop_bits!(moves, self.masks.file_ab);
            }
            set_bits!(unsafe_b, moves);
            pop_bits!(bitboards[Piece::WK], king); // remove current king
            king = get_ls1b!(bitboards[Piece::WK]);
        }
        unsafe_b
    }


    pub fn unsafeForWhite(&mut self, mut bitboards: [u64; 13]) -> u64 {
        self.masks.occupied = or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK], bitboards);
        // pawn threats
        let mut unsafe_w: u64 = (bitboards[Piece::BP] >> 7) & !self.masks.file_masks[7]; // pawn right capture
        set_bits!(unsafe_w, (bitboards[Piece::BP] >> 9) & !self.masks.file_masks[0]); // pawn left capture

        // knight threat
        let mut knight: u64 = get_ls1b!(bitboards[Piece::BN]);
        let knight_span_c6_idx: usize = 18;
        while knight != 0 {
            let knight_idx: usize = knight.leading_zeros() as usize;
            // allign the knight_span_c6 mask
            let mut moves: u64;
            if knight_idx <= knight_span_c6_idx {
                moves = self.masks.knight_span_c6 << (knight_span_c6_idx - knight_idx);
            } else {
                moves = self.masks.knight_span_c6 >> (knight_idx - knight_span_c6_idx);
            }
            // remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4 {
                pop_bits!(moves, self.masks.file_gh);
            } else {
                pop_bits!(moves, self.masks.file_ab);
            }
            set_bits!(unsafe_w, moves);
            pop_bits!(bitboards[Piece::BN], knight); // remove current knight
            knight = get_ls1b!(bitboards[Piece::BN]);
        }

        // bishop / queen threats (diagonals)
        let mut bQB: u64 = bitboards[Piece::BQ] | bitboards[Piece::BB];
        let mut b_or_q: u64 = get_ls1b!(bQB);
        while b_or_q != 0 {
            let b_or_q_idx: usize = b_or_q.leading_zeros() as usize;
            let moves: u64 = self.possibleDiagAndAntiDiagMoves(b_or_q_idx);
            set_bits!(unsafe_w, moves);
            pop_bits!(bQB, b_or_q); // remove current bishop or queen
            b_or_q = get_ls1b!(bQB);
        }

        // rook / queen threats (hor and vert)
        let mut bQR: u64 = bitboards[Piece::BQ] | bitboards[Piece::BR];
        let mut r_or_q: u64 = get_ls1b!(bQR);
        while r_or_q != 0 {
            let r_or_q_idx: usize = r_or_q.leading_zeros() as usize;
            let moves: u64 = self.possibleHAndVMoves(r_or_q_idx);
            set_bits!(unsafe_w, moves);
            pop_bits!(bQR, r_or_q); // remove current rook or queen
            r_or_q = get_ls1b!(bQR);
        }

        // king threats
        let mut king = get_ls1b!(bitboards[Piece::BK]);
        let king_span_c7_idx: usize = 10;
        while king != 0 {
            let king_idx: usize = king.leading_zeros() as usize;
            // allign the king_span_c7 mask
            let mut moves: u64;
            if king_idx <= king_span_c7_idx {
                moves = self.masks.king_span_c7 << (king_span_c7_idx - king_idx);
            } else {
                moves = self.masks.king_span_c7 >> (king_idx - king_span_c7_idx);
            }
            // remove moves sliding off board or allied pieces
            if king_idx % 8 < 4 {
                pop_bits!(moves, self.masks.file_gh);
            } else {
                pop_bits!(moves, self.masks.file_ab);
            }
            set_bits!(unsafe_w, moves);
            pop_bits!(bitboards[Piece::BK], king); // remove current king
            king = get_ls1b!(bitboards[Piece::BK]);
        }
        unsafe_w
    }


    pub fn getUpdatedCastleRights(&self, z: &mut Zobrist, move_str: &str, castle_rights: [bool; 4], bitboards: [u64; 13], mut hash_key: u64) -> ([bool; 4], u64) {
        // remove current castle rights from hash
        hash_key ^= z.castle_keys[
            ((castle_rights[CastleRights::CBQ] as usize) << 3)
            | ((castle_rights[CastleRights::CBK] as usize) << 2)
            | ((castle_rights[CastleRights::CWQ] as usize) << 1)
            | (castle_rights[CastleRights::CWK] as usize)
        ];
        let mut castle_rights_t: [bool; 4] = castle_rights;
        if move_str.chars().nth(3).unwrap().is_numeric() {
            let (r1, c1, r2, c2) = move_to_u32s!(move_str);
            let start_sq: u32 = r1 * 8 + c1;
            let end_sq: u32 = r2 * 8 + c2;
            if get_bit!(bitboards[Piece::WK], start_sq) != 0 { // white king move
                (castle_rights_t[CastleRights::CWK], castle_rights_t[CastleRights::CWQ]) = (false, false);
            }
            if get_bit!(bitboards[Piece::BK], start_sq) != 0 { // black king move
                (castle_rights_t[CastleRights::CBK], castle_rights_t[CastleRights::CBQ]) = (false, false);
            }
            if start_sq == self.castle_rooks[3] as u32 && get_bit!(bitboards[Piece::WR], start_sq) == 1 { // white king side rook move
                castle_rights_t[CastleRights::CWK] = false;
            }
            if start_sq == self.castle_rooks[2] as u32 && get_bit!(bitboards[Piece::WR], start_sq) == 1 { // white queen side rook move
                castle_rights_t[CastleRights::CWQ] = false;
            }
            if start_sq == self.castle_rooks[1] as u32 && get_bit!(bitboards[Piece::BR], start_sq) == 1 { // black king side rook move
                castle_rights_t[CastleRights::CBK] = false;
            }
            if start_sq == self.castle_rooks[0] as u32 && get_bit!(bitboards[Piece::BR], start_sq) == 1 { // black queen side rook move
                castle_rights_t[CastleRights::CBQ] = false;
            }
            if end_sq == self.castle_rooks[3] as u32 && get_bit!(bitboards[Piece::WR], end_sq) == 1 { // white king side rook taken
                castle_rights_t[CastleRights::CWK] = false;
            }
            if end_sq == self.castle_rooks[2] as u32 && get_bit!(bitboards[Piece::WR], end_sq) == 1 { // white queen side rook taken
                castle_rights_t[CastleRights::CWQ] = false;
            }
            if end_sq == self.castle_rooks[1] as u32 && get_bit!(bitboards[Piece::BR], end_sq) == 1 { // black king side rook taken
                castle_rights_t[CastleRights::CBK] = false;
            }
            if end_sq == self.castle_rooks[0] as u32 && get_bit!(bitboards[Piece::BR], end_sq) == 1 { // black queen side rook taken
                castle_rights_t[CastleRights::CBQ] = false;
            }
        }
        // add next moves castle rights to hash
        hash_key ^= z.castle_keys[
            ((castle_rights_t[CastleRights::CBQ] as usize) << 3)
            | ((castle_rights_t[CastleRights::CBK] as usize) << 2)
            | ((castle_rights_t[CastleRights::CWQ] as usize) << 1)
            | (castle_rights_t[CastleRights::CWK] as usize)
        ];
        (castle_rights_t, hash_key)
    }


    pub fn getUpdatedBitboards(&self, z: &mut Zobrist, move_str: &str, bitboards: [u64; 13], mut hash_key: u64, whites_turn: bool) -> ([u64; 13], u64) {
        hash_key ^= z.side_key; // hash side
        let mut bitboards_t: [u64; 13] = [0; 13];
        (bitboards_t[Piece::WP], hash_key) = self.makeMove(z, bitboards[Piece::WP], hash_key, move_str, Piece::WP); (bitboards_t[Piece::WN], hash_key) = self.makeMove(z, bitboards[Piece::WN], hash_key, move_str, Piece::WN);
        (bitboards_t[Piece::WB], hash_key) = self.makeMove(z, bitboards[Piece::WB], hash_key, move_str, Piece::WB); (bitboards_t[Piece::WR], hash_key) = self.makeMove(z, bitboards[Piece::WR], hash_key, move_str, Piece::WR);
        (bitboards_t[Piece::WQ], hash_key) = self.makeMove(z, bitboards[Piece::WQ], hash_key, move_str, Piece::WQ); (bitboards_t[Piece::WK], hash_key) = self.makeMove(z, bitboards[Piece::WK], hash_key, move_str, Piece::WK);
        (bitboards_t[Piece::BP], hash_key) = self.makeMove(z, bitboards[Piece::BP], hash_key, move_str, Piece::BP); (bitboards_t[Piece::BN], hash_key) = self.makeMove(z, bitboards[Piece::BN], hash_key, move_str, Piece::BN);
        (bitboards_t[Piece::BB], hash_key) = self.makeMove(z, bitboards[Piece::BB], hash_key, move_str, Piece::BB); (bitboards_t[Piece::BR], hash_key) = self.makeMove(z, bitboards[Piece::BR], hash_key, move_str, Piece::BR);
        (bitboards_t[Piece::BQ], hash_key) = self.makeMove(z, bitboards[Piece::BQ], hash_key, move_str, Piece::BQ); (bitboards_t[Piece::BK], hash_key) = self.makeMove(z, bitboards[Piece::BK], hash_key, move_str, Piece::BK);
        (bitboards_t[Piece::WR], hash_key) = self.makeMoveCastle(z, bitboards_t[Piece::WR], bitboards[Piece::WK], hash_key, move_str, Piece::WR);
        (bitboards_t[Piece::BR], hash_key) = self.makeMoveCastle(z, bitboards_t[Piece::BR], bitboards[Piece::BK], hash_key, move_str, Piece::BR);
        (bitboards_t[Piece::EP], hash_key) = self.makeMoveEP(z, bitboards[Piece::EP], or_array_elems!([Piece::WP, Piece::BP], bitboards), hash_key, move_str, whites_turn);
        (bitboards_t, hash_key)
    }


    pub fn isValidMove(&mut self, bitboards: [u64; 13], whites_turn: bool) -> bool {
        (whites_turn && (bitboards[Piece::WK] & self.unsafeForWhite(bitboards)) == 0)
            || (!whites_turn && (bitboards[Piece::BK] & self.unsafeForBlack(bitboards)) == 0)
    }


    pub fn isKingAttacked(&mut self, bitboards: [u64; 13], whites_turn: bool) -> bool {
        (whites_turn && (bitboards[Piece::WK] & self.unsafeForWhite(bitboards)) != 0)
            || (!whites_turn && (bitboards[Piece::BK] & self.unsafeForBlack(bitboards)) != 0)
    }


    pub fn isAttackingMove(&mut self, bitboards: [u64; 13], bitboards_t: [u64; 13], whites_turn: bool) -> bool {
        self.isValidMove(bitboards_t, whites_turn)
        && (
            (!whites_turn
                && or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ], bitboards).count_ones()
                != or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ], bitboards_t).count_ones()
            )
            || (whites_turn
                && or_array_elems!([Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ], bitboards).count_ones()
                != or_array_elems!([Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ], bitboards_t).count_ones()
            )
        )
    }


    /// Alias so conversion can be done in python as well
    fn moveToAlgebra(&self, move_str: &str) -> String {
        move_to_algebra!(&move_str)
    }


    /// Alias so conversion can be done in python as well
    fn algebraToMove(&self, alg_str: &str) -> String {
        algebra_to_move!(&alg_str)
    }
}
