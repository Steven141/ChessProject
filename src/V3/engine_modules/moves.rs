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
    pub castle_rooks: [usize; 4],
    pub masks: SpecialBitBoards,
    pub checkmate: bool,
    pub stalemate: bool,
}


#[pymethods]
impl Moves {
    #[new]
    pub fn new() -> Self {
        Moves {
            castle_rooks: [63, 56, 7, 0],
            masks: SpecialBitBoards::new(),
            checkmate: false,
            stalemate: false,
        }
    }


    pub fn getValidMoves(&mut self, z: &mut Zobrist, bitboards: [i64; 13], castle_rights: [bool; 4], hash_key: u64, whites_turn: bool, depth: u32) -> String {
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


    pub fn makeMove(&self, z: &mut Zobrist, mut bitboard: i64, mut hash_key: u64, move_str: &str, p_type: Piece) -> (i64, u64) {
        let start_shift: u32; let end_shift: u32;
        let start_bitboard: i64; let end_bitboard: i64;
        if move_str.chars().nth(3).unwrap().is_numeric() { // regular move
            let (r1, c1, r2, c2) = move_to_u32s!(move_str);
            start_shift = 64 - 1 - (r1 * 8 + c1);
            end_shift = 64 - 1 - (r2 * 8 + c2);
            if usgn_r_shift!(bitboard, end_shift) & 1 == 1 {
                hash_key ^= z.piece_keys[p_type][(r2 * 8 + c2) as usize]; // remove taken piece from hash
            }
            if usgn_r_shift!(bitboard, start_shift) & 1 == 1 {
                hash_key ^= z.piece_keys[p_type][(r1 * 8 + c1) as usize]; // remove source piece from hash
                hash_key ^= z.piece_keys[p_type][(r2 * 8 + c2) as usize]; // add target piece to hash
                bitboard &= !(1 << start_shift); // remove moving piece from board
                bitboard |= 1 << end_shift; // add at new position
            } else {
                bitboard &= !(1 << end_shift); // remove piece at end
            }
        } else if move_str.chars().nth(3).unwrap() == 'P' { // pawn promo
            let whites_turn: bool = move_str.chars().nth(2).unwrap().is_uppercase();
            let (c1, c2, _, _) = move_to_u32s!(move_str);
            let (r1, r2) = if whites_turn {(1, 0)} else {(6, 7)};
            if whites_turn { // white promo
                start_bitboard = self.masks.file_masks[c1 as usize] & self.masks.rank_masks[1];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = self.masks.file_masks[c2 as usize] & self.masks.rank_masks[0];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
            } else { // black promo
                start_bitboard = self.masks.file_masks[c1 as usize] & self.masks.rank_masks[6];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = self.masks.file_masks[c2 as usize] & self.masks.rank_masks[7];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
            }
            if usgn_r_shift!(bitboard, end_shift) & 1 == 1 {
                hash_key ^= z.piece_keys[p_type][(r2 * 8 + c2) as usize]; // remove taken piece from hash
            }
            if usgn_r_shift!(bitboard, start_shift) & 1 == 1 {
                hash_key ^= z.piece_keys[p_type][(r1 * 8 + c1) as usize]; // remove source piece from hash
            }
            if p_type == move_str.chars().nth(2).unwrap() {
                hash_key ^= z.piece_keys[p_type][(r2 * 8 + c2) as usize]; // add promoted piece to hash
                bitboard |= 1 << end_shift;
            } else {
                bitboard &= !(1 << start_shift);
                bitboard &= !(1 << end_shift);
            }
        } else if move_str.chars().nth(3).unwrap() == 'E' { // enpassant
            let whites_turn: bool = move_str.chars().nth(2).unwrap() == 'w';
            let (c1, c2, _, _) = move_to_u32s!(move_str);
            let (r1, r2) = if whites_turn {(3, 2)} else {(4, 5)};
            if whites_turn { // white
                start_bitboard = self.masks.file_masks[c1 as usize] & self.masks.rank_masks[3];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = self.masks.file_masks[c2 as usize] & self.masks.rank_masks[2];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
                if usgn_r_shift!(bitboard, end_shift-8) & 1 == 1 {
                    hash_key ^= z.piece_keys[p_type][(r1 * 8 + c2) as usize] // remove taken piece from hash
                }
                bitboard &= !(self.masks.file_masks[c2 as usize] & self.masks.rank_masks[3]);
            } else { // black
                start_bitboard = self.masks.file_masks[c1 as usize] & self.masks.rank_masks[4];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = self.masks.file_masks[c2 as usize] & self.masks.rank_masks[5];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
                if usgn_r_shift!(bitboard, end_shift+8) & 1 == 1 {
                    hash_key ^= z.piece_keys[p_type][(r1 * 8 + c2) as usize] // remove taken piece from hash
                }
                bitboard &= !(self.masks.file_masks[c2 as usize] & self.masks.rank_masks[4]);
            }
            if usgn_r_shift!(bitboard, start_shift) & 1 == 1 {
                hash_key ^= z.piece_keys[p_type][(r1 * 8 + c1) as usize]; // remove source piece from hash
                hash_key ^= z.piece_keys[p_type][(r2 * 8 + c2) as usize]; // add target piece to hash
                bitboard &= !(1 << start_shift);
                bitboard |= 1 << end_shift;
            }
        } else {
            panic!("INVALID MOVE TYPE");
        }
        (bitboard, hash_key)
    }


    pub fn makeMoveCastle(&self, z: &mut Zobrist, mut rook: i64, king: i64, mut hash_key: u64, move_str: &str, p_type: Piece) -> (i64, u64) {
        let (r1, c1, _, _) = move_to_u32s!(move_str);
        let start_shift: u32 = 64 - 1 - (r1 * 8 + c1);
        if (usgn_r_shift!(king, start_shift) & 1 == 1) && ((move_str == "0402") || (move_str == "0406") || (move_str == "7472") || (move_str == "7476")) {
            if p_type == Piece::WR { // white
                match move_str {
                    "7476" => { // king side
                        hash_key ^= z.piece_keys[p_type][63 - self.castle_rooks[3]];
                        hash_key ^= z.piece_keys[p_type][63 - (self.castle_rooks[3] + 2)];
                        rook &= !(1 << self.castle_rooks[3]);
                        rook |= 1 << (self.castle_rooks[3] + 2);
                    },
                    "7472" => { // queen side
                        hash_key ^= z.piece_keys[p_type][63 - self.castle_rooks[2]];
                        hash_key ^= z.piece_keys[p_type][63 - (self.castle_rooks[2] - 3)];
                        rook &= !(1 << self.castle_rooks[2]);
                        rook |= 1 << (self.castle_rooks[2] - 3);
                    },
                    _ => (),
                }
            } else { // black
                match move_str {
                    "0406" => { // king side
                        hash_key ^= z.piece_keys[p_type][63 - self.castle_rooks[1]];
                        hash_key ^= z.piece_keys[p_type][63 - (self.castle_rooks[1] + 2)];
                        rook &= !(1 << self.castle_rooks[1]);
                        rook |= 1 << (self.castle_rooks[1] + 2);
                    },
                    "0402" => { // queen side
                        hash_key ^= z.piece_keys[p_type][63 - self.castle_rooks[0]];
                        hash_key ^= z.piece_keys[p_type][63 - (self.castle_rooks[0] - 3)];
                        rook &= !(1 << self.castle_rooks[0]);
                        rook |= 1 << (self.castle_rooks[0] - 3);
                    },
                    _ => (),
                }
            }
        }
        (rook, hash_key)
    }


    pub fn makeMoveEP(&self, z: &mut Zobrist, ep: i64, bitboard: i64, mut hash_key: u64, move_str: &str, whites_turn: bool) -> (i64, u64) {
        // remove current enpassant status from hash
        if ep != 0 {
            let col: usize = ep.leading_zeros() as usize;
            let row: usize = if whites_turn {2} else {5};
            hash_key ^= z.enpassant_keys[row * 8 + col];
        }
        let mut ep_t: i64 = 0;
        if move_str.chars().nth(3).unwrap().is_numeric() {
            let (r1, c1, r2, _) = move_to_u32s!(move_str);
            let start_shift: u32 = 64 - 1 - (r1 * 8 + c1);
            if (r1 as i64 - r2 as i64).abs() == 2 && (usgn_r_shift!(bitboard, start_shift) & 1) == 1 {
                ep_t = self.masks.file_masks[c1 as usize];
                let col: usize = ep_t.leading_zeros() as usize;
                let row: usize = if !whites_turn {2} else {5};
                hash_key ^= z.enpassant_keys[row * 8 + col]; // add next move enpassant status to hash
            }
        }
        (ep_t, hash_key)
    }


    pub fn getPossibleMoves(&mut self, bitboards: [i64; 13], castle_rights: [bool; 4], whites_turn: bool) -> String {
        if whites_turn {self.possibleMovesW(bitboards, castle_rights)}
        else {self.possibleMovesB(bitboards, castle_rights)}
    }


    pub fn possibleMovesW(&mut self, bitboards: [i64; 13], castle_rights: [bool; 4]) -> String {
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


    pub fn possibleMovesB(&mut self, bitboards: [i64; 13], castle_rights: [bool; 4]) -> String {
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


    fn possibleWP(&self, wP: i64, bP: i64, EP: i64) -> String {
        // standard moves and captures
        let mut move_list: String = String::new(); // r1,c1,r2,c2
        let mut moves: i64 = (wP << 7) & self.masks.enemy_pieces & !self.masks.rank_masks[0] & !self.masks.file_masks[0]; // right capture
        let mut possible_move: i64 = moves & !wrap_op!(moves, 1, '-'); // selects single possible move
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) + 1, (idx % 8) - 1, idx / 8, idx % 8);
            moves &= !possible_move; // remove current move from moves
            possible_move = moves & !wrap_op!(moves, 1, '-'); // get next possible move
        }

        moves = (wP << 9) & self.masks.enemy_pieces & !self.masks.rank_masks[0] & !self.masks.file_masks[7]; // left capture
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) + 1, (idx % 8) + 1, idx / 8, idx % 8);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = (wP << 8) & self.masks.empty & !self.masks.rank_masks[0]; // move forward 1
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) + 1, idx % 8, idx / 8, idx % 8);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = (wP << 16) & self.masks.empty & (self.masks.empty << 8) & self.masks.rank_masks[4]; // move forward 2
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) + 2, idx % 8, idx / 8, idx % 8);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        // pawn promotion, move_list -> c1,c2,promo type,'P'
        moves = (wP << 7) & self.masks.enemy_pieces & self.masks.rank_masks[0] & !self.masks.file_masks[0]; // promo by right capture
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) - 1; let c2 = idx % 8;
            move_list += &format!("{}{}QP{}{}RP{}{}BP{}{}NP", c1, c2, c1, c2, c1, c2, c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = (wP << 9) & self.masks.enemy_pieces & self.masks.rank_masks[0] & !self.masks.file_masks[7]; // promo by left capture
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) + 1; let c2 = idx % 8;
            move_list += &format!("{}{}QP{}{}RP{}{}BP{}{}NP", c1, c2, c1, c2, c1, c2, c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = (wP << 8) & self.masks.empty & self.masks.rank_masks[0]; // promo by move forward 1
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = idx % 8; let c2 = idx % 8;
            move_list += &format!("{}{}QP{}{}RP{}{}BP{}{}NP", c1, c2, c1, c2, c1, c2, c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        // enpassant, move_list -> c1,c2,'wE'
        moves = usgn_r_shift!(wP, 1) & bP & self.masks.rank_masks[3] & !self.masks.file_masks[0] & EP; // enpassant right
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) - 1; let c2 = idx % 8;
            move_list += &format!("{}{}wE", c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = (wP << 1) & bP & self.masks.rank_masks[3] & !self.masks.file_masks[7] & EP; // enpassant left
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) + 1; let c2 = idx % 8;
            move_list += &format!("{}{}wE", c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }
        move_list
    }


    fn possibleBP(&self, wP: i64, bP: i64, EP: i64) -> String {
        // standard moves and captures
        let mut move_list: String = String::new(); // r1,c1,r2,c2
        let mut moves: i64 = usgn_r_shift!(bP, 7) & self.masks.enemy_pieces & !self.masks.rank_masks[7] & !self.masks.file_masks[7]; // right capture
        let mut possible_move: i64 = moves & !wrap_op!(moves, 1, '-'); // selects single possible move
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) - 1, (idx % 8) + 1, idx / 8, idx % 8);
            moves &= !possible_move; // remove current move from moves
            possible_move = moves & !wrap_op!(moves, 1, '-'); // get next possible move
        }

        moves = usgn_r_shift!(bP, 9) & self.masks.enemy_pieces & !self.masks.rank_masks[7] & !self.masks.file_masks[0]; // left capture
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) - 1, (idx % 8) - 1, idx / 8, idx % 8);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = usgn_r_shift!(bP, 8) & self.masks.empty & !self.masks.rank_masks[7]; // move forward 1
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) - 1, idx % 8, idx / 8, idx % 8);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = usgn_r_shift!(bP, 16) & self.masks.empty & usgn_r_shift!(self.masks.empty, 8) & self.masks.rank_masks[3]; // move forward 2
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            move_list += &format!("{}{}{}{}", (idx / 8) - 2, idx % 8, idx / 8, idx % 8);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        // pawn promotion, move_list -> c1,c2,promo type,'P'
        moves = usgn_r_shift!(bP, 7) & self.masks.enemy_pieces & self.masks.rank_masks[7] & !self.masks.file_masks[7]; // promo by right capture
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) + 1; let c2 = idx % 8;
            move_list += &format!("{}{}qP{}{}rP{}{}bP{}{}nP", c1, c2, c1, c2, c1, c2, c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = usgn_r_shift!(bP, 9) & self.masks.enemy_pieces & self.masks.rank_masks[7] & !self.masks.file_masks[0]; // promo by left capture
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) - 1; let c2 = idx % 8;
            move_list += &format!("{}{}qP{}{}rP{}{}bP{}{}nP", c1, c2, c1, c2, c1, c2, c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = usgn_r_shift!(bP, 8) & self.masks.empty & self.masks.rank_masks[7]; // promo by move forward 1
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = idx % 8; let c2 = idx % 8;
            move_list += &format!("{}{}qP{}{}rP{}{}bP{}{}nP", c1, c2, c1, c2, c1, c2, c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        // enpassant, move_list -> c1,c2,'wE'
        moves = (bP << 1) & wP & self.masks.rank_masks[4] & !self.masks.file_masks[7] & EP; // enpassant right
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) + 1; let c2 = idx % 8;
            move_list += &format!("{}{}bE", c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }

        moves = usgn_r_shift!(bP, 1) & wP & self.masks.rank_masks[4] & !self.masks.file_masks[0] & EP; // enpassant left
        possible_move = moves & !wrap_op!(moves, 1, '-');
        while possible_move != 0 {
            let idx: u32 = possible_move.leading_zeros();
            let c1 = (idx % 8) - 1; let c2 = idx % 8;
            move_list += &format!("{}{}bE", c1, c2);
            moves &= !possible_move;
            possible_move = moves & !wrap_op!(moves, 1, '-');
        }
        move_list
    }


    fn possibleB(&self, mut B: i64) -> String {
        let mut move_list: String = String::new();
        let mut bishop: i64 = B & !wrap_op!(B, 1, '-');
        while bishop != 0 {
            let bishop_idx: usize = bishop.leading_zeros() as usize;
            let mut moves: i64 = self.possibleDiagAndAntiDiagMoves(bishop_idx) & self.masks.not_allied_pieces;
            let mut possible_move: i64 = moves & !wrap_op!(moves, 1, '-'); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", bishop_idx / 8, bishop_idx % 8, move_idx / 8, move_idx % 8);
                moves &= !possible_move; // remove current possible move
                possible_move = moves & !wrap_op!(moves, 1, '-');
            }

            B &= !bishop; // remove current bishop
            bishop = B & !wrap_op!(B, 1, '-');
        }
        move_list
    }


    fn possibleQ(&self, mut Q: i64) -> String {
        let mut move_list: String = String::new();
        let mut queen: i64 = Q & !wrap_op!(Q, 1, '-');
        while queen != 0 {
            let queen_idx: usize = queen.leading_zeros() as usize;
            let mut moves: i64 = (self.possibleDiagAndAntiDiagMoves(queen_idx) | self.possibleHAndVMoves(queen_idx)) & self.masks.not_allied_pieces;
            let mut possible_move: i64 = moves & !wrap_op!(moves, 1, '-'); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", queen_idx / 8, queen_idx % 8, move_idx / 8, move_idx % 8);
                moves &= !possible_move; // remove current possible move
                possible_move = moves & !wrap_op!(moves, 1, '-');
            }

            Q &= !queen; // remove current queen
            queen = Q & !wrap_op!(Q, 1, '-');
        }
        move_list
    }


    fn possibleR(&self, mut R: i64) -> String {
        let mut move_list: String = String::new();
        let mut rook: i64 = R & !wrap_op!(R, 1, '-');
        while rook != 0 {
            let rook_idx: usize = rook.leading_zeros() as usize;
            let mut moves: i64 = self.possibleHAndVMoves(rook_idx) & self.masks.not_allied_pieces;
            let mut possible_move: i64 = moves & !wrap_op!(moves, 1, '-'); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", rook_idx / 8, rook_idx % 8, move_idx / 8, move_idx % 8);
                moves &= !possible_move; // remove current possible move
                possible_move = moves & !wrap_op!(moves, 1, '-');
            }

            R &= !rook; // remove current rook
            rook = R & !wrap_op!(R, 1, '-');
        }
        move_list
    }


    fn possibleN(&self, mut N: i64) -> String {
        let mut move_list: String = String::new();
        let mut knight: i64 = N & !wrap_op!(N, 1, '-');
        let knight_span_c6_idx: usize = 18;
        while knight != 0 {
            let knight_idx: usize = knight.leading_zeros() as usize;

            // allign the knight_span_c6 mask
            let mut moves: i64;
            if knight_idx <= knight_span_c6_idx {
                moves = self.masks.knight_span_c6 << (knight_span_c6_idx - knight_idx);
            } else {
                moves = usgn_r_shift!(self.masks.knight_span_c6, knight_idx - knight_span_c6_idx);
            }

            // remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4 {
                moves &= !self.masks.file_gh & self.masks.not_allied_pieces;
            } else {
                moves &= !self.masks.file_ab & self.masks.not_allied_pieces;
            }
            let mut possible_move: i64 = moves & !wrap_op!(moves, 1, '-'); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", knight_idx / 8, knight_idx % 8, move_idx / 8, move_idx % 8);
                moves &= !possible_move; // remove current possible move
                possible_move = moves & !wrap_op!(moves, 1, '-');
            }

            N &= !knight; // remove current knight
            knight = N & !wrap_op!(N, 1, '-');
        }
        move_list
    }


    fn possibleK(&self, mut K: i64) -> String {
        let mut move_list: String = String::new();
        let mut king: i64 = K & !wrap_op!(K, 1, '-');
        let king_span_c7_idx: usize = 10;
        while king != 0 {
            let king_idx: usize = king.leading_zeros() as usize;

            // allign the king_span_c7 mask
            let mut moves: i64;
            if king_idx <= king_span_c7_idx {
                moves = self.masks.king_span_c7 << (king_span_c7_idx - king_idx);
            } else {
                moves = usgn_r_shift!(self.masks.king_span_c7, king_idx - king_span_c7_idx);
            }

            // remove moves sliding off board or allied pieces
            if king_idx % 8 < 4 {
                moves &= !self.masks.file_gh & self.masks.not_allied_pieces;
            } else {
                moves &= !self.masks.file_ab & self.masks.not_allied_pieces;
            }
            let mut possible_move: i64 = moves & !wrap_op!(moves, 1, '-'); // selects single possible move

            while possible_move != 0 {
                let move_idx: usize = possible_move.leading_zeros() as usize;
                move_list += &format!("{}{}{}{}", king_idx / 8, king_idx % 8, move_idx / 8, move_idx % 8);
                moves &= !possible_move; // remove current possible move
                possible_move = moves & !wrap_op!(moves, 1, '-');
            }

            K &= !king; // remove current king
            king = K & !wrap_op!(K, 1, '-');
        }
        move_list
    }


    fn possibleCastleW(&mut self, bitboards: [i64; 13], castle_rights: [bool; 4]) -> String {
        let unsafe_w: i64 = self.unsafeForWhite(bitboards);
        let mut move_list: String = String::new(); // king move r1c1r2c1
        if unsafe_w & bitboards[Piece::WK] == 0 {
            if castle_rights[CastleRights::CWK] && (((1 << self.castle_rooks[3]) & bitboards[Piece::WR]) != 0) {
                if ((self.masks.occupied | unsafe_w) & ((1 << 1) | (1 << 2))) == 0 {
                    move_list += "7476";
                }
            }
            if castle_rights[CastleRights::CWQ] && (((1 << self.castle_rooks[2]) & bitboards[Piece::WR]) != 0) {
                if ((self.masks.occupied | (unsafe_w & !(1 << 6))) & ((1 << 4) | (1 << 5) | (1 << 6))) == 0 {
                    move_list += "7472";
                }
            }
        }
        move_list
    }


    fn possibleCastleB(&mut self, bitboards: [i64; 13], castle_rights: [bool; 4]) -> String {
        let unsafe_b = self.unsafeForBlack(bitboards);
        let mut move_list: String = String::new(); // king move r1c1r2c1
        if unsafe_b & bitboards[Piece::BK] == 0 {
            if castle_rights[CastleRights::CBK] && (((1 << self.castle_rooks[1]) & bitboards[Piece::BR]) != 0) {
                if ((self.masks.occupied | unsafe_b) & ((1 << 58) | (1 << 57))) == 0 {
                    move_list += "0406";
                }
            }
            if castle_rights[CastleRights::CBQ] && (((1 << self.castle_rooks[0]) & bitboards[Piece::BR]) != 0) {
                if ((self.masks.occupied | (unsafe_b & !(1 << 62))) & ((1 << 62) | (1 << 61) | (1 << 60))) == 0 {
                    move_list += "0402";
                }
            }
        }
        move_list
    }


    fn possibleHAndVMoves(&self, piece_idx: usize) -> i64 {
        // piece_idx = 0 -> top left of board -> 1000...000
        let binary_idx: i64 = 1 << (64 - 1 - piece_idx);
        let rank_mask = self.masks.rank_masks[piece_idx / 8];
        let file_mask = self.masks.file_masks[piece_idx % 8];
        let possible_h = (wrap_op!(self.masks.occupied, wrap_op!(binary_idx, 2, '*'), '-')) ^ (wrap_op!(self.masks.occupied.reverse_bits(), wrap_op!(binary_idx.reverse_bits(), 2, '*'), '-')).reverse_bits();
        let possible_v = (wrap_op!((self.masks.occupied & file_mask), wrap_op!(binary_idx, 2, '*'), '-')) ^ (wrap_op!((self.masks.occupied & file_mask).reverse_bits(), wrap_op!(binary_idx.reverse_bits(), 2, '*'), '-')).reverse_bits();
        (possible_h & rank_mask) | (possible_v & file_mask)
    }


    fn possibleDiagAndAntiDiagMoves(&self, piece_idx: usize) -> i64 {
        // piece_idx = 0 -> top left of board -> 1000...000
        let binary_idx: i64 = 1 << (64 - 1 - piece_idx);
        let diag_mask = self.masks.diagonal_masks[(piece_idx / 8) + (piece_idx % 8)];
        let a_diag_mask = self.masks.anti_diagonal_masks[7 + (piece_idx / 8) - (piece_idx % 8)];
        let possible_d = (wrap_op!((self.masks.occupied & diag_mask), wrap_op!(binary_idx, 2, '*'), '-')) ^ (wrap_op!((self.masks.occupied & diag_mask).reverse_bits(), wrap_op!(binary_idx.reverse_bits(), 2, '*'), '-')).reverse_bits();
        let possible_ad = (wrap_op!((self.masks.occupied & a_diag_mask), wrap_op!(binary_idx, 2, '*'), '-')) ^ (wrap_op!((self.masks.occupied & a_diag_mask).reverse_bits(), wrap_op!(binary_idx.reverse_bits(), 2, '*'), '-')).reverse_bits();
        (possible_d & diag_mask) | (possible_ad & a_diag_mask)
    }


    pub fn unsafeForBlack(&mut self, mut bitboards: [i64; 13]) -> i64 {
        self.masks.occupied = or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK], bitboards);
        // pawn threats
        let mut unsafe_b: i64 = (bitboards[Piece::WP] << 7) & !self.masks.file_masks[0]; // pawn right capture
        unsafe_b |= (bitboards[Piece::WP] << 9) & !self.masks.file_masks[7]; // pawn left capture

        // knight threat
        let mut knight: i64 = bitboards[Piece::WN] & !wrap_op!(bitboards[Piece::WN], 1, '-');
        let knight_span_c6_idx: usize = 18;
        while knight != 0 {
            let knight_idx: usize = knight.leading_zeros() as usize;
            // allign the knight_span_c6 mask
            let mut moves: i64;
            if knight_idx <= knight_span_c6_idx {
                moves = self.masks.knight_span_c6 << (knight_span_c6_idx - knight_idx);
            } else {
                moves = usgn_r_shift!(self.masks.knight_span_c6, knight_idx - knight_span_c6_idx);
            }
            // remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4 {
                moves &= !self.masks.file_gh;
            } else {
                moves &= !self.masks.file_ab;
            }
            unsafe_b |= moves;
            bitboards[Piece::WN] &= !knight; // remove current knight
            knight = bitboards[Piece::WN] & !wrap_op!(bitboards[Piece::WN], 1, '-');
        }

        // bishop / queen threats (diagonals)
        let mut wQB: i64 = bitboards[Piece::WQ] | bitboards[Piece::WB];
        let mut b_or_q: i64 = wQB & !wrap_op!(wQB, 1, '-');
        while b_or_q != 0 {
            let b_or_q_idx: usize = b_or_q.leading_zeros() as usize;
            let moves: i64 = self.possibleDiagAndAntiDiagMoves(b_or_q_idx);
            unsafe_b |= moves;
            wQB &= !b_or_q; // remove current bishop or queen
            b_or_q = wQB & !wrap_op!(wQB, 1, '-');
        }

        // rook / queen threats (hor and vert)
        let mut wQR: i64 = bitboards[Piece::WQ] | bitboards[Piece::WR];
        let mut r_or_q: i64 = wQR & !wrap_op!(wQR, 1, '-');
        while r_or_q != 0 {
            let r_or_q_idx: usize = r_or_q.leading_zeros() as usize;
            let moves: i64 = self.possibleHAndVMoves(r_or_q_idx);
            unsafe_b |= moves;
            wQR &= !r_or_q; // remove current rook or queen
            r_or_q = wQR & !wrap_op!(wQR, 1, '-');
        }

        // king threats
        let mut king: i64 = bitboards[Piece::WK] & !wrap_op!(bitboards[Piece::WK], 1, '-');
        let king_span_c7_idx: usize = 10;
        while king != 0 {
            let king_idx: usize = king.leading_zeros() as usize;
            // allign the king_span_c7 mask
            let mut moves: i64;
            if king_idx <= king_span_c7_idx {
                moves = self.masks.king_span_c7 << (king_span_c7_idx - king_idx);
            } else {
                moves = usgn_r_shift!(self.masks.king_span_c7, king_idx - king_span_c7_idx);
            }
            // remove moves sliding off board or allied pieces
            if king_idx % 8 < 4 {
                moves &= !self.masks.file_gh;
            } else {
                moves &= !self.masks.file_ab;
            }
            unsafe_b |= moves;
            bitboards[Piece::WK] &= !king; // remove current king
            king = bitboards[Piece::WK] & !wrap_op!(bitboards[Piece::WK], 1, '-');
        }
        unsafe_b
    }


    pub fn unsafeForWhite(&mut self, mut bitboards: [i64; 13]) -> i64 {
        self.masks.occupied = or_array_elems!([Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK], bitboards);
        // pawn threats
        let mut unsafe_w: i64 = usgn_r_shift!(bitboards[Piece::BP], 7) & !self.masks.file_masks[7]; // pawn right capture
        unsafe_w |= usgn_r_shift!(bitboards[Piece::BP], 9) & !self.masks.file_masks[0]; // pawn left capture

        // knight threat
        let mut knight: i64 = bitboards[Piece::BN] & !wrap_op!(bitboards[Piece::BN], 1, '-');
        let knight_span_c6_idx: usize = 18;
        while knight != 0 {
            let knight_idx: usize = knight.leading_zeros() as usize;
            // allign the knight_span_c6 mask
            let mut moves: i64;
            if knight_idx <= knight_span_c6_idx {
                moves = self.masks.knight_span_c6 << (knight_span_c6_idx - knight_idx);
            } else {
                moves = usgn_r_shift!(self.masks.knight_span_c6, knight_idx - knight_span_c6_idx);
            }
            // remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4 {
                moves &= !self.masks.file_gh;
            } else {
                moves &= !self.masks.file_ab;
            }
            unsafe_w |= moves;
            bitboards[Piece::BN] &= !knight; // remove current knight
            knight = bitboards[Piece::BN] & !wrap_op!(bitboards[Piece::BN], 1, '-');
        }

        // bishop / queen threats (diagonals)
        let mut bQB: i64 = bitboards[Piece::BQ] | bitboards[Piece::BB];
        let mut b_or_q: i64 = bQB & !wrap_op!(bQB, 1, '-');
        while b_or_q != 0 {
            let b_or_q_idx: usize = b_or_q.leading_zeros() as usize;
            let moves: i64 = self.possibleDiagAndAntiDiagMoves(b_or_q_idx);
            unsafe_w |= moves;
            bQB &= !b_or_q; // remove current bishop or queen
            b_or_q = bQB & !wrap_op!(bQB, 1, '-');
        }

        // rook / queen threats (hor and vert)
        let mut bQR: i64 = bitboards[Piece::BQ] | bitboards[Piece::BR];
        let mut r_or_q: i64 = bQR & !wrap_op!(bQR, 1, '-');
        while r_or_q != 0 {
            let r_or_q_idx: usize = r_or_q.leading_zeros() as usize;
            let moves: i64 = self.possibleHAndVMoves(r_or_q_idx);
            unsafe_w |= moves;
            bQR &= !r_or_q; // remove current rook or queen
            r_or_q = bQR & !wrap_op!(bQR, 1, '-');
        }

        // king threats
        let mut king = bitboards[Piece::BK] & !wrap_op!(bitboards[Piece::BK], 1, '-');
        let king_span_c7_idx: usize = 10;
        while king != 0 {
            let king_idx: usize = king.leading_zeros() as usize;
            // allign the king_span_c7 mask
            let mut moves: i64;
            if king_idx <= king_span_c7_idx {
                moves = self.masks.king_span_c7 << (king_span_c7_idx - king_idx);
            } else {
                moves = usgn_r_shift!(self.masks.king_span_c7, king_idx - king_span_c7_idx);
            }
            // remove moves sliding off board or allied pieces
            if king_idx % 8 < 4 {
                moves &= !self.masks.file_gh;
            } else {
                moves &= !self.masks.file_ab;
            }
            unsafe_w |= moves;
            bitboards[Piece::BK] &= !king; // remove current king
            king = bitboards[Piece::BK] & !wrap_op!(bitboards[Piece::BK], 1, '-');
        }
        unsafe_w
    }


    pub fn getUpdatedCastleRights(&self, z: &mut Zobrist, move_str: &str, castle_rights: [bool; 4], bitboards: [i64; 13], mut hash_key: u64) -> ([bool; 4], u64) {
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
            let start_shift: u32 = 64 - 1 - (r1 * 8 + c1);
            let end_shift: u32 = 64 - 1 - (r2 * 8 + c2);
            if ((1i64 << start_shift) & bitboards[Piece::WK]) != 0 { // white king move
                (castle_rights_t[CastleRights::CWK], castle_rights_t[CastleRights::CWQ]) = (false, false);
            }
            if ((1i64 << start_shift) & bitboards[Piece::BK]) != 0 { // black king move
                (castle_rights_t[CastleRights::CBK], castle_rights_t[CastleRights::CBQ]) = (false, false);
            }
            if ((1i64 << start_shift) & bitboards[Piece::WR] & 1) != 0 { // white king side rook move
                castle_rights_t[CastleRights::CWK] = false;
            }
            if ((1i64 << start_shift) & bitboards[Piece::WR] & (1 << 7)) != 0 { // white queen side rook move
                castle_rights_t[CastleRights::CWQ] = false;
            }
            if ((1i64 << start_shift) & bitboards[Piece::BR] & (1 << 56)) != 0 { // black king side rook move
                castle_rights_t[CastleRights::CBK] = false;
            }
            if ((1i64 << start_shift) & bitboards[Piece::BR] & (1 << 63)) != 0 { // black queen side rook move
                castle_rights_t[CastleRights::CBQ] = false;
            }
            if ((1i64 << end_shift) & 1) != 0 { // white king side rook taken
                castle_rights_t[CastleRights::CWK] = false;
            }
            if ((1i64 << end_shift) & (1 << 7)) != 0 { // white queen side rook taken
                castle_rights_t[CastleRights::CWQ] = false;
            }
            if ((1i64 << end_shift) & ((1 as i64) << 56)) != 0 { // black king side rook taken
                castle_rights_t[CastleRights::CBK] = false;
            }
            if ((1i64 << end_shift) & ((1 as i64) << 63)) != 0 { // black queen side rook taken
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


    pub fn getUpdatedBitboards(&self, z: &mut Zobrist, move_str: &str, bitboards: [i64; 13], mut hash_key: u64, whites_turn: bool) -> ([i64; 13], u64) {
        hash_key ^= z.side_key; // hash side
        let mut bitboards_t: [i64; 13] = [0; 13];
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


    pub fn isValidMove(&mut self, bitboards: [i64; 13], whites_turn: bool) -> bool {
        (whites_turn && (bitboards[Piece::WK] & self.unsafeForWhite(bitboards)) == 0)
            || (!whites_turn && (bitboards[Piece::BK] & self.unsafeForBlack(bitboards)) == 0)
    }


    pub fn isKingAttacked(&mut self, bitboards: [i64; 13], whites_turn: bool) -> bool {
        (whites_turn && (bitboards[Piece::WK] & self.unsafeForWhite(bitboards)) != 0)
            || (!whites_turn && (bitboards[Piece::BK] & self.unsafeForBlack(bitboards)) != 0)
    }


    pub fn isAttackingMove(&mut self, bitboards: [i64; 13], bitboards_t: [i64; 13], whites_turn: bool) -> bool {
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
}
