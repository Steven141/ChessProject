//! Module holding information about all the moves


use pyo3::prelude::*;
use crate::castle_rights::CastleRights;
use crate::special_bitboards::SpecialBitBoards;
use crate::piece::Piece;
use std::str::from_utf8;
use rand::thread_rng;
use rand::seq::SliceRandom;


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct Moves {
    castle_rooks: [usize; 4],
    masks: SpecialBitBoards,
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


    pub fn getValidMoves(&mut self, bitboards: [i64; 13], castle_rights: [bool; 4], whites_turn: bool, depth: u32) -> String {
        let mut moves: String;
        if whites_turn {
            moves = self.possibleMovesW(bitboards, castle_rights);
        } else {
            moves = self.possibleMovesB(bitboards, castle_rights);
        }
        if depth == 0 {
            // TODO: look to replace shuffling with sorting
            let mut move_groups: Vec<&str> = moves.as_bytes().chunks(4).map(|chunk| from_utf8(chunk).unwrap()).collect();
            move_groups.shuffle(&mut thread_rng());
            moves = move_groups.join("");
        }
        let mut valid_moves: String = String::new();
        for i in (0..moves.len()).step_by(4) {
            let mut bitboards_t: [i64; 13] = [0; 13];
            bitboards_t[Piece::WP] = self.makeMove(bitboards[Piece::WP], moves[i..i+4].to_string(), 'P'); bitboards_t[Piece::WN] = self.makeMove(bitboards[Piece::WN], moves[i..i+4].to_string(), 'N');
            bitboards_t[Piece::WB] = self.makeMove(bitboards[Piece::WB], moves[i..i+4].to_string(), 'B'); bitboards_t[Piece::WR] = self.makeMove(bitboards[Piece::WR], moves[i..i+4].to_string(), 'R');
            bitboards_t[Piece::WQ] = self.makeMove(bitboards[Piece::WQ], moves[i..i+4].to_string(), 'Q'); bitboards_t[Piece::WK] = self.makeMove(bitboards[Piece::WK], moves[i..i+4].to_string(), 'K');
            bitboards_t[Piece::BP] = self.makeMove(bitboards[Piece::BP], moves[i..i+4].to_string(), 'p'); bitboards_t[Piece::BN] = self.makeMove(bitboards[Piece::BN], moves[i..i+4].to_string(), 'n');
            bitboards_t[Piece::BB] = self.makeMove(bitboards[Piece::BB], moves[i..i+4].to_string(), 'b'); bitboards_t[Piece::BR] = self.makeMove(bitboards[Piece::BR], moves[i..i+4].to_string(), 'r');
            bitboards_t[Piece::BQ] = self.makeMove(bitboards[Piece::BQ], moves[i..i+4].to_string(), 'q'); bitboards_t[Piece::BK] = self.makeMove(bitboards[Piece::BK], moves[i..i+4].to_string(), 'k');
            bitboards_t[Piece::WR] = self.makeMoveCastle(bitboards_t[Piece::WR], bitboards[Piece::WK], moves[i..i+4].to_string(), 'R'); bitboards_t[Piece::BR] = self.makeMoveCastle(bitboards_t[Piece::BR], bitboards[Piece::BK], moves[i..i+4].to_string(), 'r');

            let is_valid_move: bool = ((bitboards_t[Piece::WK] & self.unsafeForWhite(bitboards_t)) == 0 && whites_turn) || ((bitboards_t[Piece::BK] & self.unsafeForBlack(bitboards_t)) == 0 && !whites_turn);
            if is_valid_move {
                valid_moves += &moves[i..i+4];
            }
        }
        if valid_moves.len() == 0 {
            if ((bitboards[Piece::WK] & self.unsafeForWhite(bitboards)) != 0 && whites_turn) || ((bitboards[Piece::BK] & self.unsafeForBlack(bitboards)) != 0 && !whites_turn) {
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


    pub fn makeMove(&self, mut bitboard: i64, move_str: String, p_type: char) -> i64 {
        let m1: u32 = move_str.chars().nth(0).unwrap().to_digit(10).unwrap_or(0);
        let m2: u32 = move_str.chars().nth(1).unwrap().to_digit(10).unwrap_or(0);
        let m3: u32 = move_str.chars().nth(2).unwrap().to_digit(10).unwrap_or(0);
        let m4: u32 = move_str.chars().nth(3).unwrap().to_digit(10).unwrap_or(0);
        let start_shift: u32;
        let end_shift: u32;
        let start_bitboard: i64;
        let end_bitboard: i64;
        if move_str.chars().nth(3).unwrap().is_numeric() { // regular move
            start_shift = 64 - 1 - (m1 * 8 + m2);
            end_shift = 64 - 1 - (m3 * 8 + m4);
            if usgn_r_shift!(bitboard, start_shift) & 1 == 1 {
                bitboard &= !(1 << start_shift); // remove moving piece from board
                bitboard |= 1 << end_shift; // add at new position
            } else {
                bitboard &= !(1 << end_shift); // remove piece at end
            }
        } else if move_str.chars().nth(3).unwrap() == 'P' { // pawn promo
            if move_str.chars().nth(2).unwrap().is_uppercase() { // white promo
                start_bitboard = self.masks.file_masks[m1 as usize] & self.masks.rank_masks[1];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = self.masks.file_masks[m2 as usize] & self.masks.rank_masks[0];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
            } else { // black promo
                start_bitboard = self.masks.file_masks[m1 as usize] & self.masks.rank_masks[6];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = self.masks.file_masks[m2 as usize] & self.masks.rank_masks[7];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
            }
            if p_type == move_str.chars().nth(2).unwrap() {
                bitboard |= 1 << end_shift;
            } else {
                bitboard &= !(1 << start_shift);
                bitboard &= !(1 << end_shift);
            }
        } else if move_str.chars().nth(3).unwrap() == 'E' { // enpassant
            if move_str.chars().nth(2).unwrap() == 'w' { // white
                start_bitboard = self.masks.file_masks[m1 as usize] & self.masks.rank_masks[3];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = self.masks.file_masks[m2 as usize] & self.masks.rank_masks[2];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
                bitboard &= !(self.masks.file_masks[m2 as usize] & self.masks.rank_masks[3]);
            } else { // black
                start_bitboard = self.masks.file_masks[m1 as usize] & self.masks.rank_masks[4];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = self.masks.file_masks[m2 as usize] & self.masks.rank_masks[5];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
                bitboard &= !(self.masks.file_masks[m2 as usize] & self.masks.rank_masks[4]);
            }
            if (bitboard >> start_shift) & 1 == 1 {
                bitboard &= !(1 << start_shift);
                bitboard |= 1 << end_shift;
            }
        } else {
            panic!("INVALID MOVE TYPE");
        }
        bitboard
    }


    pub fn makeMoveCastle(&self, mut rook: i64, king: i64, move_str: String, p_type: char) -> i64 {
        let r1: usize = move_str.chars().nth(0).unwrap().to_digit(10).unwrap_or(0) as usize;
        let c1: usize = move_str.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as usize;
        let start_shift: usize = 64 - 1 - (r1 * 8 + c1);
        if (usgn_r_shift!(king, start_shift) & 1 == 1) && ((move_str == "0402") || (move_str == "0406") || (move_str == "7472") || (move_str == "7476")) {
            if p_type == 'R' { // white
                match move_str.as_str() {
                    "7476" => { // king side
                        rook &= !(1 << self.castle_rooks[3]);
                        rook |= 1 << (self.castle_rooks[3] + 2);
                    },
                    "7472" => { // queen side
                        rook &= !(1 << self.castle_rooks[2]);
                        rook |= 1 << (self.castle_rooks[2] - 3);
                    },
                    _ => (),
                }
            } else { // black
                match move_str.as_str() {
                    "0406" => { // king side
                        rook &= !(1 << self.castle_rooks[1]);
                        rook |= 1 << (self.castle_rooks[1] + 2);
                    },
                    "0402" => { // queen side
                        rook &= !(1 << self.castle_rooks[0]);
                        rook |= 1 << (self.castle_rooks[0] - 3);
                    },
                    _ => (),
                }
            }
        }
        rook
    }


    pub fn makeMoveEP(&self, bitboard: i64, move_str: String) -> i64 {
        let r1: usize = move_str.chars().nth(0).unwrap().to_digit(10).unwrap_or(0) as usize;
        let c1: usize = move_str.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as usize;
        let r2: usize = move_str.chars().nth(2).unwrap().to_digit(10).unwrap_or(0) as usize;
        let start_shift: usize = 64 - 1 - (r1 * 8 + c1);
        if move_str.chars().nth(3).unwrap().is_numeric() && ((r1 as i64 - r2 as i64).abs() == 2) && ((usgn_r_shift!(bitboard, start_shift) & 1) == 1) {
            self.masks.file_masks[c1]
        } else {
            0
        }
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


    pub fn getNewCastleRights(&self, move_str: &str, castle_rights: [bool; 4], bitboards: [i64; 13]) -> [bool; 4] {
        let mut castle_rights_t: [bool; 4] = castle_rights;
        if move_str.chars().nth(3).unwrap().is_numeric() {
            let m1: u32 = move_str.chars().nth(0).unwrap().to_digit(10).unwrap();
            let m2: u32 = move_str.chars().nth(1).unwrap().to_digit(10).unwrap();
            let m3: u32 = move_str.chars().nth(2).unwrap().to_digit(10).unwrap();
            let m4: u32 = move_str.chars().nth(3).unwrap().to_digit(10).unwrap();
            let start_shift: u32 = 64 - 1 - (m1 * 8 + m2);
            let end_shift: u32 = 64 - 1 - (m3 * 8 + m4);
            if ((1 << start_shift) & bitboards[Piece::WK]) != 0 { // white king move
                (castle_rights_t[CastleRights::CWK], castle_rights_t[CastleRights::CWQ]) = (false, false);
            }
            if ((1 << start_shift) & bitboards[Piece::BK]) != 0 { // black king move
                (castle_rights_t[CastleRights::CBK], castle_rights_t[CastleRights::CBQ]) = (false, false);
            }
            if ((1 << start_shift) & bitboards[Piece::WR] & 1) != 0 { // white king side rook move
                castle_rights_t[CastleRights::CWK] = false;
            }
            if ((1 << start_shift) & bitboards[Piece::WR] & (1 << 7)) != 0 { // white queen side rook move
                castle_rights_t[CastleRights::CWQ] = false;
            }
            if ((1 << start_shift) & bitboards[Piece::BR] & (1 << 56)) != 0 { // black king side rook move
                castle_rights_t[CastleRights::CBK] = false;
            }
            if ((1 << start_shift) & bitboards[Piece::BR] & (1 << 63)) != 0 { // black queen side rook move
                castle_rights_t[CastleRights::CBQ] = false;
            }
            if (((1 as i64) << end_shift) & 1) != 0 { // white king side rook taken
                castle_rights_t[CastleRights::CWK] = false;
            }
            if (((1 as i64) << end_shift) & (1 << 7)) != 0 { // white queen side rook taken
                castle_rights_t[CastleRights::CWQ] = false;
            }
            if ((1 << end_shift) & ((1 as i64) << 56)) != 0 { // black king side rook taken
                castle_rights_t[CastleRights::CBK] = false;
            }
            if ((1 << end_shift) & ((1 as i64) << 63)) != 0 { // black queen side rook taken
                castle_rights_t[CastleRights::CBQ] = false;
            }
        }
        castle_rights_t
    }
}
