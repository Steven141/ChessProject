//! Module to keeps track of game state and game progressing functions


use pyo3::prelude::*;
use crate::engine_modules::special_bitboards::SpecialBitBoards;
use crate::engine_modules::moves::Moves;


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct GameState {
    board: [[char; 8]; 8],
    pub cwK: bool,
    pub cwQ: bool,
    pub cbK: bool,
    pub cbQ: bool,
    whites_turn: bool,
    pub wP: i64,
    pub wN: i64,
    pub wB: i64,
    pub wR: i64,
    pub wQ: i64,
    pub wK: i64,
    pub bP: i64,
    pub bN: i64,
    pub bB: i64,
    pub bR: i64,
    pub bQ: i64,
    pub bK: i64,
    pub EP: i64,
    masks: SpecialBitBoards,
    move_log: String,
    recent_piece_moved: char,
    recent_piece_captured: char,
}


#[pymethods]
impl GameState {
    #[new]
    pub fn new() -> Self {
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


    pub fn importFEN(&mut self, fen_str: String) {
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
                    self.wP |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'N' => {
                    self.wN |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'B' => {
                    self.wB |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'R' => {
                    self.wR |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'Q' => {
                    self.wQ |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'K' => {
                    self.wK |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'p' => {
                    self.bP |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'n' => {
                    self.bN |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'b' => {
                    self.bB |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'r' => {
                    self.bR |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'q' => {
                    self.bQ |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'k' => {
                    self.bK |= 1 << board_idx_shift;
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