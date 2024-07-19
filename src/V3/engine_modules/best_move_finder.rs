//! Module with different ways to find the best next move


use pyo3::prelude::*;
use crate::castle_rights::CastleRights;
use crate::moves::Moves;
use crate::piece::Piece;
use std::collections::HashMap;
use std::time::{Duration, Instant};


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct BestMoveFinder {
    search_depth: u32,
    max_depth: u32,
    mate_score: i64,
    stale_score: i64,
    move_counter: u32,
    piece_scores: HashMap<char, i64>,
    piece_position_scores: HashMap<char, [[i64; 8]; 8]>,
    mvv_lva: [[i64; 12]; 12], // [attacker][victim]
    pv_length: [u32; 64],
    pv_table: Vec<Vec<String>>,
    follow_pv: bool,
    score_pv: bool,
    full_depth_moves: u32,
    reduction_limit: u32,
    random_seed: u32,
    piece_keys: [[u64; 64]; 12],
    enpassant_keys: [u64; 64],
    castle_keys: [u64; 16],
    side_key: u64,
    hash_key: u64,
}


/// TODO: Look into transpositions tables and iterative deepening
#[pymethods]
impl BestMoveFinder {
    #[new]
    fn new(search_depth: u32) -> Self {
        let mut bmf: BestMoveFinder = BestMoveFinder {
            search_depth: search_depth,
            max_depth: 0,
            mate_score: 49000,
            stale_score: 0,
            move_counter: 0,
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
            /*
                ================================
                    Triangular PV table
                --------------------------------
                PV line: e2e4 e7e5 g1f3 b8c6
                ================================

                    0    1    2    3    4    5

                0    m1   m2   m3   m4   m5   m6

                1    0    m2   m3   m4   m5   m6

                2    0    0    m3   m4   m5   m6

                3    0    0    0    m4   m5   m6

                4    0    0    0    0    m5   m6

                5    0    0    0    0    0    m6
            */
            pv_length: [0; 64],
            pv_table: vec![vec![String::with_capacity(4); 64]; 64],
            follow_pv: false,
            score_pv: false,
            // LMR
            full_depth_moves: 4,
            reduction_limit: 3,
            // Zobrist Hashing
            random_seed: 1804289383,
            piece_keys: [[0; 64]; 12],
            enpassant_keys: [0; 64],
            castle_keys: [0; 16],
            side_key: 0,
            hash_key: 0,
        };
        bmf.initRandomKeys();
        bmf
    }


