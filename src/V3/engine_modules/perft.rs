//! Module used for performance testing


use pyo3::prelude::*;
use crate::moves::Moves;
use crate::piece::Piece;


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct Perft {
    max_depth: u32,
    move_counter: u32,
    total_move_counter: u32,
}


#[pymethods]
impl Perft {
    #[new]
    fn new(max_depth: u32) -> Self {
        Perft {
            max_depth: max_depth,
            move_counter: 0,
            total_move_counter: 0,
        }
    }


    fn perft(&mut self, mm: &mut Moves, bitboards: [i64; 13], cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, depth: u32) {
        if depth < self.max_depth {
            let moves: String;
            if whites_turn {
                moves = mm.possibleMovesW(bitboards, cwK, cwQ);
            } else {
                moves = mm.possibleMovesB(bitboards, cbK, cbQ);
            }
            for i in (0..moves.len()).step_by(4) {
                let mut bitboards_t: [i64; 13] = [0; 13];
                bitboards_t[Piece::WP] = mm.makeMove(bitboards[Piece::WP], moves[i..i+4].to_string(), 'P'); bitboards_t[Piece::WN] = mm.makeMove(bitboards[Piece::WN], moves[i..i+4].to_string(), 'N');
                bitboards_t[Piece::WB] = mm.makeMove(bitboards[Piece::WB], moves[i..i+4].to_string(), 'B'); bitboards_t[Piece::WR] = mm.makeMove(bitboards[Piece::WR], moves[i..i+4].to_string(), 'R');
                bitboards_t[Piece::WQ] = mm.makeMove(bitboards[Piece::WQ], moves[i..i+4].to_string(), 'Q'); bitboards_t[Piece::WK] = mm.makeMove(bitboards[Piece::WK], moves[i..i+4].to_string(), 'K');
                bitboards_t[Piece::BP] = mm.makeMove(bitboards[Piece::BP], moves[i..i+4].to_string(), 'p'); bitboards_t[Piece::BN] = mm.makeMove(bitboards[Piece::BN], moves[i..i+4].to_string(), 'n');
                bitboards_t[Piece::BB] = mm.makeMove(bitboards[Piece::BB], moves[i..i+4].to_string(), 'b'); bitboards_t[Piece::BR] = mm.makeMove(bitboards[Piece::BR], moves[i..i+4].to_string(), 'r');
                bitboards_t[Piece::BQ] = mm.makeMove(bitboards[Piece::BQ], moves[i..i+4].to_string(), 'q'); bitboards_t[Piece::BK] = mm.makeMove(bitboards[Piece::BK], moves[i..i+4].to_string(), 'k');
                bitboards_t[Piece::WR] = mm.makeMoveCastle(bitboards_t[Piece::WR], bitboards[Piece::WK], moves[i..i+4].to_string(), 'R'); bitboards_t[Piece::BR] = mm.makeMoveCastle(bitboards_t[Piece::BR], bitboards[Piece::BK], moves[i..i+4].to_string(), 'r');
                bitboards_t[Piece::EP] = mm.makeMoveEP(bitboards[Piece::WP] | bitboards[Piece::BP], moves[i..i+4].to_string());

                let mut cwKt: bool = cwK; let mut cwQt: bool = cwQ; let mut cbKt: bool = cbK; let mut cbQt: bool = cbQ;

                if moves.chars().nth(i + 3).unwrap().is_numeric() {
                    let m1: u32 = moves.chars().nth(i).unwrap().to_digit(10).unwrap();
                    let m2: u32 = moves.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                    let m3: u32 = moves.chars().nth(i + 2).unwrap().to_digit(10).unwrap();
                    let m4: u32 = moves.chars().nth(i + 3).unwrap().to_digit(10).unwrap();
                    let start_shift: u32 = 64 - 1 - (m1 * 8 + m2);
                    let end_shift: u32 = 64 - 1 - (m3 * 8 + m4);
                    if ((1 << start_shift) & bitboards[Piece::WK]) != 0 { // white king move
                        (cwKt, cwQt) = (false, false);
                    }
                    if ((1 << start_shift) & bitboards[Piece::BK]) != 0 { // black king move
                        (cbKt, cbQt) = (false, false);
                    }
                    if ((1 << start_shift) & bitboards[Piece::WR] & 1) != 0 { // white king side rook move
                        cwKt = false;
                    }
                    if ((1 << start_shift) & bitboards[Piece::WR] & (1 << 7)) != 0 { // white queen side rook move
                        cwQt = false;
                    }
                    if ((1 << start_shift) & bitboards[Piece::BR] & (1 << 56)) != 0 { // black king side rook move
                        cbKt = false;
                    }
                    if ((1 << start_shift) & bitboards[Piece::BR] & (1 << 63)) != 0 { // black queen side rook move
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

                if ((bitboards_t[Piece::WK] & mm.unsafeForWhite(bitboards_t)) == 0 && whites_turn) || ((bitboards_t[Piece::BK] & mm.unsafeForBlack(bitboards_t)) == 0 && !whites_turn) {
                    if depth + 1 == self.max_depth { // only count leaf nodes
                        self.move_counter += 1
                    }
                    // println!("{:?}", self.move_counter);
                    self.perft(mm, bitboards_t, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth + 1)
                }
            }
        } else if self.move_counter == 0 {
            self.move_counter += 1;
        }
    }


    fn perftRoot(&mut self, mm: &mut Moves, bitboards: [i64; 13], cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, depth: u32) {
        let moves: String;
        if whites_turn {
            moves = mm.possibleMovesW(bitboards, cwK, cwQ);
        } else {
            moves = mm.possibleMovesB(bitboards, cbK, cbQ);
        }
        for i in (0..moves.len()).step_by(4) {
            let mut bitboards_t: [i64; 13] = [0; 13];
            bitboards_t[Piece::WP] = mm.makeMove(bitboards[Piece::WP], moves[i..i+4].to_string(), 'P'); bitboards_t[Piece::WN] = mm.makeMove(bitboards[Piece::WN], moves[i..i+4].to_string(), 'N');
            bitboards_t[Piece::WB] = mm.makeMove(bitboards[Piece::WB], moves[i..i+4].to_string(), 'B'); bitboards_t[Piece::WR] = mm.makeMove(bitboards[Piece::WR], moves[i..i+4].to_string(), 'R');
            bitboards_t[Piece::WQ] = mm.makeMove(bitboards[Piece::WQ], moves[i..i+4].to_string(), 'Q'); bitboards_t[Piece::WK] = mm.makeMove(bitboards[Piece::WK], moves[i..i+4].to_string(), 'K');
            bitboards_t[Piece::BP] = mm.makeMove(bitboards[Piece::BP], moves[i..i+4].to_string(), 'p'); bitboards_t[Piece::BN] = mm.makeMove(bitboards[Piece::BN], moves[i..i+4].to_string(), 'n');
            bitboards_t[Piece::BB] = mm.makeMove(bitboards[Piece::BB], moves[i..i+4].to_string(), 'b'); bitboards_t[Piece::BR] = mm.makeMove(bitboards[Piece::BR], moves[i..i+4].to_string(), 'r');
            bitboards_t[Piece::BQ] = mm.makeMove(bitboards[Piece::BQ], moves[i..i+4].to_string(), 'q'); bitboards_t[Piece::BK] = mm.makeMove(bitboards[Piece::BK], moves[i..i+4].to_string(), 'k');
            bitboards_t[Piece::WR] = mm.makeMoveCastle(bitboards_t[Piece::WR], bitboards[Piece::WK], moves[i..i+4].to_string(), 'R'); bitboards_t[Piece::BR] = mm.makeMoveCastle(bitboards_t[Piece::BR], bitboards[Piece::BK], moves[i..i+4].to_string(), 'r');
            bitboards_t[Piece::EP] = mm.makeMoveEP(bitboards[Piece::WP] | bitboards[Piece::BP], moves[i..i+4].to_string());

            let mut cwKt: bool = cwK; let mut cwQt: bool = cwQ; let mut cbKt: bool = cbK; let mut cbQt: bool = cbQ;

            if moves.chars().nth(i + 3).unwrap().is_numeric() {
                let m1: u32 = moves.chars().nth(i).unwrap().to_digit(10).unwrap();
                let m2: u32 = moves.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                let m3: u32 = moves.chars().nth(i + 2).unwrap().to_digit(10).unwrap();
                let m4: u32 = moves.chars().nth(i + 3).unwrap().to_digit(10).unwrap();
                let start_shift: u32 = 64 - 1 - (m1 * 8 + m2);
                let end_shift: u32 = 64 - 1 - (m3 * 8 + m4);
                if ((1 << start_shift) & bitboards[Piece::WK]) != 0 { // white king move
                    (cwKt, cwQt) = (false, false);
                }
                if ((1 << start_shift) & bitboards[Piece::BK]) != 0 { // black king move
                    (cbKt, cbQt) = (false, false);
                }
                if ((1 << start_shift) & bitboards[Piece::WR] & 1) != 0 { // white king side rook move
                    cwKt = false;
                }
                if ((1 << start_shift) & bitboards[Piece::WR] & (1 << 7)) != 0 { // white queen side rook move
                    cwQt = false;
                }
                if ((1 << start_shift) & bitboards[Piece::BR] & (1 << 56)) != 0 { // black king side rook move
                    cbKt = false;
                }
                if ((1 << start_shift) & bitboards[Piece::BR] & (1 << 63)) != 0 { // black queen side rook move
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

            if ((bitboards_t[Piece::WK] & mm.unsafeForWhite(bitboards_t)) == 0 && whites_turn) || ((bitboards_t[Piece::BK] & mm.unsafeForBlack(bitboards_t)) == 0 && !whites_turn) {
                self.perft(mm, bitboards_t, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth + 1);
                println!("{} {}", move_to_algebra!(moves[i..i+4]), self.move_counter);
                self.total_move_counter += self.move_counter;
                self.move_counter = 0;
            }
        }
    }
}


/// Tests


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine_modules::game_state::GameState;

    #[test]
    fn basic_test() {
        println!("Basic Test!");
        // let mut gs = GameState::new();
        // let mut m: Moves = Moves::new();
        // let mut p: Perft = Perft::new(3);
        // let mut bmf: BestMoveFinder = BestMoveFinder::new(3);
        println!("DONE!");
        // panic!();
    }

    #[test]
    fn perft_starting_pos() {
        let gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 4865609);
    }

    #[test]
    fn perft_complex_pos() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 4085603);
    }

