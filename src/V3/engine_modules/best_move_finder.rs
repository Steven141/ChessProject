//! Module with different ways to find the best next move


use pyo3::prelude::*;
use crate::moves::Moves;
use crate::piece::Piece;
use std::collections::HashMap;
use std::str::from_utf8;
use rand::thread_rng;
use rand::seq::SliceRandom;


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct BestMoveFinder {
    search_depth: u32,
    mate_score: i64,
    stale_score: i64,
    move_counter: u32,
    next_move: String,
    piece_scores: HashMap<char, i64>,
    piece_position_scores: HashMap<char, [[i64; 8]; 8]>,
    mvv_lva: [[i64; 12]; 12], // [attacker][victim]
}


/// TODO: Look into transpositions tables and iterative deepening
#[pymethods]
impl BestMoveFinder {
    #[new]
    fn new(search_depth: u32) -> Self {
        BestMoveFinder {
            search_depth: search_depth,
            mate_score: 10000,
            stale_score: 0,
            move_counter: 0,
            next_move: String::new(),
            piece_scores: HashMap::from([
                ('K', 10000),
                ('Q', 1000),
                ('R', 500),
                ('B', 350),
                ('N', 300),
                ('P', 100),
            ]),
            piece_position_scores: HashMap::from([
                ('K', [
                    [0,   0,   0,   0,   0,   0,   0,   0],
                    [0,   0,   5,   5,   5,   5,   0,   0],
                    [0,   5,   5,  10,  10,   5,   5,   0],
                    [0,   5,  10,  20,  20,  10,   5,   0],
                    [0,   5,  10,  20,  20,  10,   5,   0],
                    [0,   0,   5,  10,  10,   5,   0,   0],
                    [0,   5,   5,  -5,  -5,   0,   5,   0],
                    [0,   0,   5,   0, -15,   0,  10,   0],
                ]),
                ('Q', [
                    [0,   0,   0,  10,   0,   0,   0,   0],
                    [0,   5,  10,  10,  10,   0,   0,   0],
                    [0,  10,  10,  10,  10,  10,   5,   0],
                    [0,   5,  10,  10,  10,   5,   5,   0],
                    [0,   5,  10,  10,  10,   5,   5,   0],
                    [0,  10,  10,  10,  10,  10,   5,   0],
                    [0,   5,  10,  10,  10,   0,   0,   0],
                    [0,   0,   0,  10,   0,   0,   0,   0],
                ]),
                ('R', [
                    [50,  50,  50,  50,  50,  50,  50,  50],
                    [50,  50,  50,  50,  50,  50,  50,  50],
                    [ 0,   0,  10,  20,  20,  10,   0,   0],
                    [ 0,   0,  10,  20,  20,  10,   0,   0],
                    [ 0,   0,  10,  20,  20,  10,   0,   0],
                    [ 0,   0,  10,  20,  20,  10,   0,   0],
                    [ 0,   0,  10,  20,  20,  10,   0,   0],
                    [ 0,   0,   0,  20,  20,   0,   0,   0],
                ]),
                ('B', [
                    [0,   0,   0,   0,   0,   0,   0,   0],
                    [0,   0,   0,   0,   0,   0,   0,   0],
                    [0,   0,   0,  10,  10,   0,   0,   0],
                    [0,   0,  10,  20,  20,  10,   0,   0],
                    [0,   0,  10,  20,  20,  10,   0,   0],
                    [0,  10,   0,   0,   0,   0,  10,   0],
                    [0,  30,   0,   0,   0,   0,  30,   0],
                    [0,   0, -10,   0,   0, -10,   0,   0],
                ]),
                ('N', [
                    [-5,   0,   0,   0,   0,   0,   0,  -5],
                    [-5,   0,   0,  10,  10,   0,   0,  -5],
                    [-5,   5,  20,  20,  20,  20,   5,  -5],
                    [-5,  10,  20,  30,  30,  20,  10,  -5],
                    [-5,  10,  20,  30,  30,  20,  10,  -5],
                    [-5,   5,  20,  10,  10,  20,   5,  -5],
                    [-5,   0,   0,   0,   0,   0,   0,  -5],
                    [-5, -10,   0,   0,   0,   0, -10,  -5],
                ]),
                ('P', [
                    [90,  90,  90,  90,  90,  90,  90,  90],
                    [30,  30,  30,  40,  40,  30,  30,  30],
                    [20,  20,  20,  30,  30,  30,  20,  20],
                    [10,  10,  10,  20,  20,  10,  10,  10],
                    [ 5,   5,  10,  20,  20,   5,   5,   5],
                    [ 0,   0,   0,   5,   5,   0,   0,   0],
                    [ 0,   0,   0, -10, -10,   0,   0,   0],
                    [ 0,   0,   0,   0,   0,   0,   0,   0],
                ]),
            ]),
            mvv_lva: [
                // (Victims) Pawn Knight Bishop   Rook  Queen   King            (Attackers)
                [105, 205, 305, 405, 505, 605,  105, 205, 305, 405, 505, 605], // Pawn
                [104, 204, 304, 404, 504, 604,  104, 204, 304, 404, 504, 604], // Knight
                [103, 203, 303, 403, 503, 603,  103, 203, 303, 403, 503, 603], // Bishop
                [102, 202, 302, 402, 502, 602,  102, 202, 302, 402, 502, 602], // Rook
                [101, 201, 301, 401, 501, 601,  101, 201, 301, 401, 501, 601], // Queen
                [100, 200, 300, 400, 500, 600,  100, 200, 300, 400, 500, 600], // King

                [105, 205, 305, 405, 505, 605,  105, 205, 305, 405, 505, 605],
                [104, 204, 304, 404, 504, 604,  104, 204, 304, 404, 504, 604],
                [103, 203, 303, 403, 503, 603,  103, 203, 303, 403, 503, 603],
                [102, 202, 302, 402, 502, 602,  102, 202, 302, 402, 502, 602],
                [101, 201, 301, 401, 501, 601,  101, 201, 301, 401, 501, 601],
                [100, 200, 300, 400, 500, 600,  100, 200, 300, 400, 500, 600],
            ],
        }
    }


