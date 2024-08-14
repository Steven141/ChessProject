//! Module with different ways to find the best next move


use pyo3::prelude::*;
use std::collections::HashMap;
use std::time::{
    Duration,
    Instant,
};
use crate::{
    moves::Moves,
    piece::Piece,
    zobrist::Zobrist,
    trans_table::*,
};


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct BestMoveFinder {
    search_depth: u32,
    max_depth: u32,
    mate_score: i32,
    stale_score: i32,
    move_counter: u32,
    piece_scores: HashMap<char, i32>,
    piece_position_scores: HashMap<char, [[i32; 8]; 8]>,
    double_pawn_penalty: i32,
    isolated_pawn_penalty: i32,
    passed_pawn_bonus: [i32; 8],
    semi_open_file_score: i32,
    open_file_score: i32,
    king_blocking_bonus: i32,
    mvv_lva: [[i32; 12]; 12], // [attacker][victim]
    killer_moves: Vec<Vec<String>>, // [id][ply]
    history_moves: [[i32; 64]; 12], // [piece][square]
    pv_length: [u32; 64],
    pv_table: Vec<Vec<String>>,
    follow_pv: bool,
    score_pv: bool,
    full_depth_moves: u32,
    reduction_limit: u32,
    repetition_table: [u64; 1000],
    repetition_idx: usize,
}


