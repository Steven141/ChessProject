//! Module with different ways to find the best next move


use pyo3::prelude::*;
use std::collections::HashMap;
use crate::moves::Moves;
use crate::piece::Piece;
use std::str::from_utf8;
use rand::thread_rng;
use rand::seq::SliceRandom;


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct BestMoveFinder {
    search_depth: u32,
    mate_score: i64,
    stale_score: i64,
    move_counter: u32,
    best_move_idx: i64,
    considered_moves: String,
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
            best_move_idx: -1,
            considered_moves: String::new(),
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


    fn quiescenceSearch(&mut self, mut alpha: i64, beta: i64, mm: &mut Moves, bitboards: [i64; 13], cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool) -> i64 {
        // look deeper for non-quiet moves (attacking)
        self.move_counter += 1;
        let eval: i64 = (if whites_turn {1} else {-1}) * self.evaluateBoard(mm, bitboards, whites_turn);
        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }
        let moves: String;
        if whites_turn {
            moves = mm.possibleMovesW(bitboards, cwK, cwQ);
        } else {
            moves = mm.possibleMovesB(bitboards, cbK, cbQ);
        }
        for i in (0..moves.len()).step_by(4) {
            let mut bitboards_t: [i64; 13] = [0; 13];
            bitboards_t[Piece::WP.idx()] = mm.makeMove(bitboards[Piece::WP.idx()], moves[i..i+4].to_string(), 'P'); bitboards_t[Piece::WN.idx()] = mm.makeMove(bitboards[Piece::WN.idx()], moves[i..i+4].to_string(), 'N');
            bitboards_t[Piece::WB.idx()] = mm.makeMove(bitboards[Piece::WB.idx()], moves[i..i+4].to_string(), 'B'); bitboards_t[Piece::WR.idx()] = mm.makeMove(bitboards[Piece::WR.idx()], moves[i..i+4].to_string(), 'R');
            bitboards_t[Piece::WQ.idx()] = mm.makeMove(bitboards[Piece::WQ.idx()], moves[i..i+4].to_string(), 'Q'); bitboards_t[Piece::WK.idx()] = mm.makeMove(bitboards[Piece::WK.idx()], moves[i..i+4].to_string(), 'K');
            bitboards_t[Piece::BP.idx()] = mm.makeMove(bitboards[Piece::BP.idx()], moves[i..i+4].to_string(), 'p'); bitboards_t[Piece::BN.idx()] = mm.makeMove(bitboards[Piece::BN.idx()], moves[i..i+4].to_string(), 'n');
            bitboards_t[Piece::BB.idx()] = mm.makeMove(bitboards[Piece::BB.idx()], moves[i..i+4].to_string(), 'b'); bitboards_t[Piece::BR.idx()] = mm.makeMove(bitboards[Piece::BR.idx()], moves[i..i+4].to_string(), 'r');
            bitboards_t[Piece::BQ.idx()] = mm.makeMove(bitboards[Piece::BQ.idx()], moves[i..i+4].to_string(), 'q'); bitboards_t[Piece::BK.idx()] = mm.makeMove(bitboards[Piece::BK.idx()], moves[i..i+4].to_string(), 'k');
            bitboards_t[Piece::WR.idx()] = mm.makeMoveCastle(bitboards_t[Piece::WR.idx()], bitboards[Piece::WK.idx()], moves[i..i+4].to_string(), 'R'); bitboards_t[Piece::BR.idx()] = mm.makeMoveCastle(bitboards_t[Piece::BR.idx()], bitboards[Piece::BK.idx()], moves[i..i+4].to_string(), 'r');
            bitboards_t[Piece::EP.idx()] = mm.makeMoveEP(bitboards[Piece::WP.idx()] | bitboards[Piece::BP.idx()], moves[i..i+4].to_string());

            let mut cwKt: bool = cwK; let mut cwQt: bool = cwQ; let mut cbKt: bool = cbK; let mut cbQt: bool = cbQ;

            if moves.chars().nth(i + 3).unwrap().is_numeric() {
                let m1: u32 = moves.chars().nth(i).unwrap().to_digit(10).unwrap();
                let m2: u32 = moves.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                let m3: u32 = moves.chars().nth(i + 2).unwrap().to_digit(10).unwrap();
                let m4: u32 = moves.chars().nth(i + 3).unwrap().to_digit(10).unwrap();
                let start_shift: u32 = 64 - 1 - (m1 * 8 + m2);
                let end_shift: u32 = 64 - 1 - (m3 * 8 + m4);
                if ((1 << start_shift) & bitboards[Piece::WK.idx()]) != 0 { // white king move
                    (cwKt, cwQt) = (false, false);
                }
                if ((1 << start_shift) & bitboards[Piece::BK.idx()]) != 0 { // black king move
                    (cbKt, cbQt) = (false, false);
                }
                if ((1 << start_shift) & bitboards[Piece::WR.idx()] & 1) != 0 { // white king side rook move
                    cwKt = false;
                }
                if ((1 << start_shift) & bitboards[Piece::WR.idx()] & (1 << 7)) != 0 { // white queen side rook move
                    cwQt = false;
                }
                if ((1 << start_shift) & bitboards[Piece::BR.idx()] & (1 << 56)) != 0 { // black king side rook move
                    cbKt = false;
                }
                if ((1 << start_shift) & bitboards[Piece::BR.idx()] & (1 << 63)) != 0 { // black queen side rook move
                    cbQt = false;
                }
                if (((1 as i64) << end_shift) & 1) != 0 { // white king side rook taken
                    cwKt = false;
                }
                if (((1 as i64) << end_shift) & (1 << 7)) != 0 { // white queen side rook taken
                    cwQt = false;
                }
                if ((1 << end_shift) & ((1 as i64) << 56)) != 0 { // black king side rook taken
                    cbKt = false;
                }
                if ((1 << end_shift) & ((1 as i64) << 63)) != 0 { // black queen side rook taken
                    cbQt = false;
                }
            }
            let is_valid_move: bool = ((bitboards_t[Piece::WK.idx()] & mm.unsafeForWhite(bitboards_t)) == 0 && whites_turn) || ((bitboards_t[Piece::BK.idx()] & mm.unsafeForBlack(bitboards_t)) == 0 && !whites_turn);
            let is_attacking_move: bool = is_valid_move && ((bitboards[Piece::WP.idx()]|bitboards[Piece::WN.idx()]|bitboards[Piece::WB.idx()]|bitboards[Piece::WR.idx()]|bitboards[Piece::WQ.idx()]).count_ones() != (bitboards_t[Piece::WP.idx()]|bitboards_t[Piece::WN.idx()]|bitboards_t[Piece::WB.idx()]|bitboards_t[Piece::WR.idx()]|bitboards_t[Piece::WQ.idx()]).count_ones() || (bitboards[Piece::BP.idx()]|bitboards[Piece::BN.idx()]|bitboards[Piece::BB.idx()]|bitboards[Piece::BR.idx()]|bitboards[Piece::BQ.idx()]).count_ones() != (bitboards_t[Piece::BP.idx()]|bitboards_t[Piece::BN.idx()]|bitboards_t[Piece::BB.idx()]|bitboards_t[Piece::BR.idx()]|bitboards_t[Piece::BQ.idx()]).count_ones());
            if is_attacking_move {
                let score: i64 = -self.quiescenceSearch(-beta, -alpha, mm, bitboards_t, cwKt, cwQt, cbKt, cbQt, !whites_turn);
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


    fn negaMaxAlphaBeta(&mut self, mut alpha: i64, beta: i64, mm: &mut Moves, bitboards: [i64; 13], cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, depth: u32) -> i64 {
        // Positive = better for current recursive player perspective
        // alpha = minimum score that the maximizing player is assured of
        // beta = maximum score that the minimizing player is assured of
        self.move_counter += 1;
        if depth == self.search_depth {
            return self.quiescenceSearch(alpha, beta, mm, bitboards, cwK, cwQ, cbK, cbQ, whites_turn);
            // return (if whites_turn {1} else {-1}) * self.evaluateBoard(mm, bitboards, whites_turn);
        }
        // self.move_counter += 1;
        let mut best_score: i64 = -self.mate_score;
        let mut moves: String;
        if whites_turn {
            moves = mm.possibleMovesW(bitboards, cwK, cwQ);
        } else {
            moves = mm.possibleMovesB(bitboards, cbK, cbQ);
        }
        if depth == 0 {
            // TODO: look to replace shuffling with sorting
            println!("Depth: {:?}", self.search_depth);
            let mut move_groups: Vec<&str> = moves.as_bytes().chunks(4).map(|chunk| from_utf8(chunk).unwrap()).collect();
            move_groups.shuffle(&mut thread_rng());
            moves = move_groups.join("");
        }
        let mut valid_move_found: bool = false;
        for i in (0..moves.len()).step_by(4) {
            let mut bitboards_t: [i64; 13] = [0; 13];
            bitboards_t[Piece::WP.idx()] = mm.makeMove(bitboards[Piece::WP.idx()], moves[i..i+4].to_string(), 'P'); bitboards_t[Piece::WN.idx()] = mm.makeMove(bitboards[Piece::WN.idx()], moves[i..i+4].to_string(), 'N');
            bitboards_t[Piece::WB.idx()] = mm.makeMove(bitboards[Piece::WB.idx()], moves[i..i+4].to_string(), 'B'); bitboards_t[Piece::WR.idx()] = mm.makeMove(bitboards[Piece::WR.idx()], moves[i..i+4].to_string(), 'R');
            bitboards_t[Piece::WQ.idx()] = mm.makeMove(bitboards[Piece::WQ.idx()], moves[i..i+4].to_string(), 'Q'); bitboards_t[Piece::WK.idx()] = mm.makeMove(bitboards[Piece::WK.idx()], moves[i..i+4].to_string(), 'K');
            bitboards_t[Piece::BP.idx()] = mm.makeMove(bitboards[Piece::BP.idx()], moves[i..i+4].to_string(), 'p'); bitboards_t[Piece::BN.idx()] = mm.makeMove(bitboards[Piece::BN.idx()], moves[i..i+4].to_string(), 'n');
            bitboards_t[Piece::BB.idx()] = mm.makeMove(bitboards[Piece::BB.idx()], moves[i..i+4].to_string(), 'b'); bitboards_t[Piece::BR.idx()] = mm.makeMove(bitboards[Piece::BR.idx()], moves[i..i+4].to_string(), 'r');
            bitboards_t[Piece::BQ.idx()] = mm.makeMove(bitboards[Piece::BQ.idx()], moves[i..i+4].to_string(), 'q'); bitboards_t[Piece::BK.idx()] = mm.makeMove(bitboards[Piece::BK.idx()], moves[i..i+4].to_string(), 'k');
            bitboards_t[Piece::WR.idx()] = mm.makeMoveCastle(bitboards_t[Piece::WR.idx()], bitboards[Piece::WK.idx()], moves[i..i+4].to_string(), 'R'); bitboards_t[Piece::BR.idx()] = mm.makeMoveCastle(bitboards_t[Piece::BR.idx()], bitboards[Piece::BK.idx()], moves[i..i+4].to_string(), 'r');
            bitboards_t[Piece::EP.idx()] = mm.makeMoveEP(bitboards[Piece::WP.idx()] | bitboards[Piece::BP.idx()], moves[i..i+4].to_string());

            let mut cwKt: bool = cwK; let mut cwQt: bool = cwQ; let mut cbKt: bool = cbK; let mut cbQt: bool = cbQ;

            if moves.chars().nth(i + 3).unwrap().is_numeric() {
                let m1: u32 = moves.chars().nth(i).unwrap().to_digit(10).unwrap();
                let m2: u32 = moves.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                let m3: u32 = moves.chars().nth(i + 2).unwrap().to_digit(10).unwrap();
                let m4: u32 = moves.chars().nth(i + 3).unwrap().to_digit(10).unwrap();
                let start_shift: u32 = 64 - 1 - (m1 * 8 + m2);
                let end_shift: u32 = 64 - 1 - (m3 * 8 + m4);
                if ((1 << start_shift) & bitboards[Piece::WK.idx()]) != 0 { // white king move
                    (cwKt, cwQt) = (false, false);
                }
                if ((1 << start_shift) & bitboards[Piece::BK.idx()]) != 0 { // black king move
                    (cbKt, cbQt) = (false, false);
                }
                if ((1 << start_shift) & bitboards[Piece::WR.idx()] & 1) != 0 { // white king side rook move
                    cwKt = false;
                }
                if ((1 << start_shift) & bitboards[Piece::WR.idx()] & (1 << 7)) != 0 { // white queen side rook move
                    cwQt = false;
                }
                if ((1 << start_shift) & bitboards[Piece::BR.idx()] & (1 << 56)) != 0 { // black king side rook move
                    cbKt = false;
                }
                if ((1 << start_shift) & bitboards[Piece::BR.idx()] & (1 << 63)) != 0 { // black queen side rook move
                    cbQt = false;
                }
                if (((1 as i64) << end_shift) & 1) != 0 { // white king side rook taken
                    cwKt = false;
                }
                if (((1 as i64) << end_shift) & (1 << 7)) != 0 { // white queen side rook taken
                    cwQt = false;
                }
                if ((1 << end_shift) & ((1 as i64) << 56)) != 0 { // black king side rook taken
                    cbKt = false;
                }
                if ((1 << end_shift) & ((1 as i64) << 63)) != 0 { // black queen side rook taken
                    cbQt = false;
                }
            }

            let is_valid_move: bool = ((bitboards_t[Piece::WK.idx()] & mm.unsafeForWhite(bitboards_t)) == 0 && whites_turn) || ((bitboards_t[Piece::BK.idx()] & mm.unsafeForBlack(bitboards_t)) == 0 && !whites_turn);
            if is_valid_move {

                valid_move_found = true;

                let mut score: i64 = -self.negaMaxAlphaBeta(-beta, -alpha, mm, bitboards_t, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth+1);
                if score == self.mate_score {
                    score -= depth as i64;
                }
                if score > best_score {
                    best_score = score;
                    if depth == 0 {
                        self.best_move_idx = i as i64;
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
            if ((bitboards[Piece::WK.idx()] & mm.unsafeForWhite(bitboards)) != 0 && whites_turn) || ((bitboards[Piece::BK.idx()] & mm.unsafeForBlack(bitboards)) != 0 && !whites_turn) {
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
            if usgn_r_shift!(bitboards[Piece::WP.idx()], shift) & 1 == 1 {
                score += self.piece_scores[&'P'] + self.piece_position_scores[&'P'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WN.idx()], shift) & 1 == 1 {
                score += self.piece_scores[&'N'] + self.piece_position_scores[&'N'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WB.idx()], shift) & 1 == 1 {
                score += self.piece_scores[&'B'] + self.piece_position_scores[&'B'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WR.idx()], shift) & 1 == 1 {
                score += self.piece_scores[&'R'] + self.piece_position_scores[&'R'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WQ.idx()], shift) & 1 == 1 {
                score += self.piece_scores[&'Q']  + self.piece_position_scores[&'Q'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::WK.idx()], shift) & 1 == 1 {
                score += self.piece_scores[&'K'] + self.piece_position_scores[&'K'][i / 8][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BP.idx()], shift) & 1 == 1 {
                score -= self.piece_scores[&'P'] + self.piece_position_scores[&'P'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BN.idx()], shift) & 1 == 1 {
                score -= self.piece_scores[&'N'] + self.piece_position_scores[&'N'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BB.idx()], shift) & 1 == 1 {
                score -= self.piece_scores[&'B'] + self.piece_position_scores[&'B'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BR.idx()], shift) & 1 == 1 {
                score -= self.piece_scores[&'R'] + self.piece_position_scores[&'R'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BQ.idx()], shift) & 1 == 1 {
                score -= self.piece_scores[&'Q'] + self.piece_position_scores[&'Q'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BK.idx()], shift) & 1 == 1 {
                score -= self.piece_scores[&'K'] + self.piece_position_scores[&'K'][7 - (i / 8)][i % 8];
            }
        }
        score
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine_modules::game_state::GameState;
    use crate::engine_modules::moves::Moves;

    #[test]
    fn qu_test() {
        println!("Basic Test!");
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        // let mut p: Perft = Perft::new(3);
        let mut bmf: BestMoveFinder = BestMoveFinder::new(2);
        bmf.negaMaxAlphaBeta(-10000, 10000, &mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        println!("DONE!");
        // panic!();
    }
}