    fn quiescenceSearch(&mut self, mut alpha: i64, beta: i64, mm: &mut Moves, bitboards: [i64; 13], castle_rights: [bool; 4], whites_turn: bool) -> i64 {
        // look deeper for non-quiet moves (attacking)
        self.move_counter += 1;
        let eval: i64 = (if whites_turn {1} else {-1}) * self.evaluateBoard(mm, bitboards, whites_turn);
        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }
        let moves: String = mm.getPossibleMoves(bitboards, castle_rights, whites_turn);
        for i in (0..moves.len()).step_by(4) {
            let bitboards_t: [i64; 13] = mm.getUpdatedBitboards(&moves[i..i+4], bitboards);
            let castle_rights_t: [bool; 4] = mm.getUpdatedCastleRights(&moves[i..i+4], castle_rights, bitboards);
            if mm.isAttackingMove(bitboards, bitboards_t, whites_turn) {
                let score: i64 = -self.quiescenceSearch(-beta, -alpha, mm, bitboards_t, castle_rights_t, !whites_turn);
                if score >= beta {
                    return beta;
                }
                if score > alpha {
                    alpha = score;
                }
            }
        }
        alpha
    }


    fn negaMaxAlphaBeta(&mut self, mut alpha: i64, beta: i64, mm: &mut Moves, bitboards: [i64; 13], castle_rights: [bool; 4], whites_turn: bool, depth: u32) -> i64 {
        // Positive = better for current recursive player perspective
        // alpha = minimum score that the maximizing player is assured of
        // beta = maximum score that the minimizing player is assured of
        // TODO: add depth for quiescenceSearch
        if depth == self.search_depth {
            return self.quiescenceSearch(alpha, beta, mm, bitboards, castle_rights, whites_turn);
            // self.move_counter += 1;
            // return (if whites_turn {1} else {-1}) * self.evaluateBoard(mm, bitboards, whites_turn);
        }
        self.move_counter += 1;
        let mut best_score: i64 = -self.mate_score;
        let mut moves: String = mm.getPossibleMoves(bitboards, castle_rights, whites_turn);
        if depth == 0 {
            // TODO: look to replace shuffling with sorting
            println!("Search Depth: {:?}", self.search_depth);
            let mut move_groups: Vec<&str> = moves.as_bytes().chunks(4).map(|chunk| from_utf8(chunk).unwrap()).collect();
            move_groups.shuffle(&mut thread_rng());
            moves = move_groups.join("");
        }
        let mut valid_move_found: bool = false;
        for i in (0..moves.len()).step_by(4) {
            let bitboards_t: [i64; 13] = mm.getUpdatedBitboards(&moves[i..i+4], bitboards);
            let castle_rights_t: [bool; 4] = mm.getUpdatedCastleRights(&moves[i..i+4], castle_rights, bitboards);

            if mm.isValidMove(bitboards_t, whites_turn) {
                valid_move_found = true;
                let mut score: i64 = -self.negaMaxAlphaBeta(-beta, -alpha, mm, bitboards_t, castle_rights_t, !whites_turn, depth+1);

                if score == self.mate_score {
                    score -= depth as i64;
                }
                if score > best_score {
                    best_score = score;
                    if depth == 0 {
                        self.next_move = moves[i..i+4].to_string();
                        println!("Considering {:?} with score: {:?}", move_to_algebra!(moves[i..i+4]), score);
                    }
                }

                if best_score > alpha {
                    alpha = best_score;
                }
                if alpha >= beta {
                    break;
                }
            }
        }
        if !valid_move_found {
            if ((bitboards[Piece::WK] & mm.unsafeForWhite(bitboards)) != 0 && whites_turn) || ((bitboards[Piece::BK] & mm.unsafeForBlack(bitboards)) != 0 && !whites_turn) {
                mm.checkmate = true;
            } else {
                mm.stalemate = true;
                return self.stale_score;
            }
        } else {
            mm.checkmate = false;
            mm.stalemate = false;
        }
        best_score
    }


    fn evaluateBoard(&self, mm: &Moves, bitboards: [i64; 13], whites_turn: bool) -> i64 {
        if mm.checkmate {
            return if whites_turn {-self.mate_score} else {self.mate_score};
        } else if mm.stalemate {
            return self.stale_score;
        }
        
        let mut score: i64 = 0;
        for i in 0..64 {
            let shift = 64 - 1 - i;
            if usgn_r_shift!(bitboards[Piece::WP], shift) & 1 == 1 {
                score += self.piece_scores[&'P'] + self.piece_position_scores[&'P'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WN], shift) & 1 == 1 {
                score += self.piece_scores[&'N'] + self.piece_position_scores[&'N'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WB], shift) & 1 == 1 {
                score += self.piece_scores[&'B'] + self.piece_position_scores[&'B'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WR], shift) & 1 == 1 {
                score += self.piece_scores[&'R'] + self.piece_position_scores[&'R'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WQ], shift) & 1 == 1 {
                score += self.piece_scores[&'Q']  + self.piece_position_scores[&'Q'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WK], shift) & 1 == 1 {
                score += self.piece_scores[&'K'] + self.piece_position_scores[&'K'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BP], shift) & 1 == 1 {
                score -= self.piece_scores[&'P'] + self.piece_position_scores[&'P'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BN], shift) & 1 == 1 {
                score -= self.piece_scores[&'N'] + self.piece_position_scores[&'N'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BB], shift) & 1 == 1 {
                score -= self.piece_scores[&'B'] + self.piece_position_scores[&'B'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BR], shift) & 1 == 1 {
                score -= self.piece_scores[&'R'] + self.piece_position_scores[&'R'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BQ], shift) & 1 == 1 {
                score -= self.piece_scores[&'Q'] + self.piece_position_scores[&'Q'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BK], shift) & 1 == 1 {
                score -= self.piece_scores[&'K'] + self.piece_position_scores[&'K'][7 - (i / 8)][i % 8];
            }
        }
        score
    }


    fn scoreMove(&self, mm: &mut Moves, bitboards: [i64; 13], move_str: &str, whites_turn: bool) -> i64 {
        let start_shift: u32; let end_shift: u32;
        let start_bitboard: i64; let end_bitboard: i64;
        let mut attacker: Piece = Piece::EP; let mut victim: Piece = Piece::EP; // EP used as default value (no attacker / no victim)
        if move_str.chars().nth(3).unwrap().is_numeric() { // regular move
            let (r1, c1, r2, c2) = move_to_u32s!(move_str);
            start_shift = 64 - 1 - (r1 * 8 + c1);
            end_shift = 64 - 1 - (r2 * 8 + c2);
        } else if move_str.chars().nth(3).unwrap() == 'P' { // pawn promo
            let (c1, c2, _, _) = move_to_u32s!(move_str);
            if move_str.chars().nth(2).unwrap().is_uppercase() { // white promo
                start_bitboard = mm.masks.file_masks[c1 as usize] & mm.masks.rank_masks[1];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = mm.masks.file_masks[c2 as usize] & mm.masks.rank_masks[0];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
            } else { // black promo
                start_bitboard = mm.masks.file_masks[c1 as usize] & mm.masks.rank_masks[6];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = mm.masks.file_masks[c2 as usize] & mm.masks.rank_masks[7];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
            }
        } else if move_str.chars().nth(3).unwrap() == 'E' { // enpassant
            let (c1, c2, _, _) = move_to_u32s!(move_str);
            if move_str.chars().nth(2).unwrap() == 'w' { // white
                start_bitboard = mm.masks.file_masks[c1 as usize] & mm.masks.rank_masks[3];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = mm.masks.file_masks[c2 as usize] & mm.masks.rank_masks[2];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
            } else { // black
                start_bitboard = mm.masks.file_masks[c1 as usize] & mm.masks.rank_masks[4];
                start_shift = 64 - 1 - start_bitboard.leading_zeros();
                end_bitboard = mm.masks.file_masks[c2 as usize] & mm.masks.rank_masks[5];
                end_shift = 64 - 1 - end_bitboard.leading_zeros();
            }
        } else {
            panic!("INVALID MOVE TYPE");
        }
        let possible_attackers: [Piece; 6] = if whites_turn {[Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK]} else {[Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK]};
        let possible_victims: [Piece; 6] = if !whites_turn {[Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK]} else {[Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK]};
        for piece in possible_attackers {
            if usgn_r_shift!(bitboards[piece], start_shift) & 1 == 1 {
                attacker = piece;
            }
        }
        for piece in possible_victims {
            if usgn_r_shift!(bitboards[piece], end_shift) & 1 == 1 {
                victim = piece;
            }
        }
        if victim != Piece::EP { // attacking move
            return self.mvv_lva[attacker][victim];
        }
        0
    }
}


/// Tests


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine_modules::game_state::GameState;

    #[test]
    fn score_move_test() {
        let mut gs = GameState::new();
        gs.importFEN(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 "));
        let mut m: Moves = Moves::new();
        let bmf: BestMoveFinder = BestMoveFinder::new(2);
        let moves: String = m.getPossibleMoves(gs.bitboards, gs.castle_rights, gs.whites_turn);
        let mut actual_scores: Vec<i64> = vec![105, 105, 303, 101, 201, 104, 104, 104];
        for i in (0..moves.len()).step_by(4) {
            let bitboards_t: [i64; 13] = m.getUpdatedBitboards(&moves[i..i+4], gs.bitboards);
            if m.isValidMove(bitboards_t, gs.whites_turn) {
                let score = bmf.scoreMove(&mut m, gs.bitboards, &moves[i..i+4], gs.whites_turn);
                if score != 0 {
                    assert!(score == actual_scores.remove(0));
                }
            }
        }
    }
}
