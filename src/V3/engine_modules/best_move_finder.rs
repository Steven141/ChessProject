//! Module with different ways to find the best next move


use pyo3::prelude::*;
use std::collections::HashMap;
use crate::engine_modules::moves::Moves;


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
    piece_position_scale: f64,
}


/// TODO: Look into transpositions tables and iterative deepening
#[pymethods]
impl BestMoveFinder {
    #[new]
    fn new(search_depth: u32) -> Self {
        BestMoveFinder {
            search_depth: search_depth,
            mate_score: 1000,
            stale_score: 0,
            move_counter: 0,
            best_move_idx: -1,
            considered_moves: String::new(),
            next_move: String::new(),
            piece_scores: HashMap::from([
                ('Q', 9),
                ('R', 5),
                ('B', 3),
                ('N', 3),
                ('P', 1),
            ]),
            piece_position_scores: HashMap::from([
                ('Q', [
                    [1, 1, 1, 3, 1, 1, 1, 1],
                    [1, 2, 3, 3, 3, 1, 1, 1],
                    [1, 4, 3, 3, 3, 4, 2, 1],
                    [1, 2, 3, 3, 3, 2, 2, 1],
                    [1, 2, 3, 3, 3, 2, 2, 1],
                    [1, 4, 3, 3, 3, 4, 2, 1],
                    [1, 2, 3, 3, 3, 1, 1, 1],
                    [1, 1, 1, 3, 1, 1, 1, 1],
                ]),
                ('R', [
                    [4, 3, 4, 4, 4, 4, 3, 4],
                    [4, 4, 4, 4, 4, 4, 4, 4],
                    [1, 1, 2, 3, 3, 2, 1, 1],
                    [1, 2, 3, 4, 4, 3, 2, 1],
                    [1, 2, 3, 4, 4, 3, 2, 1],
                    [1, 1, 2, 3, 3, 2, 1, 1],
                    [4, 4, 4, 4, 4, 4, 4, 4],
                    [4, 3, 4, 4, 4, 4, 3, 4],
                ]),
                ('B', [
                    [4, 3, 2, 1, 1, 2, 3, 4],
                    [3, 4, 3, 2, 2, 3, 4, 3],
                    [2, 3, 4, 3, 3, 4, 3, 2],
                    [1, 2, 3, 4, 4, 3, 2, 1],
                    [1, 2, 3, 4, 4, 3, 2, 1],
                    [2, 3, 4, 3, 3, 4, 3, 2],
                    [3, 4, 3, 2, 2, 3, 4, 3],
                    [4, 3, 2, 1, 1, 2, 3, 4],
                ]),
                ('N', [
                    [1, 1, 1, 1, 1, 1, 1, 1],
                    [1, 2, 2, 2, 2, 2, 2, 1],
                    [1, 2, 3, 3, 3, 3, 2, 1],
                    [1, 2, 3, 4, 4, 3, 2, 1],
                    [1, 2, 3, 4, 4, 3, 2, 1],
                    [1, 2, 3, 3, 3, 3, 2, 1],
                    [1, 2, 2, 2, 2, 2, 2, 1],
                    [1, 1, 1, 1, 1, 1, 1, 1],
                ]),
                ('P', [
                    [8, 8, 8, 8, 8, 8, 8, 8],
                    [8, 8, 8, 8, 8, 8, 8, 8],
                    [5, 6, 6, 7, 7, 6, 6, 5],
                    [2, 3, 3, 5, 5, 3, 3, 2],
                    [1, 2, 3, 4, 4, 3, 2, 1],
                    [1, 1, 2, 3, 3, 2, 1, 1],
                    [1, 1, 1, 0, 0, 1, 1, 1],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ]),
                ('p', [
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 1, 1, 0, 0, 1, 1, 1],
                    [1, 1, 2, 3, 3, 2, 1, 1],
                    [1, 2, 3, 4, 4, 3, 2, 1],
                    [2, 3, 3, 5, 5, 3, 3, 2],
                    [5, 6, 6, 7, 7, 6, 6, 5],
                    [8, 8, 8, 8, 8, 8, 8, 8],
                    [8, 8, 8, 8, 8, 8, 8, 8],
                ]),
            ]),
            piece_position_scale: 0.1,
        }
    }


    fn negaMaxAlphaBeta(&mut self, mut alpha: f64, beta: f64, mm: &mut Moves, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, EP: i64, cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, depth: u32) -> f64 {
        // Positive = better for current recursive player perspective
        self.move_counter += 1;
        if depth == self.search_depth {
            return (if whites_turn {1.0} else {-1.0}) * self.evaluate(mm, wP, wN, wB, wR, wQ, bP, bN, bB, bR, bQ, whites_turn);
        }
        let mut best_score: f64 = -self.mate_score as f64;
        let valid_moves: String = mm.getValidMoves(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ, whites_turn, depth);
        if mm.stalemate {
            return self.stale_score as f64;
        }
        for i in (0..valid_moves.len()).step_by(4) {
            let wPt: i64 = mm.makeMove(wP, valid_moves[i..i+4].to_string(), 'P'); let wNt: i64 = mm.makeMove(wN, valid_moves[i..i+4].to_string(), 'N');
            let wBt: i64 = mm.makeMove(wB, valid_moves[i..i+4].to_string(), 'B'); let wRt: i64 = mm.makeMove(wR, valid_moves[i..i+4].to_string(), 'R');
            let wQt: i64 = mm.makeMove(wQ, valid_moves[i..i+4].to_string(), 'Q'); let wKt: i64 = mm.makeMove(wK, valid_moves[i..i+4].to_string(), 'K');
            let bPt: i64 = mm.makeMove(bP, valid_moves[i..i+4].to_string(), 'p'); let bNt: i64 = mm.makeMove(bN, valid_moves[i..i+4].to_string(), 'n');
            let bBt: i64 = mm.makeMove(bB, valid_moves[i..i+4].to_string(), 'b'); let bRt: i64 = mm.makeMove(bR, valid_moves[i..i+4].to_string(), 'r');
            let bQt: i64 = mm.makeMove(bQ, valid_moves[i..i+4].to_string(), 'q'); let bKt: i64 = mm.makeMove(bK, valid_moves[i..i+4].to_string(), 'k');
            let wRt: i64 = mm.makeMoveCastle(wRt, wK, valid_moves[i..i+4].to_string(), 'R'); let bRt: i64 = mm.makeMoveCastle(bRt, bK, valid_moves[i..i+4].to_string(), 'r');
            let EPt: i64 = mm.makeMoveEP(wP|bP, valid_moves[i..i+4].to_string());

            let mut cwKt: bool = cwK; let mut cwQt: bool = cwQ; let mut cbKt: bool = cbK; let mut cbQt: bool = cbQ;

            if valid_moves.chars().nth(i + 3).unwrap().is_numeric() {
                let m1: u32 = valid_moves.chars().nth(i).unwrap().to_digit(10).unwrap();
                let m2: u32 = valid_moves.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                let m3: u32 = valid_moves.chars().nth(i + 2).unwrap().to_digit(10).unwrap();
                let m4: u32 = valid_moves.chars().nth(i + 3).unwrap().to_digit(10).unwrap();
                let start_shift: u32 = 64 - 1 - (m1 * 8 + m2);
                let end_shift: u32 = 64 - 1 - (m3 * 8 + m4);
                if ((1 << start_shift) & wK) != 0 { // white king move
                    (cwKt, cwQt) = (false, false);
                }
                if ((1 << start_shift) & bK) != 0 { // black king move
                    (cbKt, cbQt) = (false, false);
                }
                if ((1 << start_shift) & wR & 1) != 0 { // white king side rook move
                    cwKt = false;
                }
                if ((1 << start_shift) & wR & (1 << 7)) != 0 { // white queen side rook move
                    cwQt = false;
                }
                if ((1 << start_shift) & bR & (1 << 56)) != 0 { // black king side rook move
                    cbKt = false;
                }
                if ((1 << start_shift) & bR & (1 << 63)) != 0 { // black queen side rook move
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

            let mut score: f64 = -self.negaMaxAlphaBeta(-beta, -alpha, mm, wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt, EPt, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth+1);
            if score == self.mate_score as f64 {
                score -= depth as f64;
            }
            if score > best_score {
                best_score = score;
                if depth == 0 {
                    self.best_move_idx = i as i64;
                    self.next_move = valid_moves[i..i+4].to_string();
                    println!("Considering {:?} with score: {:?}", move_to_algebra!(valid_moves[i..i+4]), score);
                }
            }

            if best_score > alpha {
                alpha = best_score;
            }
            if alpha >= beta {
                break;
            }
        }
        best_score
    }


    fn evaluate(&self, mm: &Moves, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, whites_turn: bool) -> f64 {
        if mm.checkmate {
            return if whites_turn {-self.mate_score as f64} else {self.mate_score as f64};
        } else if mm.stalemate {
            return self.stale_score as f64;
        }
        
        let mut score: f64 = 0.0;
        for i in 0..64 {
            let shift = 64 - 1 - i;
            if usgn_r_shift!(wP, shift) & 1 == 1 {
                score += (self.piece_scores[&'P'] as f64) + (self.piece_position_scores[&'P'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
            if usgn_r_shift!(wN, shift) & 1 == 1 {
                score += (self.piece_scores[&'N'] as f64) + (self.piece_position_scores[&'N'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
            if usgn_r_shift!(wB, shift) & 1 == 1 {
                score += (self.piece_scores[&'B'] as f64) + (self.piece_position_scores[&'B'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
            if usgn_r_shift!(wR, shift) & 1 == 1 {
                score += (self.piece_scores[&'R'] as f64) + (self.piece_position_scores[&'R'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
            if usgn_r_shift!(wQ, shift) & 1 == 1 {
                score += (self.piece_scores[&'Q'] as f64) + (self.piece_position_scores[&'Q'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
            if usgn_r_shift!(bP, shift) & 1 == 1 {
                score -= (self.piece_scores[&'P'] as f64) + (self.piece_position_scores[&'p'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
            if usgn_r_shift!(bN, shift) & 1 == 1 {
                score -= (self.piece_scores[&'N'] as f64) + (self.piece_position_scores[&'N'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
            if usgn_r_shift!(bB, shift) & 1 == 1 {
                score -= (self.piece_scores[&'B'] as f64) + (self.piece_position_scores[&'B'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
            if usgn_r_shift!(bR, shift) & 1 == 1 {
                score -= (self.piece_scores[&'R'] as f64) + (self.piece_position_scores[&'R'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
            if usgn_r_shift!(bQ, shift) & 1 == 1 {
                score -= (self.piece_scores[&'Q'] as f64) + (self.piece_position_scores[&'Q'][i / 8][i % 8] as f64) * self.piece_position_scale;
            }
        }
        score
    }
}