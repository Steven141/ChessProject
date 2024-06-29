//! Chess Engine Library


#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]


use pyo3::prelude::*;
use std::time::Instant;
use std::collections::HashMap;
use std::str::from_utf8;
use rand::thread_rng;
use rand::seq::SliceRandom;


/// Holds specific bitboards
#[pyclass(module = "ChessProject", get_all, set_all)]
#[derive(Clone)]
#[derive(Debug)]
pub struct SpecialBitBoards {
    // specific bitboard masks
    file_ab: i64,
    file_gh: i64,
    centre: i64,
    extended_centre: i64,
    king_side: i64,
    queen_side: i64,
    king_span_c7: i64, // where c7 king can attack
    knight_span_c6: i64, // where c6 knight can attack
    not_allied_pieces: i64, // if in white func: all pieces white can capture (not black king
    enemy_pieces: i64, // if in white func: black pieces but no black king
    empty: i64,
    occupied: i64,

    // region based bitboard masks
    rank_masks: [i64; 8], // from rank 8 to rank 1
    file_masks: [i64; 8], // from file a to file h
    diagonal_masks: [i64; 15], // from top left to bottom right
    anti_diagonal_masks: [i64; 15], // from top right to bottom left
}


#[pymethods]
impl SpecialBitBoards {
    #[new]
    fn new() -> Self {
        SpecialBitBoards {
            file_ab: -4557430888798830400,
            file_gh: 217020518514230019,
            centre: 103481868288,
            extended_centre: 66229406269440,
            king_side: 1085102592571150095,
            queen_side: -1085102592571150096,
            king_span_c7: 8093091675687092224,
            knight_span_c6: 5802888705324613632,
            not_allied_pieces: 0,
            enemy_pieces: 0,
            empty: 0,
            occupied: 0,
            rank_masks: [
                -72057594037927936,
                71776119061217280,
                280375465082880,
                1095216660480,
                4278190080,
                16711680,
                65280,
                255,
            ],
            file_masks: [
                -9187201950435737472,
                4629771061636907072,
                2314885530818453536,
                1157442765409226768,
                578721382704613384,
                289360691352306692,
                144680345676153346,
                72340172838076673,
            ],
            diagonal_masks: [
                -9223372036854775808,
                4647714815446351872,
                2323998145211531264,
                1161999622361579520,
                580999813328273408,
                290499906672525312,
                145249953336295424,
                72624976668147840,
                283691315109952,
                1108169199648,
                4328785936,
                16909320,
                66052,
                258,
                1,
            ],
            anti_diagonal_masks: [
                72057594037927936,
                144396663052566528,
                288794425616760832,
                577588855528488960,
                1155177711073755136,
                2310355422147575808,
                4620710844295151872,
                -9205322385119247871,
                36099303471055874,
                141012904183812,
                550831656968,
                2151686160,
                8405024,
                32832,
                128,
            ],
        }
    }
}


/// Keeps track of game state, possible moves, and old moves
#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct GameState {
    board: [[char; 8]; 8],
    cwK: bool,
    cwQ: bool,
    cbK: bool,
    cbQ: bool,
    whites_turn: bool,
    wP: i64,
    wN: i64,
    wB: i64,
    wR: i64,
    wQ: i64,
    wK: i64,
    bP: i64,
    bN: i64,
    bB: i64,
    bR: i64,
    bQ: i64,
    bK: i64,
    EP: i64,
    masks: SpecialBitBoards,
    move_log: String,
    recent_piece_moved: char,
    recent_piece_captured: char,
}


#[pymethods]
impl GameState {
    #[new]
    fn new() -> Self {
        let mut gs: GameState = GameState {
            board: [
                ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
                ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
                ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
            ],
            cwK: true,
            cwQ: true,
            cbK: true,
            cbQ: true,
            whites_turn: true,
            wP: 0,
            wN: 0,
            wB: 0,
            wR: 0,
            wQ: 0,
            wK: 0,
            bP: 0,
            bN: 0,
            bB: 0,
            bR: 0,
            bQ: 0,
            bK: 0,
            EP: 0,
            masks: SpecialBitBoards::new(),
            move_log: String::new(),
            recent_piece_moved: ' ',
            recent_piece_captured: ' ',
        };
        gs.arrayToI64();
        return gs;
    }


    fn arrayToI64(&mut self) {
        for i in 0..64 {
            let mut bin_str: String = String::from("0000000000000000000000000000000000000000000000000000000000000000");
            bin_str.replace_range(i..i+1, "1");
            match self.board[i / 8][i % 8] {
                'P' => self.wP += self.binToI64(&bin_str),
                'N' => self.wN += self.binToI64(&bin_str),
                'B' => self.wB += self.binToI64(&bin_str),
                'R' => self.wR += self.binToI64(&bin_str),
                'Q' => self.wQ += self.binToI64(&bin_str),
                'K' => self.wK += self.binToI64(&bin_str),
                'p' => self.bP += self.binToI64(&bin_str),
                'n' => self.bN += self.binToI64(&bin_str),
                'b' => self.bB += self.binToI64(&bin_str),
                'r' => self.bR += self.binToI64(&bin_str),
                'q' => self.bQ += self.binToI64(&bin_str),
                'k' => self.bK += self.binToI64(&bin_str),
                _ => (),
            }
        }
    }


    fn binToI64(&self, bin_str: &str) -> i64 {
        let mut usgn_value: u64 = u64::from_str_radix(&bin_str, 2).unwrap();
        if bin_str.chars().next() == Some('1') {
            usgn_value -= (1 << 63) - (1 << 63); // Two's Compliment
        }
        usgn_value as i64
    }


    fn drawGameArray(&self) {
        let mut new_board: [[char; 8]; 8] = [[' '; 8]; 8];
        for i in 0..64 {
            let shift = 64 - 1 - i;
            if usgn_r_shift!(self.wP, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'P';
            }
            if usgn_r_shift!(self.wN, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'N';
            }
            if usgn_r_shift!(self.wB, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'B';
            }
            if usgn_r_shift!(self.wR, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'R';
            }
            if usgn_r_shift!(self.wQ, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'Q';
            }
            if usgn_r_shift!(self.wK, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'K';
            }
            if usgn_r_shift!(self.bP, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'p';
            }
            if usgn_r_shift!(self.bN, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'n';
            }
            if usgn_r_shift!(self.bB, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'b';
            }
            if usgn_r_shift!(self.bR, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'r';
            }
            if usgn_r_shift!(self.bQ, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'q';
            }
            if usgn_r_shift!(self.bK, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'k';
            }
        }
        for row in 0..8 {
            for col in 0..8 {
                print!("{}", new_board[row][col]);
            }
            println!();
        }
        println!();
    }


    fn updateBoardArray(&mut self) {
        self.board = [[' '; 8]; 8];
        for i in 0..64 {
            let shift = 64 - 1 - i;
            if usgn_r_shift!(self.wP, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'P';
            }
            if usgn_r_shift!(self.wN, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'N';
            }
            if usgn_r_shift!(self.wB, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'B';
            }
            if usgn_r_shift!(self.wR, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'R';
            }
            if usgn_r_shift!(self.wQ, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'Q';
            }
            if usgn_r_shift!(self.wK, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'K';
            }
            if usgn_r_shift!(self.bP, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'p';
            }
            if usgn_r_shift!(self.bN, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'n';
            }
            if usgn_r_shift!(self.bB, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'b';
            }
            if usgn_r_shift!(self.bR, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'r';
            }
            if usgn_r_shift!(self.bQ, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'q';
            }
            if usgn_r_shift!(self.bK, shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'k';
            }
        }
    }