    fn initRandomKeys(&mut self) {
        for piece in [Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK] {
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


    fn setHashKey(&mut self, bitboards: [i64; 13], castle_rights: [bool; 4], whites_turn: bool) {
        let mut final_key: u64 = 0;
        for piece in [Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK] {
            let mut bitboard: i64 = bitboards[piece];
            let mut bitboard_ls1b: i64 = bitboard & !wrap_op!(bitboard, 1, '-'); // selects single 1 bit
            let mut idxs: Vec<u32> = vec![];
            while bitboard_ls1b != 0 {
                let idx: u32 = bitboard_ls1b.leading_zeros();
                idxs.push(idx);
                bitboard &= !bitboard_ls1b;
                bitboard_ls1b = bitboard & !wrap_op!(bitboard, 1, '-');
            }
            idxs.reverse();
            idxs.iter().for_each(|&idx| final_key ^= self.piece_keys[piece][idx as usize]);
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
        self.hash_key = final_key;
    }


    fn searchPosition(&mut self, mm: &mut Moves, bitboards: [i64; 13], castle_rights: [bool; 4], whites_turn: bool) {
        self.pv_length = [0; 64];
        self.pv_table = vec![vec![String::with_capacity(4); 64]; 64];
        self.follow_pv = false; self.score_pv = false;
        let start_time: Instant = Instant::now();

        // iterative deepening
        for current_depth in 1..(self.search_depth+1) {
            // enable PV following
            self.follow_pv = true;
            self.max_depth = current_depth;
            self.negaMaxAlphaBeta(-10000, 10000, mm, bitboards, castle_rights, whites_turn, 0);
            println!("Total moves analyzed: {}, Duration: {:?}", self.move_counter, start_time.elapsed());
            print!("Best Move Sequence: ");
            for depth in 0..(self.pv_length[0]) {
                print!("{:?} ", move_to_algebra!(self.pv_table[0][depth as usize]));
            }
            println!("\n");
            if start_time.elapsed() > Duration::from_secs(3) {
                break
            }
        }
    }


    fn quiescenceSearch(&mut self, mut alpha: i64, beta: i64, mm: &mut Moves, bitboards: [i64; 13], castle_rights: [bool; 4], whites_turn: bool, depth: u32) -> i64 {
        // look deeper for non-quiet moves (attacking)
        self.move_counter += 1;
        let eval: i64 = (if whites_turn {1} else {-1}) * self.evaluateBoard(mm, bitboards, whites_turn);
        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }
        let mut moves: String = mm.getPossibleMoves(bitboards, castle_rights, whites_turn);
        moves = self.sortMoves(mm, &moves, bitboards, whites_turn, depth);
        for i in (0..moves.len()).step_by(4) {
            let bitboards_t: [i64; 13] = mm.getUpdatedBitboards(&moves[i..i+4], bitboards);
            let castle_rights_t: [bool; 4] = mm.getUpdatedCastleRights(&moves[i..i+4], castle_rights, bitboards);
            if mm.isAttackingMove(bitboards, bitboards_t, whites_turn) {
                let score: i64 = -self.quiescenceSearch(-beta, -alpha, mm, bitboards_t, castle_rights_t, !whites_turn, depth+1);
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

    /*
    Positive = better for current recursive player perspective
    alpha = minimum score that the maximizing player is assured of
    beta = maximum score that the minimizing player is assured of
    depth = how deep current iteration is
    */
    fn negaMaxAlphaBeta(&mut self, mut alpha: i64, beta: i64, mm: &mut Moves, bitboards: [i64; 13], castle_rights: [bool; 4], whites_turn: bool, depth: u32) -> i64 {
        let mut found_pv: bool = false;
        // init current depths PV table entry length
        self.pv_length[depth as usize] = depth;
        if depth >= self.max_depth {
            return self.quiescenceSearch(alpha, beta, mm, bitboards, castle_rights, whites_turn, depth+1);
        }
        if depth >= 64 {
            // prevent PV table overflow
            self.move_counter += 1;
            return (if whites_turn {1} else {-1}) * self.evaluateBoard(mm, bitboards, whites_turn);
        }
        self.move_counter += 1;
        let mut best_score: i64 = -self.mate_score;
        let mut moves: String = mm.getPossibleMoves(bitboards, castle_rights, whites_turn);
        if self.follow_pv {
            // now following PV line so enable PV move scoring
            self.enablePVScoring(&moves, depth);
        }
        moves = self.sortMoves(mm, &moves, bitboards, whites_turn, depth);
        let mut moves_searched: u32 = 0;
        let mut valid_move_found: bool = false;
        for i in (0..moves.len()).step_by(4) {
            let bitboards_t: [i64; 13] = mm.getUpdatedBitboards(&moves[i..i+4], bitboards);
            let castle_rights_t: [bool; 4] = mm.getUpdatedCastleRights(&moves[i..i+4], castle_rights, bitboards);
            valid_move_found = true;
            let mut score: i64;

            // on PV node hit
            if found_pv {
                /*
                Once you've found a move with a score that is between alpha and beta,
                the rest of the moves are searched with the goal of proving that they are all bad.
                It's possible to do this a bit faster than a search that worries that one
                of the remaining moves might be good.
                */
                score = -self.negaMaxAlphaBeta(-alpha-1, -alpha, mm, bitboards_t, castle_rights_t, !whites_turn, depth+1);
                /*
                If the algorithm finds out that it was wrong, and that one of the
                subsequent moves was better than the first PV move, it has to search again,
                in the normal alpha-beta manner.  This happens sometimes, and it's a waste of time,
                but generally not often enough to counteract the savings gained from doing the
                "bad move proof" search referred to earlier.
               */
                if (score > alpha) && (score < beta) {
                    score = -self.negaMaxAlphaBeta(-beta, -alpha, mm, bitboards_t, castle_rights_t, !whites_turn, depth+1);
                }
            } else {
                if moves_searched == 0 {
                    // normal alpha beta search (full depth)
                    score = -self.negaMaxAlphaBeta(-beta, -alpha, mm, bitboards_t, castle_rights_t, !whites_turn, depth+1);
                } else {
                    // consider Late Move Reduction (LMR)
                    if moves_searched >= self.full_depth_moves && depth >= self.reduction_limit && !mm.isAttackingMove(bitboards, bitboards_t, whites_turn) && moves[i..i+4].chars().nth(3).unwrap() != 'P' {
                        // search current move with reduced depth
                        score = -self.negaMaxAlphaBeta(-alpha-1, -alpha, mm, bitboards_t, castle_rights_t, !whites_turn, depth+2);
                    } else {
                        score = alpha + 1;
                    }

                    if score > alpha {
                        // found better move during LMR, re-search at normal depth with null window
                        score = -self.negaMaxAlphaBeta(-alpha-1, -alpha, mm, bitboards_t, castle_rights_t, !whites_turn, depth+1);
                        if score > alpha && score < beta {
                            // LMR fails, re-search at full depth and full window
                            score = -self.negaMaxAlphaBeta(-beta, -alpha, mm, bitboards_t, castle_rights_t, !whites_turn, depth+1);
                        }
                    }
                }
            }

            moves_searched += 1;

            if score == self.mate_score {
                score -= depth as i64;
            }
            if score > best_score {
                best_score = score;
                if depth == 0 {
                    println!("Depth: {}, Move: {}, Score: {}", self.max_depth, move_to_algebra!(moves[i..i+4]), score);
                }
            }

            if best_score > alpha {
                alpha = best_score;
                found_pv = true;
                // write PV move to table
                self.pv_table[depth as usize][depth as usize] = moves[i..i+4].to_string();
                // loop over the next depth in table to propagate next moves up a row
                for next_depth in (depth+1)..self.pv_length[(depth+1) as usize] {
                    // copy move from deeper depth into a current depth's line
                    self.pv_table[depth as usize][next_depth as usize] = self.pv_table[(depth+1) as usize][next_depth as usize].clone();
                }
                // adjust PV table length to account for propagated values in current depth row
                self.pv_length[depth as usize] = self.pv_length[(depth+1) as usize];
            }
            if alpha >= beta {
                break;
            }
        }
        if !valid_move_found {
            if mm.isKingAttacked(bitboards, whites_turn) {
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
                score += self.piece_scores[&'Q'];// + self.piece_position_scores[&'Q'][i / 8][i % 8];
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
                score -= self.piece_scores[&'Q'];// + self.piece_position_scores[&'Q'][7 - (i / 8)][i % 8];
            }
            if usgn_r_shift!(bitboards[Piece::BK], shift) & 1 == 1 {
                score -= self.piece_scores[&'K'] + self.piece_position_scores[&'K'][7 - (i / 8)][i % 8];
            }
        }
        score
    }


    fn enablePVScoring(&mut self, moves: &str, depth: u32) {
        // disable PV following
        self.follow_pv = false;
        for i in (0..moves.len()).step_by(4) {
            // make sure to hit a PV move
            if self.pv_table[0][depth as usize] == moves[i..i+4] {
                // enable move scoring
                self.score_pv = true;
                // enable further PV following
                self.follow_pv = true;
            }
        }
    }


    fn scoreMove(&mut self, mm: &mut Moves, bitboards: [i64; 13], move_str: &str, whites_turn: bool, depth: u32) -> i64 {
        if self.score_pv {
            if self.pv_table[0][depth as usize] == move_str {
                self.score_pv = false;
                // println!("Current PV Move: {}, Depth: {}", move_to_algebra!(move_str), depth);
                // give PV move the highest score to search it first
                return 20000;
            }
        }
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
            return self.mvv_lva[attacker][victim] + 10000;
        }
        0
    }


    /*
    Note this function excludes invalid moves

    Function Optimization Details:

    1. Pre-allocation: use 'with_capacity'
        - creates a heap item with given capacity but with zero length
        - until capacity is reached, push() calls won't reallocate, making push() essentially free

    2. In-place Sorting: use 'sort_unstable_by'
        - sort is unstable (i.e., may reorder equal elements)
        - in-place (i.e., does not allocate)
        - O(n * log(n)) worst-case

    3. Resulted in 1.24 times speedup on average (24% faster) than no pre-allocation or in-place sorting
    */
    fn sortMoves(&mut self, mm: &mut Moves, moves: &str, bitboards: [i64; 13], whites_turn: bool, depth: u32) -> String {
        let mut move_scores: Vec<(i64, &str)> = Vec::with_capacity(moves.len() / 4);
        for i in (0..moves.len()).step_by(4) {
            let move_slice: &str = &moves[i..i + 4];
            let bitboards_t: [i64; 13] = mm.getUpdatedBitboards(move_slice, bitboards);
            if mm.isValidMove(bitboards_t, whites_turn) {
                move_scores.push((self.scoreMove(mm, bitboards, move_slice, whites_turn, depth), move_slice));
            }
        }
        move_scores.sort_unstable_by(|a: &(i64, &str), b: &(i64, &str)| b.0.cmp(&a.0));
        let mut sorted_moves: String = String::with_capacity(moves.len());
        for (_, m) in move_scores {
            sorted_moves.push_str(m);
        }
        sorted_moves
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
        let mut bmf: BestMoveFinder = BestMoveFinder::new(2);
        let moves: String = m.getPossibleMoves(gs.bitboards, gs.castle_rights, gs.whites_turn);
        let mut actual_scores: Vec<i64> = vec![10105, 10105, 10303, 10101, 10201, 10104, 10104, 10104];
        for i in (0..moves.len()).step_by(4) {
            let bitboards_t: [i64; 13] = m.getUpdatedBitboards(&moves[i..i+4], gs.bitboards);
            if m.isValidMove(bitboards_t, gs.whites_turn) {
                let score = bmf.scoreMove(&mut m, gs.bitboards, &moves[i..i+4], gs.whites_turn, 0);
                if score != 0 {
                    assert!(score == actual_scores.remove(0));
                }
            }
        }
    }

    #[test]
    fn sort_moves_test() {
        let mut gs = GameState::new();
        gs.importFEN(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 "));
        let mut m: Moves = Moves::new();
        let mut bmf: BestMoveFinder = BestMoveFinder::new(2);
        let moves: String = m.getPossibleMoves(gs.bitboards, gs.castle_rights, gs.whites_turn);
        let sorted_moves: String = bmf.sortMoves(&mut m, &moves, gs.bitboards, gs.whites_turn, 0);
        let mut score: i64 = i64::MAX;
        for i in (0..sorted_moves.len()).step_by(4) {
            let current_score: i64 = bmf.scoreMove(&mut m, gs.bitboards, &sorted_moves[i..i+4], gs.whites_turn, 0);
            assert!(current_score <= score);
            score = current_score;
        }
    }

    #[test]
    fn basic_test() {
        let mut gs = GameState::new();
        gs.importFEN(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ")); // tricky
        // gs.importFEN(String::from("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ")); // cmk
        let mut m: Moves = Moves::new();
        let mut bmf: BestMoveFinder = BestMoveFinder::new(7);
        // bmf.searchPosition(&mut m, gs.bitboards, gs.castle_rights, gs.whites_turn);
        bmf.setHashKey(gs.bitboards, gs.castle_rights, gs.whites_turn);
        println!("hash key: {:x}", bmf.hash_key);
        panic!();
    }
}
