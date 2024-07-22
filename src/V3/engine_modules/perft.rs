//! Module used for performance testing


use pyo3::prelude::*;
use crate::{
    castle_rights::CastleRights,
    moves::Moves,
    piece::Piece,
    zobrist::Zobrist,
};


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct Perft {
    max_depth: u32,
    move_counter: u32,
    pub total_move_counter: u32,
}


#[pymethods]
impl Perft {
    #[new]
    pub fn new(max_depth: u32) -> Self {
        Perft {
            max_depth: max_depth,
            move_counter: 0,
            total_move_counter: 0,
        }
    }


    fn perft(&mut self, mm: &mut Moves, z: &mut Zobrist, bitboards: [i64; 13], castle_rights: [bool; 4], hash_key: u64, whites_turn: bool, depth: u32) {
        if depth < self.max_depth {
            let moves: String = mm.getPossibleMoves(bitboards, castle_rights, whites_turn);
            for i in (0..moves.len()).step_by(4) {
                let bitboards_t: [i64; 13] = mm.getUpdatedBitboards(&moves[i..i+4], bitboards);
                let castle_rights_t: [bool; 4] = mm.getUpdatedCastleRights(&moves[i..i+4], castle_rights, bitboards);



                let mut hash_key_t: u64 = hash_key;
                let scratch_hash: u64 = z.generateHashKey(bitboards_t, castle_rights_t, !whites_turn); // build updated position hash key after move

                hash_key_t ^= z.side_key; // hash side

                let start_shift: u32; let end_shift: u32;
                let start_bitboard: i64; let end_bitboard: i64;
                if moves[i..i+4].chars().nth(3).unwrap().is_numeric() { // regular move
                    let (r1, c1, r2, c2) = move_to_u32s!(moves[i..i+4]);
                    start_shift = 64 - 1 - (r1 * 8 + c1);
                    end_shift = 64 - 1 - (r2 * 8 + c2);
                    for piece in [Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK] {
                        if usgn_r_shift!(bitboards[piece], start_shift) & 1 == 1 {
                            hash_key_t ^= z.piece_keys[piece][(r1 * 8 + c1) as usize] // remove source piece from hash
                        }
                        if usgn_r_shift!(bitboards_t[piece], end_shift) & 1 == 1 {
                            hash_key_t ^= z.piece_keys[piece][(r2 * 8 + c2) as usize] // add target piece to hash
                        }
                        if usgn_r_shift!(bitboards[piece], end_shift) & 1 == 1 {
                            hash_key_t ^= z.piece_keys[piece][(r2 * 8 + c2) as usize] // remove taken piece from hash
                        }
                    }
                } else if moves[i..i+4].chars().nth(3).unwrap() == 'P' { // pawn promo
                    let (c1, c2, _, _) = move_to_u32s!(moves[i..i+4]);
                    let (r1, r2) = if whites_turn {(1, 0)} else {(6, 7)};
                    let piece: Piece = if whites_turn {Piece::WP} else {Piece::BP};
                    let promo_piece: Piece;
                    match moves[i..i+4].chars().nth(2).unwrap() {
                        'Q' => promo_piece = Piece::WQ,
                        'R' => promo_piece = Piece::WR,
                        'B' => promo_piece = Piece::WB,
                        'N' => promo_piece = Piece::WN,
                        'q' => promo_piece = Piece::BQ,
                        'r' => promo_piece = Piece::BR,
                        'b' => promo_piece = Piece::BB,
                        'n' => promo_piece = Piece::BN,
                        _ => panic!("INVALID PROMO TYPE"),
                    }
                    if moves[i..i+4].chars().nth(2).unwrap().is_uppercase() { // white promo
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
                    hash_key_t ^= z.piece_keys[piece][(r1 * 8 + c1) as usize]; // remove source piece from hash
                    hash_key_t ^= z.piece_keys[promo_piece][(r2 * 8 + c2) as usize]; // add promoted piece to hash
                    for piece in [Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK] {
                        if usgn_r_shift!(bitboards[piece], end_shift) & 1 == 1 {
                            hash_key_t ^= z.piece_keys[piece][(r2 * 8 + c2) as usize] // remove taken piece from hash
                        }
                    }
                } else if moves[i..i+4].chars().nth(3).unwrap() == 'E' { // enpassant
                    let (c1, c2, _, _) = move_to_u32s!(moves[i..i+4]);
                    let (r1, r2) = if whites_turn {(3, 2)} else {(4, 5)};
                    if moves[i..i+4].chars().nth(2).unwrap() == 'w' { // white
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
                    for piece in [Piece::WP, Piece::BP] {
                        if usgn_r_shift!(bitboards[piece], start_shift) & 1 == 1 {
                            hash_key_t ^= z.piece_keys[piece][(r1 * 8 + c1) as usize] // remove source piece from hash
                        }
                        if usgn_r_shift!(bitboards_t[piece], end_shift) & 1 == 1 {
                            hash_key_t ^= z.piece_keys[piece][(r2 * 8 + c2) as usize] // add target piece to hash
                        }
                        if usgn_r_shift!(bitboards[piece], if whites_turn {end_shift-8} else {end_shift+8}) & 1 == 1 {
                            hash_key_t ^= z.piece_keys[piece][(r1 * 8 + c2) as usize] // remove taken piece from hash
                        }
                    }
                } else {
                    panic!("INVALID MOVE TYPE");
                }
                // remove current enpassant status from hash
                if bitboards[Piece::EP] != 0 {
                    let col: usize = bitboards[Piece::EP].leading_zeros() as usize;
                    let row: usize = if whites_turn {2} else {5};
                    hash_key_t ^= z.enpassant_keys[row * 8 + col];
                }
                // add next move enpassant status to hash
                if bitboards_t[Piece::EP] != 0 {
                    let col: usize = bitboards_t[Piece::EP].leading_zeros() as usize;
                    let row: usize = if !whites_turn {2} else {5};
                    hash_key_t ^= z.enpassant_keys[row * 8 + col];
                }
                // remove current castle rights from hash
                hash_key_t ^= z.castle_keys[
                    ((castle_rights[CastleRights::CBQ] as usize) << 3)
                    | ((castle_rights[CastleRights::CBK] as usize) << 2)
                    | ((castle_rights[CastleRights::CWQ] as usize) << 1)
                    | (castle_rights[CastleRights::CWK] as usize)
                ];
                // add next moves castle rights to hash
                hash_key_t ^= z.castle_keys[
                    ((castle_rights_t[CastleRights::CBQ] as usize) << 3)
                    | ((castle_rights_t[CastleRights::CBK] as usize) << 2)
                    | ((castle_rights_t[CastleRights::CWQ] as usize) << 1)
                    | (castle_rights_t[CastleRights::CWK] as usize)
                ];
                // hash castle moves (rook move)
                if ((usgn_r_shift!(bitboards[Piece::WK], start_shift) & 1 == 1) || (usgn_r_shift!(bitboards[Piece::BK], start_shift) & 1 == 1)) && ((&moves[i..i+4] == "0402") || (&moves[i..i+4] == "0406") || (&moves[i..i+4] == "7472") || (&moves[i..i+4] == "7476")) {
                    if whites_turn { // white
                        match &moves[i..i+4] {
                            "7476" => { // king side
                                hash_key_t ^= z.piece_keys[Piece::WR][63 - mm.castle_rooks[3]];
                                hash_key_t ^= z.piece_keys[Piece::WR][63 - (mm.castle_rooks[3] + 2)];
                            },
                            "7472" => { // queen side
                                hash_key_t ^= z.piece_keys[Piece::WR][63 - mm.castle_rooks[2]];
                                hash_key_t ^= z.piece_keys[Piece::WR][63 - (mm.castle_rooks[2] - 3)];
                            },
                            _ => (),
                        }
                    } else { // black
                        match &moves[i..i+4] {
                            "0406" => { // king side
                                hash_key_t ^= z.piece_keys[Piece::BR][63 - mm.castle_rooks[1]];
                                hash_key_t ^= z.piece_keys[Piece::BR][63 - (mm.castle_rooks[1] + 2)];
                            },
                            "0402" => { // queen side
                                hash_key_t ^= z.piece_keys[Piece::BR][63 - mm.castle_rooks[0]];
                                hash_key_t ^= z.piece_keys[Piece::BR][63 - (mm.castle_rooks[0] - 3)];
                            },
                            _ => (),
                        }
                    }
                }
                if scratch_hash != hash_key_t {
                    println!("move: {}", move_to_algebra!(moves[i..i+4]));
                    println!("hash key should be: {:x}", scratch_hash);
                    println!("iterative hash key: {:x}", hash_key_t);
                    break
                }



                if mm.isValidMove(bitboards_t, whites_turn) {
                    if depth + 1 == self.max_depth { // only count leaf nodes
                        self.move_counter += 1
                    }
                    self.perft(mm, z, bitboards_t, castle_rights_t, hash_key_t, !whites_turn, depth + 1)
                }
            }
        } else if self.move_counter == 0 {
            self.move_counter += 1;
        }
    }


    pub fn perftRoot(&mut self, mm: &mut Moves, z: &mut Zobrist, bitboards: [i64; 13], castle_rights: [bool; 4], hash_key: u64, whites_turn: bool, depth: u32) {
        let moves: String = mm.getPossibleMoves(bitboards, castle_rights, whites_turn);
        for i in (0..moves.len()).step_by(4) {
            let bitboards_t: [i64; 13] = mm.getUpdatedBitboards(&moves[i..i+4], bitboards);
            let castle_rights_t: [bool; 4] = mm.getUpdatedCastleRights(&moves[i..i+4], castle_rights, bitboards);



            let mut hash_key_t: u64 = hash_key;
            let scratch_hash: u64 = z.generateHashKey(bitboards_t, castle_rights_t, !whites_turn); // build updated position hash key after move

            hash_key_t ^= z.side_key; // hash side

            let start_shift: u32; let end_shift: u32;
            let start_bitboard: i64; let end_bitboard: i64;
            if moves[i..i+4].chars().nth(3).unwrap().is_numeric() { // regular move
                let (r1, c1, r2, c2) = move_to_u32s!(moves[i..i+4]);
                start_shift = 64 - 1 - (r1 * 8 + c1);
                end_shift = 64 - 1 - (r2 * 8 + c2);
                for piece in [Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK] {
                    if usgn_r_shift!(bitboards[piece], start_shift) & 1 == 1 {
                        hash_key_t ^= z.piece_keys[piece][(r1 * 8 + c1) as usize] // remove source piece from hash
                    }
                    if usgn_r_shift!(bitboards_t[piece], end_shift) & 1 == 1 {
                        hash_key_t ^= z.piece_keys[piece][(r2 * 8 + c2) as usize] // add target piece to hash
                    }
                    if usgn_r_shift!(bitboards[piece], end_shift) & 1 == 1 {
                        hash_key_t ^= z.piece_keys[piece][(r2 * 8 + c2) as usize] // remove taken piece from hash
                    }
                }
            } else if moves[i..i+4].chars().nth(3).unwrap() == 'P' { // pawn promo
                let (c1, c2, _, _) = move_to_u32s!(moves[i..i+4]);
                let (r1, r2) = if whites_turn {(1, 0)} else {(6, 7)};
                let piece: Piece = if whites_turn {Piece::WP} else {Piece::BP};
                let promo_piece: Piece;
                match moves[i..i+4].chars().nth(2).unwrap() {
                    'Q' => promo_piece = Piece::WQ,
                    'R' => promo_piece = Piece::WR,
                    'B' => promo_piece = Piece::WB,
                    'N' => promo_piece = Piece::WN,
                    'q' => promo_piece = Piece::BQ,
                    'r' => promo_piece = Piece::BR,
                    'b' => promo_piece = Piece::BB,
                    'n' => promo_piece = Piece::BN,
                    _ => panic!("INVALID PROMO TYPE"),
                }
                if moves[i..i+4].chars().nth(2).unwrap().is_uppercase() { // white promo
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
                hash_key_t ^= z.piece_keys[piece][(r1 * 8 + c1) as usize]; // remove source piece from hash
                hash_key_t ^= z.piece_keys[promo_piece][(r2 * 8 + c2) as usize]; // add promoted piece to hash
                for piece in [Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK] {
                    if usgn_r_shift!(bitboards[piece], end_shift) & 1 == 1 {
                        hash_key_t ^= z.piece_keys[piece][(r2 * 8 + c2) as usize] // remove taken piece from hash
                    }
                }
            } else if moves[i..i+4].chars().nth(3).unwrap() == 'E' { // enpassant
                let (c1, c2, _, _) = move_to_u32s!(moves[i..i+4]);
                let (r1, r2) = if whites_turn {(3, 2)} else {(4, 5)};
                if moves[i..i+4].chars().nth(2).unwrap() == 'w' { // white
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
                for piece in [Piece::WP, Piece::BP] {
                    if usgn_r_shift!(bitboards[piece], start_shift) & 1 == 1 {
                        hash_key_t ^= z.piece_keys[piece][(r1 * 8 + c1) as usize] // remove source piece from hash
                    }
                    if usgn_r_shift!(bitboards_t[piece], end_shift) & 1 == 1 {
                        hash_key_t ^= z.piece_keys[piece][(r2 * 8 + c2) as usize] // add target piece to hash
                    }
                    if usgn_r_shift!(bitboards[piece], if whites_turn {end_shift-8} else {end_shift+8}) & 1 == 1 {
                        hash_key_t ^= z.piece_keys[piece][(r1 * 8 + c2) as usize] // remove taken piece from hash
                    }
                }
            } else {
                panic!("INVALID MOVE TYPE");
            }
            // remove current enpassant status from hash
            if bitboards[Piece::EP] != 0 {
                let col: usize = bitboards[Piece::EP].leading_zeros() as usize;
                let row: usize = if whites_turn {2} else {5};
                hash_key_t ^= z.enpassant_keys[row * 8 + col];
            }
            // add next move enpassant status to hash
            if bitboards_t[Piece::EP] != 0 {
                let col: usize = bitboards_t[Piece::EP].leading_zeros() as usize;
                let row: usize = if !whites_turn {2} else {5};
                hash_key_t ^= z.enpassant_keys[row * 8 + col];
            }
            // remove current castle rights from hash
            hash_key_t ^= z.castle_keys[
                ((castle_rights[CastleRights::CBQ] as usize) << 3)
                | ((castle_rights[CastleRights::CBK] as usize) << 2)
                | ((castle_rights[CastleRights::CWQ] as usize) << 1)
                | (castle_rights[CastleRights::CWK] as usize)
            ];
            // add next moves castle rights to hash
            hash_key_t ^= z.castle_keys[
                ((castle_rights_t[CastleRights::CBQ] as usize) << 3)
                | ((castle_rights_t[CastleRights::CBK] as usize) << 2)
                | ((castle_rights_t[CastleRights::CWQ] as usize) << 1)
                | (castle_rights_t[CastleRights::CWK] as usize)
            ];
            // hash castle moves (rook move)
            if ((usgn_r_shift!(bitboards[Piece::WK], start_shift) & 1 == 1) || (usgn_r_shift!(bitboards[Piece::BK], start_shift) & 1 == 1)) && ((&moves[i..i+4] == "0402") || (&moves[i..i+4] == "0406") || (&moves[i..i+4] == "7472") || (&moves[i..i+4] == "7476")) {
                if whites_turn { // white
                    match &moves[i..i+4] {
                        "7476" => { // king side
                            hash_key_t ^= z.piece_keys[Piece::WR][63 - mm.castle_rooks[3]];
                            hash_key_t ^= z.piece_keys[Piece::WR][63 - (mm.castle_rooks[3] + 2)];
                        },
                        "7472" => { // queen side
                            hash_key_t ^= z.piece_keys[Piece::WR][63 - mm.castle_rooks[2]];
                            hash_key_t ^= z.piece_keys[Piece::WR][63 - (mm.castle_rooks[2] - 3)];
                        },
                        _ => (),
                    }
                } else { // black
                    match &moves[i..i+4] {
                        "0406" => { // king side
                            hash_key_t ^= z.piece_keys[Piece::BR][63 - mm.castle_rooks[1]];
                            hash_key_t ^= z.piece_keys[Piece::BR][63 - (mm.castle_rooks[1] + 2)];
                        },
                        "0402" => { // queen side
                            hash_key_t ^= z.piece_keys[Piece::BR][63 - mm.castle_rooks[0]];
                            hash_key_t ^= z.piece_keys[Piece::BR][63 - (mm.castle_rooks[0] - 3)];
                        },
                        _ => (),
                    }
                }
            }
            if scratch_hash != hash_key_t {
                println!("move: {}", move_to_algebra!(moves[i..i+4]));
                println!("hash key should be: {:x}", scratch_hash);
                println!("iterative hash key: {:x}", hash_key_t);
                break
            }



            if mm.isValidMove(bitboards_t, whites_turn) {
                self.perft(mm, z, bitboards_t, castle_rights_t, hash_key_t, !whites_turn, depth + 1);
                println!("{} {}", move_to_algebra!(moves[i..i+4]), self.move_counter);
                self.total_move_counter += self.move_counter;
                self.move_counter = 0;
            }
        }
    }
}


/// Tests (50,657,065 total moves made in these tests)


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine_modules::game_state::GameState;

    #[test]
    fn perft_starting_pos() {
        let mut z: Zobrist = Zobrist::new();
        let gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 4865609);
    }

    #[test]
    fn perft_complex_pos() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(&mut z, String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 4085603);
    }

    #[test]
    fn perft_wikispaces1() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 11030083);
    }

    #[test]
    fn perft_wikispaces2() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        gs.importFEN(&mut z, String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 15833292);
    }

    #[test]
    fn perft_wikispaces3() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        gs.importFEN(&mut z, String::from("1k6/1b6/8/8/7R/8/8/4K2R b K - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, false, 0);
        assert!(p.total_move_counter == 1063513);
    }

    #[test]
    fn perft_illegal_ep1() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, false, 0);
        assert!(p.total_move_counter == 1134888);
    }

    #[test]
    fn perft_illegal_ep2() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 1015133);
    }

    #[test]
    fn perft_ep_capture_checks_opponent() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, false, 0);
        assert!(p.total_move_counter == 1440467);
    }

    #[test]
    fn perft_short_castling_gives_check() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("5k2/8/8/8/8/8/8/4K2R w K - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 661072);
    }

    #[test]
    fn perft_long_castling_gives_check() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 803711);
    }

    #[test]
    fn perft_castle_rights() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(&mut z, String::from("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 1274206);
    }

    #[test]
    fn perft_castling_prevented() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(&mut z, String::from("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, false, 0);
        assert!(p.total_move_counter == 1720476);
    }

    #[test]
    fn perft_promote_out_of_check() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 3821001);
    }

    #[test]
    fn perft_discovered_check() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        gs.importFEN(&mut z, String::from("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, false, 0);
        assert!(p.total_move_counter == 1004658);
    }

    #[test]
    fn perft_promote_to_give_check() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("4k3/1P6/8/8/8/8/K7/8 w - - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 217342);
    }

    #[test]
    fn perft_under_promote_to_give_check() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("8/P1k5/K7/8/8/8/8/8 w - - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 92683);
    }

    #[test]
    fn perft_self_stalemate() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(&mut z, String::from("K1k5/8/P7/8/8/8/8/8 w - - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 2217);
    }

    #[test]
    fn perft_stalemate_and_checkmate1() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(7);
        gs.importFEN(&mut z, String::from("8/k1P5/8/1K6/8/8/8/8 w - - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
        assert!(p.total_move_counter == 567584);
    }

    #[test]
    fn perft_stalemate_and_checkmate2() {
        let mut z: Zobrist = Zobrist::new();
        let mut gs = GameState::new(&mut z);
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(&mut z, String::from("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1"));
        p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, false, 0);
        assert!(p.total_move_counter == 23527);
    }
}