    fn importFEN(&mut self, fen_str: String) {
        self.wP = 0; self.wN = 0; self.wB = 0;
        self.wR = 0; self.wQ = 0; self.wK = 0;
        self.bP = 0; self.bN = 0; self.bB = 0;
        self.bR = 0; self.bQ = 0; self.bK = 0;
        self.cwK = false; self.cwQ = false;
        self.cbK = false; self.cbQ = false;
        let mut char_idx: usize = 0;
        let mut board_idx: i64 = 0;
        while fen_str.chars().nth(char_idx).unwrap() != ' ' {
            let board_idx_shift: i64 = 64 - 1 - board_idx;
            match fen_str.chars().nth(char_idx).unwrap() {
                'P' => {
                    self.wP |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'N' => {
                    self.wN |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'B' => {
                    self.wB |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'R' => {
                    self.wR |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'Q' => {
                    self.wQ |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'K' => {
                    self.wK |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'p' => {
                    self.bP |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'n' => {
                    self.bN |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'b' => {
                    self.bB |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'r' => {
                    self.bR |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'q' => {
                    self.bQ |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                'k' => {
                    self.bK |= (1 << board_idx_shift);
                    board_idx += 1;
                },
                '1' => board_idx += 1,
                '2' => board_idx += 2,
                '3' => board_idx += 3,
                '4' => board_idx += 4,
                '5' => board_idx += 5,
                '6' => board_idx += 6,
                '7' => board_idx += 7,
                '8' => board_idx += 8,
                _ => (),
            }
            char_idx += 1;
        }

        char_idx += 1;
        self.whites_turn = fen_str.chars().nth(char_idx).unwrap() == 'w';
        char_idx += 2;

        while fen_str.chars().nth(char_idx).unwrap() != ' ' {
            match fen_str.chars().nth(char_idx).unwrap() {
                'K' => self.cwK = true,
                'Q' => self.cwQ = true,
                'k' => self.cbK = true,
                'q' => self.cbQ = true,
                _ => (),
            }
            char_idx += 1;
        }

        char_idx += 1;
        if fen_str.chars().nth(char_idx).unwrap() != '-' {
            self.EP = self.masks.file_masks[fen_str.chars().nth(char_idx).unwrap() as usize - 'a' as usize];
            char_idx += 1;
        }
        self.updateBoardArray()
        // Rest of FEN not used
    }


    fn makeMove(&mut self, mm: &mut Moves, move_str: String) {
        if move_str.chars().nth(3).unwrap() == 'E' {
            self.recent_piece_captured = if self.whites_turn {'p'} else {'P'};
            self.recent_piece_moved = if self.whites_turn {'P'} else {'p'};
        } else if move_str.chars().nth(3).unwrap() == 'P' {
            self.recent_piece_captured = self.board[if self.whites_turn {0} else {7}][move_str.chars().nth(1).unwrap().to_digit(10).unwrap() as usize];
            self.recent_piece_moved = if self.whites_turn {'P'} else {'p'};
        } else {
            self.recent_piece_captured = self.board[move_str.chars().nth(2).unwrap().to_digit(10).unwrap() as usize][move_str.chars().nth(3).unwrap().to_digit(10).unwrap() as usize];
            self.recent_piece_moved = self.board[move_str.chars().nth(0).unwrap().to_digit(10).unwrap() as usize][move_str.chars().nth(1).unwrap().to_digit(10).unwrap() as usize];
        }
        self.move_log.push_str(&move_str);
        let wK_cached: i64 = self.wK;
        let bK_cached: i64 = self.bK;
        let wR_cached: i64 = self.wR;
        let bR_cached: i64 = self.bR;
        let wP_cached: i64 = self.wP;
        let bP_cached: i64 = self.bP;

        self.wP = mm.makeMove(self.wP, move_str.clone(), 'P'); self.wN = mm.makeMove(self.wN, move_str.clone(), 'N');
        self.wB = mm.makeMove(self.wB, move_str.clone(), 'B'); self.wR = mm.makeMove(self.wR, move_str.clone(), 'R');
        self.wQ = mm.makeMove(self.wQ, move_str.clone(), 'Q'); self.wK = mm.makeMove(self.wK, move_str.clone(), 'K');
        self.bP = mm.makeMove(self.bP, move_str.clone(), 'p'); self.bN = mm.makeMove(self.bN, move_str.clone(), 'n');
        self.bB = mm.makeMove(self.bB, move_str.clone(), 'b'); self.bR = mm.makeMove(self.bR, move_str.clone(), 'r');
        self.bQ = mm.makeMove(self.bQ, move_str.clone(), 'q'); self.bK = mm.makeMove(self.bK, move_str.clone(), 'k');
        self.wR = mm.makeMoveCastle(self.wR, wK_cached, move_str.clone(), 'R'); self.bR = mm.makeMoveCastle(self.bR, bK_cached, move_str.clone(), 'r');
        self.EP = mm.makeMoveEP(wP_cached|bP_cached, move_str.clone());

        if move_str.chars().nth(3).unwrap().is_numeric() {
            let m1: u32 = move_str.chars().nth(0).unwrap().to_digit(10).unwrap();
            let m2: u32 = move_str.chars().nth(1).unwrap().to_digit(10).unwrap();
            let m3: u32 = move_str.chars().nth(2).unwrap().to_digit(10).unwrap();
            let m4: u32 = move_str.chars().nth(3).unwrap().to_digit(10).unwrap();
            let start_shift: u32 = 64 - 1 - (m1 * 8 + m2);
            let end_shift: u32 = 64 - 1 - (m3 * 8 + m4);
            if ((1 << start_shift) & wK_cached) != 0 { // white king move
                (self.cwK, self.cwQ) = (false, false);
            }
            if ((1 << start_shift) & bK_cached) != 0 { // black king move
                (self.cbK, self.cbQ) = (false, false);
            }
            if ((1 << start_shift) & wR_cached & 1) != 0 { // white king side rook move
                self.cwK = false;
            }
            if ((1 << start_shift) & wR_cached & (1 << 7)) != 0 { // white queen side rook move
                self.cwQ = false;
            }
            if ((1 << start_shift) & bR_cached & (1 << 56)) != 0 { // black king side rook move
                self.cbK = false;
            }
            if ((1 << start_shift) & bR_cached & (1 << 63)) != 0 { // black queen side rook move
                self.cbQ = false;
            }
            if (((1 as i64) << end_shift) & 1) != 0 { // white king side rook taken
                self.cwK = false;
            }
            if (((1 as i64) << end_shift) & (1 << 7)) != 0 { // white queen side rook taken
                self.cwQ = false;
            }
            if ((1 << end_shift) & ((1 as i64) << 56)) != 0 { // black king side rook taken
                self.cbK = false;
            }
            if ((1 << end_shift) & ((1 as i64) << 63)) != 0 { // black queen side rook taken
                self.cbQ = false;
            }
        }

        self.whites_turn = !self.whites_turn;
        self.updateBoardArray();
    }
}


/// Holds information about all the moves
#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct Moves {
    castle_rooks: [usize; 4],
    masks: SpecialBitBoards,
    checkmate: bool,
    stalemate: bool,
}


#[pymethods]
impl Moves {
    #[new]
    fn new() -> Self {
        Moves {
            castle_rooks: [63, 56, 7, 0],
            masks: SpecialBitBoards::new(),
            checkmate: false,
            stalemate: false,
        }
    }


    fn getValidMoves(&mut self, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, EP: i64, cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, depth: u32) -> String {
        let mut moves: String = String::new();
        if whites_turn {
            moves = self.possibleMovesW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        } else {
            moves = self.possibleMovesB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        }
        if depth == 0 {
            let mut move_groups: Vec<&str> = moves.as_bytes().chunks(4).map(|chunk| from_utf8(chunk).unwrap()).collect();
            move_groups.shuffle(&mut thread_rng());
            moves = move_groups.join("");
        }
        let mut valid_moves: String = String::new();
        for i in (0..moves.len()).step_by(4) {
            let wPt: i64 = self.makeMove(wP, moves[i..i+4].to_string(), 'P'); let wNt: i64 = self.makeMove(wN, moves[i..i+4].to_string(), 'N');
            let wBt: i64 = self.makeMove(wB, moves[i..i+4].to_string(), 'B'); let wRt: i64 = self.makeMove(wR, moves[i..i+4].to_string(), 'R');
            let wQt: i64 = self.makeMove(wQ, moves[i..i+4].to_string(), 'Q'); let wKt: i64 = self.makeMove(wK, moves[i..i+4].to_string(), 'K');
            let bPt: i64 = self.makeMove(bP, moves[i..i+4].to_string(), 'p'); let bNt: i64 = self.makeMove(bN, moves[i..i+4].to_string(), 'n');
            let bBt: i64 = self.makeMove(bB, moves[i..i+4].to_string(), 'b'); let bRt: i64 = self.makeMove(bR, moves[i..i+4].to_string(), 'r');
            let bQt: i64 = self.makeMove(bQ, moves[i..i+4].to_string(), 'q'); let bKt: i64 = self.makeMove(bK, moves[i..i+4].to_string(), 'k');
            let wRt: i64 = self.makeMoveCastle(wRt, wK, moves[i..i+4].to_string(), 'R'); let bRt: i64 = self.makeMoveCastle(bRt, bK, moves[i..i+4].to_string(), 'r');

            let is_valid_move: bool = ((wKt & self.unsafeForWhite(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && whites_turn) || ((bKt & self.unsafeForBlack(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && !whites_turn);
            if is_valid_move {
                valid_moves += &moves[i..i+4];
            }
        }
        if valid_moves.len() == 0 {
            if ((wK & self.unsafeForWhite(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK)) != 0 && whites_turn) || ((bK & self.unsafeForBlack(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK)) != 0 && !whites_turn) {
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


    fn makeMove(&self, mut bitboard: i64, move_str: String, p_type: char) -> i64 {
        let m1: u32 = move_str.chars().nth(0).unwrap().to_digit(10).unwrap_or(0);
        let m2: u32 = move_str.chars().nth(1).unwrap().to_digit(10).unwrap_or(0);
        let m3: u32 = move_str.chars().nth(2).unwrap().to_digit(10).unwrap_or(0);
        let m4: u32 = move_str.chars().nth(3).unwrap().to_digit(10).unwrap_or(0);
        let mut start_shift: u32 = 0;
        let mut end_shift: u32 = 0;
        let mut start_bitboard: i64 = 0;
        let mut end_bitboard: i64 = 0;
        if move_str.chars().nth(3).unwrap().is_numeric() { // regular move
            start_shift = 64 - 1 - (m1 * 8 + m2);
            end_shift = 64 - 1 - (m3 * 8 + m4);
            if usgn_r_shift!(bitboard, start_shift) & 1 == 1 {
                bitboard &= !(1 << start_shift); // remove moving piece from board
                bitboard |= (1 << end_shift); // add at new position
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
                bitboard |= (1 << end_shift);
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
                bitboard |= (1 << end_shift);
            }
        } else {
            panic!("INVALID MOVE TYPE");
        }
        bitboard
    }


    fn makeMoveCastle(&self, mut rook: i64, king: i64, move_str: String, p_type: char) -> i64 {
        let r1: usize = move_str.chars().nth(0).unwrap().to_digit(10).unwrap_or(0) as usize;
        let c1: usize = move_str.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as usize;
        let r2: usize = move_str.chars().nth(2).unwrap().to_digit(10).unwrap_or(0) as usize;
        let c2: usize = move_str.chars().nth(3).unwrap().to_digit(10).unwrap_or(0) as usize;
        let start_shift: usize = 64 - 1 - (r1 * 8 + c1);
        if (usgn_r_shift!(king, start_shift) & 1 == 1) && ((move_str == "0402") || (move_str == "0406") || (move_str == "7472") || (move_str == "7476")) {
            if p_type == 'R' { // white
                match move_str.as_str() {
                    "7476" => { // king side
                        rook &= !(1 << self.castle_rooks[3]);
                        rook |= (1 << (self.castle_rooks[3] + 2));
                    },
                    "7472" => { // queen side
                        rook &= !(1 << self.castle_rooks[2]);
                        rook |= (1 << (self.castle_rooks[2] - 3));
                    },
                    _ => (),
                }
            } else { // black
                match move_str.as_str() {
                    "0406" => { // king side
                        rook &= !(1 << self.castle_rooks[1]);
                        rook |= (1 << (self.castle_rooks[1] + 2));
                    },
                    "0402" => { // queen side
                        rook &= !(1 << self.castle_rooks[0]);
                        rook |= (1 << (self.castle_rooks[0] - 3));
                    },
                    _ => (),
                }
            }
        }
        rook
    }


    fn makeMoveEP(&self, bitboard: i64, move_str: String) -> i64 {
        let r1: usize = move_str.chars().nth(0).unwrap().to_digit(10).unwrap_or(0) as usize;
        let c1: usize = move_str.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as usize;
        let r2: usize = move_str.chars().nth(2).unwrap().to_digit(10).unwrap_or(0) as usize;
        let c2: usize = move_str.chars().nth(3).unwrap().to_digit(10).unwrap_or(0) as usize;
        let start_shift: usize = 64 - 1 - (r1 * 8 + c1);
        if move_str.chars().nth(3).unwrap().is_numeric() && ((r1 as i64 - r2 as i64).abs() == 2) && ((usgn_r_shift!(bitboard, start_shift) & 1) == 1) {
            self.masks.file_masks[c1]
        } else {
            0
        }
    }


    fn possibleMovesW(&mut self, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, EP: i64, cwK: bool, cwQ: bool, cbK: bool, cbQ: bool) -> String {
        self.masks.not_allied_pieces = !(wP|wN|wB|wR|wQ|wK|bK); // avoid illegal bK capture
        self.masks.enemy_pieces = bP|bN|bB|bR|bQ; // avoid illegal bK capture
        self.masks.empty = !(wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK);
        self.masks.occupied = !self.masks.empty;
        self.possibleWP(wP, bP, EP)
            + &self.possibleB(wB)
            + &self.possibleQ(wQ)
            + &self.possibleR(wR)
            + &self.possibleN(wN)
            + &self.possibleK(wK)
            + &self.possibleCastleW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, cwK, cwQ)
    }


    fn possibleMovesB(&mut self, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, EP: i64, cwK: bool, cwQ: bool, cbK: bool, cbQ: bool) -> String {
        self.masks.not_allied_pieces = !(bP|bN|bB|bR|bQ|bK|wK); // avoid illegal wK capture
        self.masks.enemy_pieces = wP|wN|wB|wR|wQ; // avoid illegal wK capture
        self.masks.empty = !(wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK);
        self.masks.occupied = !self.masks.empty;
        self.possibleBP(wP, bP, EP)
            + &self.possibleB(bB)
            + &self.possibleQ(bQ)
            + &self.possibleR(bR)
            + &self.possibleN(bN)
            + &self.possibleK(bK)
            + &self.possibleCastleB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, cbK, cbQ)
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
            let mut moves: i64 = 0;
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
            let mut moves: i64 = 0;
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


    fn possibleCastleW(&mut self, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, cwK: bool, cwQ: bool) -> String {
        let unsafe_w: i64 = self.unsafeForWhite(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK);
        let mut move_list: String = String::new(); // king move r1c1r2c1
        if unsafe_w & wK == 0 {
            if cwK && (((1 << self.castle_rooks[3]) & wR) != 0) {
                if ((self.masks.occupied | unsafe_w) & ((1 << 1) | (1 << 2))) == 0 {
                    move_list += "7476";
                }
            }
            if cwQ && (((1 << self.castle_rooks[2]) & wR) != 0) {
                if ((self.masks.occupied | (unsafe_w & !(1 << 6))) & ((1 << 4) | (1 << 5) | (1 << 6))) == 0 {
                    move_list += "7472";
                }
            }
        }
        move_list
    }


    fn possibleCastleB(&mut self, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, cbK: bool, cbQ: bool) -> String {
        let unsafe_b = self.unsafeForBlack(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK);
        let mut move_list: String = String::new(); // king move r1c1r2c1
        if unsafe_b & bK == 0 {
            if cbK && (((1 << self.castle_rooks[1]) & bR) != 0) {
                if ((self.masks.occupied | unsafe_b) & ((1 << 58) | (1 << 57))) == 0 {
                    move_list += "0406";
                }
            }
            if cbQ && (((1 << self.castle_rooks[0]) & bR) != 0) {
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


    fn unsafeForBlack(&mut self, mut wP: i64, mut wN: i64, mut wB: i64, mut wR: i64, mut wQ: i64, mut wK: i64, mut bP: i64, mut bN: i64, mut bB: i64, mut bR: i64, mut bQ: i64, mut bK: i64) -> i64 {
        self.masks.occupied = wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK;
        // pawn threats
        let mut unsafe_b: i64 = (wP << 7) & !self.masks.file_masks[0]; // pawn right capture
        unsafe_b |= ((wP << 9) & !self.masks.file_masks[7]); // pawn left capture

        // knight threat
        let mut knight: i64 = wN & !wrap_op!(wN, 1, '-');
        let knight_span_c6_idx: usize = 18;
        while knight != 0 {
            let knight_idx: usize = knight.leading_zeros() as usize;
            // allign the knight_span_c6 mask
            let mut moves: i64 = 0;
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
            wN &= !knight; // remove current knight
            knight = wN & !wrap_op!(wN, 1, '-');
        }

        // bishop / queen threats (diagonals)
        let mut wQB: i64 = wQ | wB;
        let mut b_or_q: i64 = wQB & !wrap_op!(wQB, 1, '-');
        while b_or_q != 0 {
            let b_or_q_idx: usize = b_or_q.leading_zeros() as usize;
            let moves: i64 = self.possibleDiagAndAntiDiagMoves(b_or_q_idx);
            unsafe_b |= moves;
            wQB &= !b_or_q; // remove current bishop or queen
            b_or_q = wQB & !wrap_op!(wQB, 1, '-');
        }

        // rook / queen threats (hor and vert)
        let mut wQR: i64 = wQ | wR;
        let mut r_or_q: i64 = wQR & !wrap_op!(wQR, 1, '-');
        while r_or_q != 0 {
            let r_or_q_idx: usize = r_or_q.leading_zeros() as usize;
            let moves: i64 = self.possibleHAndVMoves(r_or_q_idx);
            unsafe_b |= moves;
            wQR &= !r_or_q; // remove current rook or queen
            r_or_q = wQR & !wrap_op!(wQR, 1, '-');
        }

        // king threats
        let mut king: i64 = wK & !wrap_op!(wK, 1, '-');
        let king_span_c7_idx: usize = 10;
        while king != 0 {
            let king_idx: usize = king.leading_zeros() as usize;
            // allign the king_span_c7 mask
            let mut moves: i64 = 0;
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
            wK &= !king; // remove current king
            king = wK & !wrap_op!(wK, 1, '-');
        }
        unsafe_b
    }


    fn unsafeForWhite(&mut self, mut wP: i64, mut wN: i64, mut wB: i64, mut wR: i64, mut wQ: i64, mut wK: i64, mut bP: i64, mut bN: i64, mut bB: i64, mut bR: i64, mut bQ: i64, mut bK: i64) -> i64 {
        self.masks.occupied = wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK;
        // pawn threats
        let mut unsafe_w: i64 = usgn_r_shift!(bP, 7) & !self.masks.file_masks[7]; // pawn right capture
        unsafe_w |= (usgn_r_shift!(bP, 9) & !self.masks.file_masks[0]); // pawn left capture

        // knight threat
        let mut knight: i64 = bN & !wrap_op!(bN, 1, '-');
        let knight_span_c6_idx: usize = 18;
        while knight != 0 {
            let knight_idx: usize = knight.leading_zeros() as usize;
            // allign the knight_span_c6 mask
            let mut moves: i64 = 0;
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
            bN &= !knight; // remove current knight
            knight = bN & !wrap_op!(bN, 1, '-');
        }

        // bishop / queen threats (diagonals)
        let mut bQB: i64 = bQ | bB;
        let mut b_or_q: i64 = bQB & !wrap_op!(bQB, 1, '-');
        while b_or_q != 0 {
            let b_or_q_idx: usize = b_or_q.leading_zeros() as usize;
            let moves: i64 = self.possibleDiagAndAntiDiagMoves(b_or_q_idx);
            unsafe_w |= moves;
            bQB &= !b_or_q; // remove current bishop or queen
            b_or_q = bQB & !wrap_op!(bQB, 1, '-');
        }

        // rook / queen threats (hor and vert)
        let mut bQR: i64 = bQ | bR;
        let mut r_or_q: i64 = bQR & !wrap_op!(bQR, 1, '-');
        while r_or_q != 0 {
            let r_or_q_idx: usize = r_or_q.leading_zeros() as usize;
            let moves: i64 = self.possibleHAndVMoves(r_or_q_idx);
            unsafe_w |= moves;
            bQR &= !r_or_q; // remove current rook or queen
            r_or_q = bQR & !wrap_op!(bQR, 1, '-');
        }

        // king threats
        let mut king = bK & !wrap_op!(bK, 1, '-');
        let king_span_c7_idx: usize = 10;
        while king != 0 {
            let king_idx: usize = king.leading_zeros() as usize;
            // allign the king_span_c7 mask
            let mut moves: i64 = 0;
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
            bK &= !king; // remove current king
            king = bK & !wrap_op!(bK, 1, '-');
        }
        unsafe_w
    }
}


/// Holds information about all the moves
#[pyclass(module = "ChessProject", get_all, set_all)]
struct Perft {
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


    fn perft(&mut self, mm: &mut Moves, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, EP: i64, cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, depth: u32) {
        if depth < self.max_depth {
            let mut moves: String = String::new();
            if whites_turn {
                moves = mm.possibleMovesW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
            } else {
                moves = mm.possibleMovesB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
            }
            for i in (0..moves.len()).step_by(4) {
                let wPt: i64 = mm.makeMove(wP, moves[i..i+4].to_string(), 'P'); let wNt: i64 = mm.makeMove(wN, moves[i..i+4].to_string(), 'N');
                let wBt: i64 = mm.makeMove(wB, moves[i..i+4].to_string(), 'B'); let wRt: i64 = mm.makeMove(wR, moves[i..i+4].to_string(), 'R');
                let wQt: i64 = mm.makeMove(wQ, moves[i..i+4].to_string(), 'Q'); let wKt: i64 = mm.makeMove(wK, moves[i..i+4].to_string(), 'K');
                let bPt: i64 = mm.makeMove(bP, moves[i..i+4].to_string(), 'p'); let bNt: i64 = mm.makeMove(bN, moves[i..i+4].to_string(), 'n');
                let bBt: i64 = mm.makeMove(bB, moves[i..i+4].to_string(), 'b'); let bRt: i64 = mm.makeMove(bR, moves[i..i+4].to_string(), 'r');
                let bQt: i64 = mm.makeMove(bQ, moves[i..i+4].to_string(), 'q'); let bKt: i64 = mm.makeMove(bK, moves[i..i+4].to_string(), 'k');
                let wRt: i64 = mm.makeMoveCastle(wRt, wK, moves[i..i+4].to_string(), 'R'); let bRt: i64 = mm.makeMoveCastle(bRt, bK, moves[i..i+4].to_string(), 'r');
                let EPt: i64 = mm.makeMoveEP(wP|bP, moves[i..i+4].to_string());

                let mut cwKt: bool = cwK; let mut cwQt: bool = cwQ; let mut cbKt: bool = cbK; let mut cbQt: bool = cbQ;

                if moves.chars().nth(i + 3).unwrap().is_numeric() {
                    let m1: u32 = moves.chars().nth(i).unwrap().to_digit(10).unwrap();
                    let m2: u32 = moves.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                    let m3: u32 = moves.chars().nth(i + 2).unwrap().to_digit(10).unwrap();
                    let m4: u32 = moves.chars().nth(i + 3).unwrap().to_digit(10).unwrap();
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

                if ((wKt & mm.unsafeForWhite(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && whites_turn) || ((bKt & mm.unsafeForBlack(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && !whites_turn) {
                    if depth + 1 == self.max_depth { // only count leaf nodes
                        self.move_counter += 1
                    }
                    // println!("{:?}", self.move_counter);
                    self.perft(mm, wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt, EPt, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth + 1)
                }
            }
        } else if self.move_counter == 0 {
            self.move_counter += 1;
        }
    }


    fn perftRoot(&mut self, mm: &mut Moves, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, EP: i64, cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, depth: u32) {
        let mut moves: String = String::new();
        if whites_turn {
            moves = mm.possibleMovesW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        } else {
            moves = mm.possibleMovesB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        }
        for i in (0..moves.len()).step_by(4) {
            let wPt: i64 = mm.makeMove(wP, moves[i..i+4].to_string(), 'P'); let wNt: i64 = mm.makeMove(wN, moves[i..i+4].to_string(), 'N');
            let wBt: i64 = mm.makeMove(wB, moves[i..i+4].to_string(), 'B'); let wRt: i64 = mm.makeMove(wR, moves[i..i+4].to_string(), 'R');
            let wQt: i64 = mm.makeMove(wQ, moves[i..i+4].to_string(), 'Q'); let wKt: i64 = mm.makeMove(wK, moves[i..i+4].to_string(), 'K');
            let bPt: i64 = mm.makeMove(bP, moves[i..i+4].to_string(), 'p'); let bNt: i64 = mm.makeMove(bN, moves[i..i+4].to_string(), 'n');
            let bBt: i64 = mm.makeMove(bB, moves[i..i+4].to_string(), 'b'); let bRt: i64 = mm.makeMove(bR, moves[i..i+4].to_string(), 'r');
            let bQt: i64 = mm.makeMove(bQ, moves[i..i+4].to_string(), 'q'); let bKt: i64 = mm.makeMove(bK, moves[i..i+4].to_string(), 'k');
            let wRt: i64 = mm.makeMoveCastle(wRt, wK, moves[i..i+4].to_string(), 'R'); let bRt: i64 = mm.makeMoveCastle(bRt, bK, moves[i..i+4].to_string(), 'r');
            let EPt: i64 = mm.makeMoveEP(wP|bP, moves[i..i+4].to_string());

            let mut cwKt: bool = cwK; let mut cwQt: bool = cwQ; let mut cbKt: bool = cbK; let mut cbQt: bool = cbQ;

            if moves.chars().nth(i + 3).unwrap().is_numeric() {
                let m1: u32 = moves.chars().nth(i).unwrap().to_digit(10).unwrap();
                let m2: u32 = moves.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                let m3: u32 = moves.chars().nth(i + 2).unwrap().to_digit(10).unwrap();
                let m4: u32 = moves.chars().nth(i + 3).unwrap().to_digit(10).unwrap();
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

            if ((wKt & mm.unsafeForWhite(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && whites_turn) || ((bKt & mm.unsafeForBlack(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && !whites_turn) {
                self.perft(mm, wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt, EPt, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth + 1);
                println!("{} {}", move_to_algebra!(moves[i..i+4]), self.move_counter);
                self.total_move_counter += self.move_counter;
                self.move_counter = 0;
            }
        }
    }
}


/// Performs a principal variation search
#[pyclass(module = "ChessProject", get_all, set_all)]
struct BestMoveFinder {
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


/// Look into transpositions tables and iterative deepening
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
            return (if whites_turn {1.0} else {-1.0}) * self.evaluate(mm, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, whites_turn);
        }
        // if mm.stalemate {
        //     return self.stale_score as f64;
        // }
        // if mm.checkmate {
        //     return if !whites_turn {(-self.mate_score + depth as i64) as f64} else {(self.mate_score - depth as i64) as f64};
        // }
        let mut best_score: f64 = -self.mate_score as f64;
        // TODO look at pawn promo score calc is good, seems to not add 9 to score

        // let mut moves: String = String::new();
        // if whites_turn {
        //     moves = mm.possibleMovesW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        // } else {
        //     moves = mm.possibleMovesB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        // }
        // if depth == 0 {
        //     self.considered_moves = moves.clone();
        //     let mut move_groups: Vec<&str> = moves.as_bytes().chunks(4).map(|chunk| from_utf8(chunk).unwrap()).collect();
        //     move_groups.shuffle(&mut thread_rng());
        //     moves = move_groups.join("");

        // }
        // TODO: change to getValidMoves
        // then add stalemant and checkmate if statements
        // reward earlier checkmates
        // let mut legal_move_idx: i64 = self.getNextLegalMoveIdx(&moves, mm, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ, whites_turn, 0);
        // while legal_move_idx != -1 {
        let valid_moves: String = mm.getValidMoves(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ, whites_turn, depth);
        if mm.stalemate {
            return self.stale_score as f64;
        }
        for i in (0..valid_moves.len()).step_by(4) {
            let mut wPt: i64 = mm.makeMove(wP, valid_moves[i..i+4].to_string(), 'P'); let mut wNt: i64 = mm.makeMove(wN, valid_moves[i..i+4].to_string(), 'N');
            let mut wBt: i64 = mm.makeMove(wB, valid_moves[i..i+4].to_string(), 'B'); let mut wRt: i64 = mm.makeMove(wR, valid_moves[i..i+4].to_string(), 'R');
            let mut wQt: i64 = mm.makeMove(wQ, valid_moves[i..i+4].to_string(), 'Q'); let mut wKt: i64 = mm.makeMove(wK, valid_moves[i..i+4].to_string(), 'K');
            let mut bPt: i64 = mm.makeMove(bP, valid_moves[i..i+4].to_string(), 'p'); let mut bNt: i64 = mm.makeMove(bN, valid_moves[i..i+4].to_string(), 'n');
            let mut bBt: i64 = mm.makeMove(bB, valid_moves[i..i+4].to_string(), 'b'); let mut bRt: i64 = mm.makeMove(bR, valid_moves[i..i+4].to_string(), 'r');
            let mut bQt: i64 = mm.makeMove(bQ, valid_moves[i..i+4].to_string(), 'q'); let mut bKt: i64 = mm.makeMove(bK, valid_moves[i..i+4].to_string(), 'k');
            let mut wRt: i64 = mm.makeMoveCastle(wRt, wK, valid_moves[i..i+4].to_string(), 'R'); let mut bRt: i64 = mm.makeMoveCastle(bRt, bK, valid_moves[i..i+4].to_string(), 'r');
            let mut EPt: i64 = mm.makeMoveEP(wP|bP, valid_moves[i..i+4].to_string());

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
                    println!("Considering {:?} with score: {:?}", valid_moves[i..i+4].to_string(), score);
                }
            }

            if best_score > alpha {
                alpha = best_score;
            }
            if alpha >= beta {
                break;
            }
            // legal_move_idx = self.getNextLegalMoveIdx(&moves, mm, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ, whites_turn, (legal_move_idx+4) as usize);
        }
        best_score
    }


    fn pvSearch(&mut self, mut alpha: f64, beta: f64, mm: &mut Moves, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, EP: i64, cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, depth: u32) -> f64 {
        let mut best_score: f64;
        if depth == self.search_depth {
            best_score = self.evaluate(mm, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, whites_turn);
            return best_score;
        }
        let mut moves: String = String::new();
        if whites_turn {
            moves = mm.possibleMovesW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        } else {
            moves = mm.possibleMovesB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        }
        if depth == 0 {
            self.considered_moves = moves.clone();
        }
        // get first move
        let mut first_legal_move: i64 = self.getNextLegalMoveIdx(&moves, mm, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ, whites_turn, 0); // diff
        if first_legal_move == -1 {
            return if whites_turn {self.mate_score as f64} else {-self.mate_score as f64};
        }
        // make first move
        let mut wPt: i64 = mm.makeMove(wP, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'P'); let mut wNt: i64 = mm.makeMove(wN, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'N');
        let mut wBt: i64 = mm.makeMove(wB, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'B'); let mut wRt: i64 = mm.makeMove(wR, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'R');
        let mut wQt: i64 = mm.makeMove(wQ, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'Q'); let mut wKt: i64 = mm.makeMove(wK, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'K');
        let mut bPt: i64 = mm.makeMove(bP, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'p'); let mut bNt: i64 = mm.makeMove(bN, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'n');
        let mut bBt: i64 = mm.makeMove(bB, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'b'); let mut bRt: i64 = mm.makeMove(bR, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'r');
        let mut bQt: i64 = mm.makeMove(bQ, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'q'); let mut bKt: i64 = mm.makeMove(bK, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'k');
        let mut wRt: i64 = mm.makeMoveCastle(wRt, wK, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'R'); let mut bRt: i64 = mm.makeMoveCastle(bRt, bK, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string(), 'r');
        let mut EPt: i64 = mm.makeMoveEP(wP|bP, moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string());

        let mut cwKt: bool = cwK; let mut cwQt: bool = cwQ; let mut cbKt: bool = cbK; let mut cbQt: bool = cbQ;

        if moves.chars().nth((first_legal_move as usize) + 3).unwrap().is_numeric() {
            let m1: u32 = moves.chars().nth(first_legal_move as usize).unwrap().to_digit(10).unwrap();
            let m2: u32 = moves.chars().nth((first_legal_move as usize) + 1).unwrap().to_digit(10).unwrap();
            let m3: u32 = moves.chars().nth((first_legal_move as usize) + 2).unwrap().to_digit(10).unwrap();
            let m4: u32 = moves.chars().nth((first_legal_move as usize) + 3).unwrap().to_digit(10).unwrap();
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
        // evaluate new position
        best_score = -self.pvSearch(-beta, -alpha, mm, wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt, EPt, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth+1);
        self.move_counter += 1;
        if best_score.abs() == (self.mate_score as f64) {
            // self.next_move = moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string();
            return best_score;
        }
        if best_score > alpha {
            if best_score >= beta {
                // self.next_move = moves[(first_legal_move as usize)..(first_legal_move as usize)+4].to_string();
                return best_score;
            }
            alpha = best_score;
        }
        // set best score so far
        self.best_move_idx = first_legal_move;
        // search subsequent moves using zero window search
        for i in (((first_legal_move + 4) as usize)..moves.len()).step_by(4) {
            let mut score: f64;
            self.move_counter += 1;
            // legal non-castle moves
            wPt = mm.makeMove(wP, moves[i..i+4].to_string(), 'P'); wNt = mm.makeMove(wN, moves[i..i+4].to_string(), 'N');
            wBt = mm.makeMove(wB, moves[i..i+4].to_string(), 'B'); wRt = mm.makeMove(wR, moves[i..i+4].to_string(), 'R');
            wQt = mm.makeMove(wQ, moves[i..i+4].to_string(), 'Q'); wKt = mm.makeMove(wK, moves[i..i+4].to_string(), 'K');
            bPt = mm.makeMove(bP, moves[i..i+4].to_string(), 'p'); bNt = mm.makeMove(bN, moves[i..i+4].to_string(), 'n');
            bBt = mm.makeMove(bB, moves[i..i+4].to_string(), 'b'); bRt = mm.makeMove(bR, moves[i..i+4].to_string(), 'r');
            bQt = mm.makeMove(bQ, moves[i..i+4].to_string(), 'q'); bKt = mm.makeMove(bK, moves[i..i+4].to_string(), 'k');
            wRt = mm.makeMoveCastle(wRt, wK, moves[i..i+4].to_string(), 'R'); bRt = mm.makeMoveCastle(bRt, bK, moves[i..i+4].to_string(), 'r');
            EPt = mm.makeMoveEP(wP|bP, moves[i..i+4].to_string());

            cwKt = cwK; cwQt = cwQ; cbKt = cbK; cbQt = cbQ;

            if moves.chars().nth(i + 3).unwrap().is_numeric() {
                let m1: u32 = moves.chars().nth(first_legal_move as usize).unwrap().to_digit(10).unwrap();
                let m2: u32 = moves.chars().nth((first_legal_move as usize) + 1).unwrap().to_digit(10).unwrap();
                let m3: u32 = moves.chars().nth((first_legal_move as usize) + 2).unwrap().to_digit(10).unwrap();
                let m4: u32 = moves.chars().nth((first_legal_move as usize) + 3).unwrap().to_digit(10).unwrap();
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
            // quick search to check if move could be best
            score = -self.zWSearch(-alpha, mm, wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt, EPt, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth+1);
            if (score > alpha) && (score < beta) {
                // re-search with window [alpha: beta] because move could be best move
                best_score = -self.pvSearch(-beta, -alpha, mm, wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt, EPt, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth+1);
                if score > alpha {
                    self.best_move_idx = i as i64;
                    // self.next_move = moves[i..i+4].to_string();
                    alpha = score;
                }
            }
            if (score != f64::MIN) && (score > best_score) {
                if score >= beta {
                    // self.next_move = moves[i..i+4].to_string();
                    return score;
                }
                best_score = score;
                if best_score.abs() == (self.mate_score as f64) {
                    // self.next_move = moves[i..i+4].to_string();
                    return best_score;
                }
            }
        }
        best_score
    }


    fn evaluate(&self, mm: &Moves, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, whites_turn: bool) -> f64 {
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


    fn getNextLegalMoveIdx(&self, moves: &str, mm: &mut Moves, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, EP: i64, cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, start_idx: usize) -> i64 {
        for i in (start_idx..moves.len()).step_by(4) {
            let wPt: i64 = mm.makeMove(wP, moves[i..i+4].to_string(), 'P'); let wNt: i64 = mm.makeMove(wN, moves[i..i+4].to_string(), 'N');
            let wBt: i64 = mm.makeMove(wB, moves[i..i+4].to_string(), 'B'); let wRt: i64 = mm.makeMove(wR, moves[i..i+4].to_string(), 'R');
            let wQt: i64 = mm.makeMove(wQ, moves[i..i+4].to_string(), 'Q'); let wKt: i64 = mm.makeMove(wK, moves[i..i+4].to_string(), 'K');
            let bPt: i64 = mm.makeMove(bP, moves[i..i+4].to_string(), 'p'); let bNt: i64 = mm.makeMove(bN, moves[i..i+4].to_string(), 'n');
            let bBt: i64 = mm.makeMove(bB, moves[i..i+4].to_string(), 'b'); let bRt: i64 = mm.makeMove(bR, moves[i..i+4].to_string(), 'r');
            let bQt: i64 = mm.makeMove(bQ, moves[i..i+4].to_string(), 'q'); let bKt: i64 = mm.makeMove(bK, moves[i..i+4].to_string(), 'k');
            let wRt: i64 = mm.makeMoveCastle(wRt, wK, moves[i..i+4].to_string(), 'R'); let bRt: i64 = mm.makeMoveCastle(bRt, bK, moves[i..i+4].to_string(), 'r');
            if ((wKt & mm.unsafeForWhite(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && whites_turn) || ((bKt & mm.unsafeForBlack(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && !whites_turn) {
                return i as i64;
            }
        }
        -1
    }


    fn zWSearch(&self, beta: f64, mm: &mut Moves, wP: i64, wN: i64, wB: i64, wR: i64, wQ: i64, wK: i64, bP: i64, bN: i64, bB: i64, bR: i64, bQ: i64, bK: i64, EP: i64, cwK: bool, cwQ: bool, cbK: bool, cbQ: bool, whites_turn: bool, depth: u32) -> f64 {
        let mut score: f64 = f64::MIN;
        // alpha == beta - 1
        if depth == self.search_depth {
            score = self.evaluate(mm, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, whites_turn);
            return score;
        }
        let mut moves: String = String::new();
        if whites_turn {
            moves = mm.possibleMovesW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        } else {
            moves = mm.possibleMovesB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ);
        }
        for i in (0..moves.len()).step_by(4) {
            let wPt: i64 = mm.makeMove(wP, moves[i..i+4].to_string(), 'P'); let wNt: i64 = mm.makeMove(wN, moves[i..i+4].to_string(), 'N');
            let wBt: i64 = mm.makeMove(wB, moves[i..i+4].to_string(), 'B'); let wRt: i64 = mm.makeMove(wR, moves[i..i+4].to_string(), 'R');
            let wQt: i64 = mm.makeMove(wQ, moves[i..i+4].to_string(), 'Q'); let wKt: i64 = mm.makeMove(wK, moves[i..i+4].to_string(), 'K');
            let bPt: i64 = mm.makeMove(bP, moves[i..i+4].to_string(), 'p'); let bNt: i64 = mm.makeMove(bN, moves[i..i+4].to_string(), 'n');
            let bBt: i64 = mm.makeMove(bB, moves[i..i+4].to_string(), 'b'); let bRt: i64 = mm.makeMove(bR, moves[i..i+4].to_string(), 'r');
            let bQt: i64 = mm.makeMove(bQ, moves[i..i+4].to_string(), 'q'); let bKt: i64 = mm.makeMove(bK, moves[i..i+4].to_string(), 'k');
            let wRt: i64 = mm.makeMoveCastle(wRt, wK, moves[i..i+4].to_string(), 'R'); let bRt: i64 = mm.makeMoveCastle(bRt, bK, moves[i..i+4].to_string(), 'r');
            let EPt: i64 = mm.makeMoveEP(wP|bP, moves[i..i+4].to_string());

            let mut cwKt: bool = cwK; let mut cwQt: bool = cwQ; let mut cbKt: bool = cbK; let mut cbQt: bool = cbQ;

            if moves.chars().nth(i + 3).unwrap().is_numeric() {
                let m1: u32 = moves.chars().nth(i).unwrap().to_digit(10).unwrap();
                let m2: u32 = moves.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                let m3: u32 = moves.chars().nth(i + 2).unwrap().to_digit(10).unwrap();
                let m4: u32 = moves.chars().nth(i + 3).unwrap().to_digit(10).unwrap();
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
            if ((wKt & mm.unsafeForWhite(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && whites_turn) || ((bKt & mm.unsafeForBlack(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 && !whites_turn) {
                score = -self.zWSearch(1.0-beta, mm, wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt, EPt, cwKt, cwQt, cbKt, cbQt, !whites_turn, depth+1);
            }
            if score >= beta {
                return score; // fail hard beta cutoff
            }
        }
        return beta - 1.0; // fail hard, return alpha
    }
}


/// Macro to draw a bitboard
#[macro_export]
macro_rules! draw_array {
    ($bitboard:expr) => {
        let mut new_board: [[char; 8]; 8] = [['0'; 8]; 8];
        for i in 0..64 {
            let shift = 64 - 1 - i;
            if usgn_r_shift!($bitboard, shift) & 1 == 1 {
                new_board[i / 8][i % 8] = '1';
            }
        }
        for row in 0..8 {
            for col in 0..8 {
                print!("{}", new_board[row][col]);
            }
            println!();
        }
        println!();
    };
}


/// Macro to perform 64 bit unsigned right shift
#[macro_export]
macro_rules! usgn_r_shift {
    ($lv:expr, $rv:expr) => {
        (($lv as u64) >> $rv) as i64
    };
}


/// Macro to convert i64 to binary string with 0 padding
#[macro_export]
macro_rules! as_bin_str {
    ($int64:expr) => {
        format!("{:064b}", $int64)
    };
}


/// Macro to perform wrapping operations
#[macro_export]
macro_rules! wrap_op {
    ($lv:expr, '!') => {
        $lv.wrapping_neg()
    };

    ($lv:expr, $rv:expr, $op:expr) => {
        match $op {
            '+' => $lv.wrapping_add($rv),
            '-' => $lv.wrapping_sub($rv),
            '*' => $lv.wrapping_mul($rv),
            _ => panic!("Wrapping operation not possible"),
        }
    };
}


/// Macro to convert move string to algebra notation
#[macro_export]
macro_rules! move_to_algebra {
    ($move:expr) => {{
        let mut move_str: String = String::new();
        let idx_to_file_ascii_shift: u8 = 49;
        let move_chars: Vec<char> = $move.chars().collect();
        if move_chars[3] == 'E' {
            move_str.push((move_chars[0] as u8 + idx_to_file_ascii_shift) as char);
            move_str.push(if move_chars[2] == 'w' {'5'} else {'4'});
            move_str.push((move_chars[1] as u8 + idx_to_file_ascii_shift) as char);
            move_str.push(if move_chars[2] == 'w' {'6'} else {'3'});
        } else if move_chars[3] == 'P' {
            move_str.push((move_chars[0] as u8 + idx_to_file_ascii_shift) as char);
            move_str.push(if move_chars[2].is_uppercase() {'7'} else {'2'});
            move_str.push((move_chars[1] as u8 + idx_to_file_ascii_shift) as char);
            move_str.push(if move_chars[2].is_uppercase() {'8'} else {'1'});
            move_str.push(move_chars[2]);
        } else {
            move_str.push((move_chars[1] as u8 + idx_to_file_ascii_shift) as char);
            move_str.push((('8' as u8 - move_chars[0] as u8) + '0' as u8) as char);
            move_str.push((move_chars[3] as u8 + idx_to_file_ascii_shift) as char);
            move_str.push((('8' as u8 - move_chars[2] as u8) + '0' as u8) as char);
        }
        move_str
    }};
}


/// Macro to add classes to PyModule
macro_rules! add_classes {
    ($module:ident, $($class:ty),+) => {
        $(
            $module.add_class::<$class>()?;
        )+
    };
}


/// Macro to add functions to PyModule
macro_rules! add_functions {
    ($module:ident, $($function:ident),+) => {
        $(
            $module.add_wrapped(wrap_pyfunction!($function))?;
        )+
    };
}


/// A Python module implemented in Rust.
#[pymodule]
fn ChessProject(_py: Python, m: &PyModule) -> PyResult<()> {
    add_classes!(m, SpecialBitBoards, GameState, Moves, Perft, BestMoveFinder);
    Ok(())
}


/// Tests


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        println!("Basic Test!");
        let mut gs = GameState::new();
        // gs.importFEN(String::from("rnbqkbnr/pppp1ppp/8/4p3/5PP1/8/PPPPP2P/RNBQKBNR b KQkq g3 0 2"));
        gs.importFEN(String::from("1R6/PK4r1/8/8/1p6/1P4k1/1P6/8 w - - 0 1"));

        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(3);
        let mut bmf: BestMoveFinder = BestMoveFinder::new(3);
        // let moves: String = m.getValidMoves(gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, gs.whites_turn);
        // let x: i64 = bmf.negaMaxAlphaBeta(-5000, 5000, &mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        // println!("{:?}", x);
        // println!("{:?}", bmf.considered_moves);
        // for i in 0..4 {
        //     print!("{:?}", bmf.considered_moves.chars().nth(bmf.best_move_idx as usize + i).unwrap());
        // }
        // println!("");
        // println!("{:?}", bmf.move_counter);
        // println!("{:?}", bmf.best_move_idx);
        // let before = Instant::now();
        // p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        // println!("Elapsed time: {:.2?}", before.elapsed());
        // println!("Total Moves: {:?}", p.total_move_counter);
        println!("DONE!");
        panic!();
    }

    #[test]
    fn perft_starting_pos() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 4865609);
    }

    #[test]
    fn perft_complex_pos() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 4085603);
    }

    #[test]
    fn perft_wikispaces1() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 11030083);
    }

    #[test]
    fn perft_wikispaces2() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        gs.importFEN(String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 15833292);
    }

    #[test]
    fn perft_wikispaces3() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        gs.importFEN(String::from("1k6/1b6/8/8/7R/8/8/4K2R b K - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1063513);
    }

    #[test]
    fn perft_illegal_ep1() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1134888);
    }

    #[test]
    fn perft_illegal_ep2() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 1015133);
    }

    #[test]
    fn perft_ep_capture_checks_opponent() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1440467);
    }

    #[test]
    fn perft_short_castling_gives_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("5k2/8/8/8/8/8/8/4K2R w K - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 661072);
    }

    #[test]
    fn perft_long_castling_gives_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 803711);
    }

    #[test]
    fn perft_castle_rights() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(String::from("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 1274206);
    }

    #[test]
    fn perft_castling_prevented() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(String::from("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1720476);
    }

    #[test]
    fn perft_promote_out_of_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 3821001);
    }

    #[test]
    fn perft_discovered_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(5);
        gs.importFEN(String::from("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 1004658);
    }

    #[test]
    fn perft_promote_to_give_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("4k3/1P6/8/8/8/8/K7/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 217342);
    }

    #[test]
    fn perft_under_promote_to_give_check() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("8/P1k5/K7/8/8/8/8/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 92683);
    }

    #[test]
    fn perft_self_stalemate() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(6);
        gs.importFEN(String::from("K1k5/8/P7/8/8/8/8/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 2217);
    }

    #[test]
    fn perft_stalemate_and_checkmate1() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(7);
        gs.importFEN(String::from("8/k1P5/8/1K6/8/8/8/8 w - - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, true, 0);
        assert!(p.total_move_counter == 567584);
    }

    #[test]
    fn perft_stalemate_and_checkmate2() {
        let mut gs = GameState::new();
        let mut m: Moves = Moves::new();
        let mut p: Perft = Perft::new(4);
        gs.importFEN(String::from("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1"));
        p.perftRoot(&mut m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, false, 0);
        assert!(p.total_move_counter == 23527);
    }
}