    #[test]
    fn perft_wikispaces1() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 11030083);
    }

    #[test]
    fn perft_wikispaces2() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        gs.importFEN(String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 15833292);
    }

    #[test]
    fn perft_wikispaces3() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        gs.importFEN(String::from("1k6/1b6/8/8/7R/8/8/4K2R b K - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1063513);
    }

    #[test]
    fn perft_illegal_ep1() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1134888);
    }

    #[test]
    fn perft_illegal_ep2() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 1015133);
    }

    #[test]
    fn perft_ep_capture_checks_opponent() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1440467);
    }

    #[test]
    fn perft_short_castling_gives_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("5k2/8/8/8/8/8/8/4K2R w K - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 661072);
    }

    #[test]
    fn perft_long_castling_gives_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 803711);
    }

    #[test]
    fn perft_castle_rights() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(String::from("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 1274206);
    }

    #[test]
    fn perft_castling_prevented() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(String::from("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1720476);
    }

    #[test]
    fn perft_promote_out_of_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 3821001);
    }

    #[test]
    fn perft_discovered_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        gs.importFEN(String::from("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1004658);
    }

    #[test]
    fn perft_promote_to_give_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("4k3/1P6/8/8/8/8/K7/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 217342);
    }

    #[test]
    fn perft_under_promote_to_give_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("8/P1k5/K7/8/8/8/8/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 92683);
    }

    #[test]
    fn perft_self_stalemate() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("K1k5/8/P7/8/8/8/8/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 2217);
    }

    #[test]
    fn perft_stalemate_and_checkmate1() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(7);
        gs.importFEN(String::from("8/k1P5/8/1K6/8/8/8/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 567584);
    }

    #[test]
    fn perft_stalemate_and_checkmate2() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(String::from("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1"));
        p.perftRoot(&mut m, gs.bitboards, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 23527);
    }
}