#[pymethods]
impl BestMoveFinder {
    #[new]
    fn new(search_depth: u32) -> Self {
        BestMoveFinder {
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
                    [0,  10,   5,  10,  10,   5,  10,   0],
                    [0,  20,   5,   5,   5,   5,  20,   0],
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
                    [15,  10,  10,  20,  20,  10,  10,  15],
                    [10,   5,  10,  20,  20,   5,   5,  10],
                    [ 5,   0,   0,   5,   5,   0,   0,   5],
                    [ 0,   0,   0, -10, -10,   0,   0,   0],
                    [ 0,   0,   0,   0,   0,   0,   0,   0],
                ]),
            ]),
            double_pawn_penalty: -10,
            isolated_pawn_penalty: -10,
            passed_pawn_bonus: [0, 10, 30, 50, 75, 100, 150, 200],
            semi_open_file_score: 10,
            open_file_score: 15,
            king_blocking_bonus: 5,
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
            // killer & history moves
            killer_moves: vec![vec![String::with_capacity(4); 64]; 2],
            history_moves: [[0; 64]; 12],
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
            // Repetition Detection
            repetition_table: [0; 1000],
            repetition_idx: 0,
        }
    }


    fn isRepetition(&self, hash_key: u64) -> bool {
        for i in 0..self.repetition_idx {
            if self.repetition_table[i] == hash_key {
                return true;
            }
        }
        false
    }


    fn searchPosition(&mut self, mm: &mut Moves, z: &mut Zobrist, tt: &mut TransTable, bitboards: [u64; 13], castle_rights: [bool; 4], hash_key: u64, whites_turn: bool) {
        self.pv_length = [0; 64];
        self.pv_table = vec![vec![String::with_capacity(4); 64]; 64];
        self.follow_pv = false; self.score_pv = false;
        let start_time: Instant = Instant::now();
        self.repetition_idx += 1;
        self.move_counter = 0;

        // iterative deepening
        for current_depth in 1..=self.search_depth {
            // enable PV following
            self.follow_pv = true;
            self.max_depth = current_depth;
            let score: i32 = self.negaMaxAlphaBeta(-50000, 50000, mm, z, tt, bitboards, castle_rights, hash_key, whites_turn, 0);
            if score >= -49000 && score < -48000 {
                println!("Depth: {}, Move: {}, Score: {}, Mate in {}", self.max_depth, move_to_algebra!(self.pv_table[0][0]), score, (score + 49000) / 2 + 1);
            } else if score <= 49000 && score > 48000 {
                println!("Depth: {}, Move: {}, Score: {}, Mate in {}", self.max_depth, move_to_algebra!(self.pv_table[0][0]), score, (49000 - score) / 2 + 1);
            } else {
                println!("Depth: {}, Move: {}, Score: {}", self.max_depth, move_to_algebra!(self.pv_table[0][0]), score);
            }
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


    fn quiescenceSearch(&mut self, mut alpha: i32, beta: i32, mm: &mut Moves, z: &mut Zobrist, bitboards: [u64; 13], castle_rights: [bool; 4], hash_key: u64, whites_turn: bool, depth: u32) -> i32 {
        // look deeper for non-quiet moves (attacking)
        self.move_counter += 1;
        let eval: i32 = (if whites_turn {1} else {-1}) * self.evaluateBoard(mm, bitboards);
        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }
        let mut moves: String = mm.getPossibleMoves(bitboards, castle_rights, whites_turn);
        moves = self.sortMoves(mm, z, &moves, bitboards, hash_key, whites_turn, depth);
        for i in (0..moves.len()).step_by(4) {
            let (bitboards_t, hash_key_t) = mm.getUpdatedBitboards(z, &moves[i..i+4], bitboards, hash_key, whites_turn);
            let (castle_rights_t, hash_key_t) = mm.getUpdatedCastleRights(z, &moves[i..i+4], castle_rights, bitboards, hash_key_t);
            if mm.isAttackingMove(bitboards, bitboards_t, whites_turn) {
                self.repetition_idx += 1;
                self.repetition_table[self.repetition_idx] = hash_key;
                let score: i32 = -self.quiescenceSearch(-beta, -alpha, mm, z, bitboards_t, castle_rights_t, hash_key_t, !whites_turn, depth+1);
                self.repetition_idx -= 1;
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
    fn negaMaxAlphaBeta(&mut self, mut alpha: i32, beta: i32, mm: &mut Moves, z: &mut Zobrist, tt: &mut TransTable, bitboards: [u64; 13], castle_rights: [bool; 4], hash_key: u64, whites_turn: bool, depth: u32) -> i32 {
        if depth > 0 && self.isRepetition(hash_key) {
            return 0; // draw score
        }
        let table_score: i32 = tt.readEntry(alpha, beta, hash_key, self.max_depth as i32 - depth as i32, depth);
        let is_pv_node: bool = beta - alpha > 1;
        if table_score != TransTable::NO_HASH_ENTRY && !is_pv_node {
            return table_score; // board state searched before
        }
        let mut hash_flag: HashFlag = HashFlag::Alpha;
        // init current depths PV table entry length
        self.pv_length[depth as usize] = depth;
        if depth >= self.max_depth {
            return self.quiescenceSearch(alpha, beta, mm, z, bitboards, castle_rights, hash_key, whites_turn, depth+1);
        }
        if depth >= 64 {
            // prevent PV table overflow
            self.move_counter += 1;
            return (if whites_turn {1} else {-1}) * self.evaluateBoard(mm, bitboards);
        }
        self.move_counter += 1;


        // null move pruning: problems arise from wrong mating depth
        // if self.max_depth - depth >= 3 && !mm.isKingAttacked(bitboards, whites_turn) && depth > 0 {
        //     self.repetition_idx += 1;
        //     self.repetition_table[self.repetition_idx] = hash_key;

        //     let mut hash_key_t: u64 = hash_key;
        //     let mut bitboards_t: [u64; 13] = bitboards;
        //     if bitboards[Piece::EP] != 0 {
        //         let col: usize = bitboards[Piece::EP].leading_zeros() as usize;
        //         let row: usize = if whites_turn {2} else {5};
        //         hash_key_t ^= z.enpassant_keys[row * 8 + col];
        //     }
        //     bitboards_t[Piece::EP] = 0;
        //     hash_key_t ^= z.side_key;

        //     // search moves with reduced depth to find beta cutoff
        //     let score: i32 = -self.negaMaxAlphaBeta(-beta, -beta+1, mm, z, tt, bitboards_t, castle_rights, hash_key_t, !whites_turn, depth+1+2);

        //     self.repetition_idx -= 1;
        //     if score >= beta {
        //         return beta;
        //     }
        // }



        let mut best_score: i32 = -self.mate_score;
        let mut moves: String = mm.getPossibleMoves(bitboards, castle_rights, whites_turn);
        if self.follow_pv {
            // now following PV line so enable PV move scoring
            self.enablePVScoring(&moves, depth);
        }
        moves = self.sortMoves(mm, z, &moves, bitboards, hash_key, whites_turn, depth);
        let mut moves_searched: u32 = 0;
        let mut valid_move_found: bool = false;
        for i in (0..moves.len()).step_by(4) {
            let (bitboards_t, hash_key_t) = mm.getUpdatedBitboards(z, &moves[i..i+4], bitboards, hash_key, whites_turn);
            let (castle_rights_t, hash_key_t) = mm.getUpdatedCastleRights(z, &moves[i..i+4], castle_rights, bitboards, hash_key_t);
            self.repetition_idx += 1;
            self.repetition_table[self.repetition_idx] = hash_key;
            valid_move_found = true;
            let mut score: i32;

            if moves_searched == 0 {
                // normal alpha beta search (full depth)
                score = -self.negaMaxAlphaBeta(-beta, -alpha, mm, z, tt, bitboards_t, castle_rights_t, hash_key_t, !whites_turn, depth+1);
            } else {
                // consider Late Move Reduction (LMR)
                if moves_searched >= self.full_depth_moves && depth >= self.reduction_limit && !mm.isAttackingMove(bitboards, bitboards_t, whites_turn) && moves[i..i+4].chars().nth(3).unwrap() != 'P' {
                    // search current move with reduced depth
                    score = -self.negaMaxAlphaBeta(-alpha-1, -alpha, mm, z, tt, bitboards_t, castle_rights_t, hash_key_t, !whites_turn, depth+2);
                } else {
                    score = alpha + 1; // ensure full-depth search is done if no LMR
                }

                if score > alpha {
                    /*
                    Once you've found a move with a score that is between alpha and beta,
                    the rest of the moves are searched with the goal of proving that they are all bad.
                    It's possible to do this a bit faster than a search that worries that one
                    of the remaining moves might be good.
                    */
                    score = -self.negaMaxAlphaBeta(-alpha-1, -alpha, mm, z, tt, bitboards_t, castle_rights_t, hash_key_t, !whites_turn, depth+1);
                    /*
                    If the algorithm finds out that it was wrong, and that one of the
                    subsequent moves was better than the first PV move, it has to search again,
                    in the normal alpha-beta manner.  This happens sometimes, and it's a waste of time,
                    but generally not often enough to counteract the savings gained from doing the
                    "bad move proof" search referred to earlier.
                    */
                    if score > alpha && score < beta {
                        // LMR fails, re-search at full depth and full window
                        score = -self.negaMaxAlphaBeta(-beta, -alpha, mm, z, tt, bitboards_t, castle_rights_t, hash_key_t, !whites_turn, depth+1);
                    }
                }
            }

            self.repetition_idx -= 1;
            moves_searched += 1;

            if score > best_score {
                best_score = score;
            }

            if best_score > alpha {
                hash_flag = HashFlag::Exact;

                // write history move if quiet move
                let (attacker, victim) = get_move_pieces!(bitboards, moves[i..i+4]);
                if victim == Piece::EP {
                    let (_, _, r2, c2) = move_to_u32s!(moves[i..i+4]);
                    self.history_moves[attacker][(r2 * 8 + c2) as usize] += (self.max_depth - depth) as i32;
                }

                alpha = best_score;
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
                tt.writeEntry(beta, hash_key, self.max_depth - depth, depth, HashFlag::Beta as i32);

                // write killer moves if quiet move
                if get_move_pieces!(bitboards, moves[i..i+4]).1 == Piece::EP {
                    self.killer_moves[1][depth as usize] = self.killer_moves[0][depth as usize].clone();
                    self.killer_moves[0][depth as usize] = moves[i..i+4].to_string();
                }

                return beta;
            }
        }
        if !valid_move_found {
            if mm.isKingAttacked(bitboards, whites_turn) {
                mm.checkmate = true;
                return -self.mate_score + depth as i32;
            } else {
                mm.stalemate = true;
                return self.stale_score;
            }
        } else {
            mm.checkmate = false;
            mm.stalemate = false;
        }
        tt.writeEntry(alpha, hash_key, self.max_depth - depth, depth, hash_flag as i32);
        alpha
    }


    fn evaluateBoard(&self, mm: &mut Moves, bitboards: [u64; 13]) -> i32 {
        // TODO better way to do doubled pawns with shifting
        let mut score: i32 = 0;
        for i in 0..64 {
            if get_bit!(bitboards[Piece::WP], i) == 1 {
                score += self.piece_scores[&'P'] + self.piece_position_scores[&'P'][i / 8][i % 8];
                // double/isolated/past pawn scoring
                let double_pawns: i32 = (bitboards[Piece::WP] & mm.masks.file_masks[i % 8]).count_ones() as i32;
                if double_pawns > 1 {
                    score += double_pawns * self.double_pawn_penalty;
                }
                if bitboards[Piece::WP] & mm.masks.isolated_masks[i % 8] == 0 {
                    score += self.isolated_pawn_penalty;
                }
                if bitboards[Piece::BP] & mm.masks.w_passed_pawn_masks[i] == 0 {
                    score += self.passed_pawn_bonus[7 - (i / 8)];
                }
            }
            if get_bit!(bitboards[Piece::WN], i) == 1 {
                score += self.piece_scores[&'N'] + self.piece_position_scores[&'N'][i / 8][i % 8];
            }
            if get_bit!(bitboards[Piece::WB], i) == 1 {
                score += self.piece_scores[&'B'] + self.piece_position_scores[&'B'][i / 8][i % 8];
                // bishop mobility scoring
                mm.masks.occupied = or_array_elems!(Piece::allPieces(), bitboards);
                score += (mm.possibleDiagAndAntiDiagMoves(bitboards[Piece::WB].leading_zeros() as usize)
                    & !or_array_elems!(Piece::whitePiecesWithEnemyKing(), bitboards)).count_ones() as i32; // avoid illegal bK capture
            }
            if get_bit!(bitboards[Piece::WR], i) == 1 {
                score += self.piece_scores[&'R'] + self.piece_position_scores[&'R'][i / 8][i % 8];
                // rook semi/open file scoring
                if bitboards[Piece::WP] & mm.masks.file_masks[i % 8] == 0 {
                    score += self.semi_open_file_score;
                }
                if or_array_elems!([Piece::WP, Piece::BP], bitboards) & mm.masks.file_masks[i % 8] == 0 {
                    score += self.open_file_score;
                }
            }
            if get_bit!(bitboards[Piece::WQ], i) == 1 {
                score += self.piece_scores[&'Q'];
                // queen mobility scoring
                mm.masks.occupied = or_array_elems!(Piece::allPieces(), bitboards);
                score += ((mm.possibleDiagAndAntiDiagMoves(bitboards[Piece::WQ].leading_zeros() as usize)
                    | (mm.possibleHAndVMoves(bitboards[Piece::WQ].leading_zeros() as usize)))
                    & !or_array_elems!(Piece::whitePiecesWithEnemyKing(), bitboards)).count_ones() as i32; // avoid illegal bK capture
            }
            if get_bit!(bitboards[Piece::WK], i) == 1 {
                score += self.piece_scores[&'K'] + self.piece_position_scores[&'K'][i / 8][i % 8];
                // king semi/open file scoring
                if bitboards[Piece::WP] & mm.masks.file_masks[i % 8] == 0 {
                    score -= self.semi_open_file_score;
                }
                if or_array_elems!([Piece::WP, Piece::BP], bitboards) & mm.masks.file_masks[i % 8] == 0 {
                    score -= self.open_file_score;
                }
                // king protection scoring
                let king_idx: usize = bitboards[Piece::WK].leading_zeros() as usize;
                let king_span_c7_idx: usize = 10;
                // allign the king_span_c7 mask
                let mut moves: u64;
                if king_idx <= king_span_c7_idx {
                    moves = mm.masks.king_span_c7 << (king_span_c7_idx - king_idx);
                } else {
                    moves = mm.masks.king_span_c7 >> (king_idx - king_span_c7_idx);
                }
                // remove moves sliding off board or allied pieces
                if king_idx % 8 < 4 {
                    pop_bits!(moves, mm.masks.file_gh);
                } else {
                    pop_bits!(moves, mm.masks.file_ab);
                }
                score += self.king_blocking_bonus * (moves & or_array_elems!(Piece::whitePiecesNoKing(), bitboards)).count_ones() as i32;
            }
            if get_bit!(bitboards[Piece::BP], i) == 1 {
                score -= self.piece_scores[&'P'] + self.piece_position_scores[&'P'][7 - (i / 8)][i % 8];
                // double/isolated/past pawn scoring
                let double_pawns: i32 = (bitboards[Piece::BP] & mm.masks.file_masks[i % 8]).count_ones() as i32;
                if double_pawns > 1 {
                    score -= double_pawns * self.double_pawn_penalty;
                }
                if bitboards[Piece::BP] & mm.masks.isolated_masks[i % 8] == 0 {
                    score -= self.isolated_pawn_penalty;
                }
                if bitboards[Piece::WP] & mm.masks.b_passed_pawn_masks[i] == 0 {
                    score -= self.passed_pawn_bonus[i / 8];
                }
            }
            if get_bit!(bitboards[Piece::BN], i) == 1 {
                score -= self.piece_scores[&'N'] + self.piece_position_scores[&'N'][7 - (i / 8)][i % 8];
            }
            if get_bit!(bitboards[Piece::BB], i) == 1 {
                score -= self.piece_scores[&'B'] + self.piece_position_scores[&'B'][7 - (i / 8)][i % 8];
                // bishop mobility scoring
                mm.masks.occupied = or_array_elems!(Piece::allPieces(), bitboards);
                score -= (mm.possibleDiagAndAntiDiagMoves(bitboards[Piece::BB].leading_zeros() as usize)
                    & !or_array_elems!(Piece::blackPiecesWithEnemyKing(), bitboards)).count_ones() as i32; // avoid illegal bK capture
            }
            if get_bit!(bitboards[Piece::BR], i) == 1 {
                score -= self.piece_scores[&'R'] + self.piece_position_scores[&'R'][7 - (i / 8)][i % 8];
                // rook semi/open file scoring
                if bitboards[Piece::BP] & mm.masks.file_masks[i % 8] == 0 {
                    score -= self.semi_open_file_score;
                }
                if or_array_elems!([Piece::WP, Piece::BP], bitboards) & mm.masks.file_masks[i % 8] == 0 {
                    score -= self.open_file_score;
                }
            }
            if get_bit!(bitboards[Piece::BQ], i) == 1 {
                score -= self.piece_scores[&'Q'];
                // queen mobility scoring
                mm.masks.occupied = or_array_elems!(Piece::allPieces(), bitboards);
                score -= ((mm.possibleDiagAndAntiDiagMoves(bitboards[Piece::BQ].leading_zeros() as usize)
                    | (mm.possibleHAndVMoves(bitboards[Piece::BQ].leading_zeros() as usize)))
                    & !or_array_elems!(Piece::blackPiecesWithEnemyKing(), bitboards)).count_ones() as i32; // avoid illegal bK capture
            }
            if get_bit!(bitboards[Piece::BK], i) == 1 {
                score -= self.piece_scores[&'K'] + self.piece_position_scores[&'K'][7 - (i / 8)][i % 8];
                // king semi/open file scoring
                if bitboards[Piece::BP] & mm.masks.file_masks[i % 8] == 0 {
                    score += self.semi_open_file_score;
                }
                if or_array_elems!([Piece::WP, Piece::BP], bitboards) & mm.masks.file_masks[i % 8] == 0 {
                    score += self.open_file_score;
                }
                // king protection scoring
                let king_idx: usize = bitboards[Piece::BK].leading_zeros() as usize;
                let king_span_c7_idx: usize = 10;
                // allign the king_span_c7 mask
                let mut moves: u64;
                if king_idx <= king_span_c7_idx {
                    moves = mm.masks.king_span_c7 << (king_span_c7_idx - king_idx);
                } else {
                    moves = mm.masks.king_span_c7 >> (king_idx - king_span_c7_idx);
                }
                // remove moves sliding off board or allied pieces
                if king_idx % 8 < 4 {
                    pop_bits!(moves, mm.masks.file_gh);
                } else {
                    pop_bits!(moves, mm.masks.file_ab);
                }
                score -= self.king_blocking_bonus * (moves & or_array_elems!(Piece::blackPiecesNoKing(), bitboards)).count_ones() as i32;
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


    fn scoreMove(&mut self, bitboards: [u64; 13], move_str: &str, depth: u32) -> i32 {
        if self.score_pv {
            if self.pv_table[0][depth as usize] == move_str {
                self.score_pv = false;
                return 20000; // give PV move the highest score to search it first
            }
        }
        let (_, _, r2, c2) = move_to_u32s!(move_str);
        let (attacker, victim) = get_move_pieces!(bitboards, move_str);
        if victim != Piece::EP { // attacking move
            return self.mvv_lva[attacker][victim] + 10000;
        } else { // quiet move
            if self.killer_moves[0][depth as usize] == move_str {
                return 9000;
            } else if self.killer_moves[1][depth as usize] == move_str {
                return 8000;
            } else {
                return self.history_moves[attacker][(r2 * 8 + c2) as usize];
            }
        }
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
    fn sortMoves(&mut self, mm: &mut Moves, z: &mut Zobrist, moves: &str, bitboards: [u64; 13], hash_key: u64, whites_turn: bool, depth: u32) -> String {
        let mut move_scores: Vec<(i32, &str)> = Vec::with_capacity(moves.len() / 4);
        for i in (0..moves.len()).step_by(4) {
            let move_slice: &str = &moves[i..i + 4];
            let (bitboards_t, _) = mm.getUpdatedBitboards(z, move_slice, bitboards, hash_key, whites_turn);
            if mm.isValidMove(bitboards_t, whites_turn) {
                move_scores.push((self.scoreMove(bitboards, move_slice, depth), move_slice));
            }
        }
        move_scores.sort_unstable_by(|a: &(i32, &str), b: &(i32, &str)| b.0.cmp(&a.0));
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
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        gs.importFEN(&m.masks, &mut z, String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 "));
        let mut bmf: BestMoveFinder = BestMoveFinder::new(2);
        let moves: String = m.getPossibleMoves(gs.bitboards, gs.castle_rights, gs.whites_turn);
        let mut actual_scores: Vec<i32> = vec![10105, 10105, 10303, 10101, 10201, 10104, 10104, 10104];
        for i in (0..moves.len()).step_by(4) {
            let (bitboards_t, _) = m.getUpdatedBitboards(&mut z, &moves[i..i+4], gs.bitboards, gs.hash_key, gs.whites_turn);
            if m.isValidMove(bitboards_t, gs.whites_turn) {
                let score = bmf.scoreMove(gs.bitboards, &moves[i..i+4], 0);
                if score != 0 {
                    assert!(score == actual_scores.remove(0));
                }
            }
        }
    }

    #[test]
    fn sort_moves_test() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        gs.importFEN(&m.masks, &mut z, String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 "));
        let mut bmf: BestMoveFinder = BestMoveFinder::new(2);
        let moves: String = m.getPossibleMoves(gs.bitboards, gs.castle_rights, gs.whites_turn);
        let sorted_moves: String = bmf.sortMoves(&mut m, &mut z, &moves, gs.bitboards, gs.hash_key, gs.whites_turn, 0);
        let mut score: i32 = i32::MAX;
        for i in (0..sorted_moves.len()).step_by(4) {
            let current_score: i32 = bmf.scoreMove(gs.bitboards, &sorted_moves[i..i+4], 0);
            assert!(current_score <= score);
            score = current_score;
        }
    }
}
