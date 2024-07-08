//! Module to keeps track of game state and game progressing functions


use pyo3::prelude::*;
use crate::special_bitboards::SpecialBitBoards;
use crate::moves::Moves;
use crate::piece::Piece;


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct GameState {
    board: [[char; 8]; 8],
    pub bitboards: [i64; 13],
    pub cwK: bool,
    pub cwQ: bool,
    pub cbK: bool,
    pub cbQ: bool,
    whites_turn: bool,
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
            bitboards: [0; 13],
            cwK: true,
            cwQ: true,
            cbK: true,
            cbQ: true,
            whites_turn: true,
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
                'P' => self.bitboards[Piece::WP] += self.binToI64(&bin_str),
                'N' => self.bitboards[Piece::WN] += self.binToI64(&bin_str),
                'B' => self.bitboards[Piece::WB] += self.binToI64(&bin_str),
                'R' => self.bitboards[Piece::WR] += self.binToI64(&bin_str),
                'Q' => self.bitboards[Piece::WQ] += self.binToI64(&bin_str),
                'K' => self.bitboards[Piece::WK] += self.binToI64(&bin_str),
                'p' => self.bitboards[Piece::BP] += self.binToI64(&bin_str),
                'n' => self.bitboards[Piece::BN] += self.binToI64(&bin_str),
                'b' => self.bitboards[Piece::BB] += self.binToI64(&bin_str),
                'r' => self.bitboards[Piece::BR] += self.binToI64(&bin_str),
                'q' => self.bitboards[Piece::BQ] += self.binToI64(&bin_str),
                'k' => self.bitboards[Piece::BK] += self.binToI64(&bin_str),
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
            if usgn_r_shift!(self.bitboards[Piece::WP], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'P';
            }
            if usgn_r_shift!(self.bitboards[Piece::WN], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'N';
            }
            if usgn_r_shift!(self.bitboards[Piece::WB], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'B';
            }
            if usgn_r_shift!(self.bitboards[Piece::WR], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'R';
            }
            if usgn_r_shift!(self.bitboards[Piece::WQ], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'Q';
            }
            if usgn_r_shift!(self.bitboards[Piece::WK], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'K';
            }
            if usgn_r_shift!(self.bitboards[Piece::BP], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'p';
            }
            if usgn_r_shift!(self.bitboards[Piece::BN], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'n';
            }
            if usgn_r_shift!(self.bitboards[Piece::BB], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'b';
            }
            if usgn_r_shift!(self.bitboards[Piece::BR], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'r';
            }
            if usgn_r_shift!(self.bitboards[Piece::BQ], shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'q';
            }
            if usgn_r_shift!(self.bitboards[Piece::BK], shift) & 1 == 1 {
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
            if usgn_r_shift!(self.bitboards[Piece::WP], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'P';
            }
            if usgn_r_shift!(self.bitboards[Piece::WN], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'N';
            }
            if usgn_r_shift!(self.bitboards[Piece::WB], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'B';
            }
            if usgn_r_shift!(self.bitboards[Piece::WR], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'R';
            }
            if usgn_r_shift!(self.bitboards[Piece::WQ], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'Q';
            }
            if usgn_r_shift!(self.bitboards[Piece::WK], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'K';
            }
            if usgn_r_shift!(self.bitboards[Piece::BP], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'p';
            }
            if usgn_r_shift!(self.bitboards[Piece::BN], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'n';
            }
            if usgn_r_shift!(self.bitboards[Piece::BB], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'b';
            }
            if usgn_r_shift!(self.bitboards[Piece::BR], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'r';
            }
            if usgn_r_shift!(self.bitboards[Piece::BQ], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'q';
            }
            if usgn_r_shift!(self.bitboards[Piece::BK], shift) & 1 == 1 {
                self.board[i / 8][i % 8] = 'k';
            }
        }
    }


    pub fn importFEN(&mut self, fen_str: String) {
        self.bitboards[Piece::WP] = 0; self.bitboards[Piece::WN] = 0; self.bitboards[Piece::WB] = 0;
        self.bitboards[Piece::WR] = 0; self.bitboards[Piece::WQ] = 0; self.bitboards[Piece::WK] = 0;
        self.bitboards[Piece::BP] = 0; self.bitboards[Piece::BN] = 0; self.bitboards[Piece::BB] = 0;
        self.bitboards[Piece::BR] = 0; self.bitboards[Piece::BQ] = 0; self.bitboards[Piece::BK] = 0;
        self.cwK = false; self.cwQ = false;
        self.cbK = false; self.cbQ = false;
        let mut char_idx: usize = 0;
        let mut board_idx: i64 = 0;
        while fen_str.chars().nth(char_idx).unwrap() != ' ' {
            let board_idx_shift: i64 = 64 - 1 - board_idx;
            match fen_str.chars().nth(char_idx).unwrap() {
                'P' => {
                    self.bitboards[Piece::WP] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'N' => {
                    self.bitboards[Piece::WN] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'B' => {
                    self.bitboards[Piece::WB] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'R' => {
                    self.bitboards[Piece::WR] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'Q' => {
                    self.bitboards[Piece::WQ] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'K' => {
                    self.bitboards[Piece::WK] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'p' => {
                    self.bitboards[Piece::BP] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'n' => {
                    self.bitboards[Piece::BN] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'b' => {
                    self.bitboards[Piece::BB] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'r' => {
                    self.bitboards[Piece::BR] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'q' => {
                    self.bitboards[Piece::BQ] |= 1 << board_idx_shift;
                    board_idx += 1;
                },
                'k' => {
                    self.bitboards[Piece::BK] |= 1 << board_idx_shift;
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
            self.bitboards[Piece::EP] = self.masks.file_masks[fen_str.chars().nth(char_idx).unwrap() as usize - 'a' as usize];
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
        let wK_cached: i64 = self.bitboards[Piece::WK];
        let bK_cached: i64 = self.bitboards[Piece::BK];
        let wR_cached: i64 = self.bitboards[Piece::WR];
        let bR_cached: i64 = self.bitboards[Piece::BR];
        let wP_cached: i64 = self.bitboards[Piece::WP];
        let bP_cached: i64 = self.bitboards[Piece::BP];

        self.bitboards[Piece::WP] = mm.makeMove(self.bitboards[Piece::WP], move_str.clone(), 'P'); self.bitboards[Piece::WN] = mm.makeMove(self.bitboards[Piece::WN], move_str.clone(), 'N');
        self.bitboards[Piece::WB] = mm.makeMove(self.bitboards[Piece::WB], move_str.clone(), 'B'); self.bitboards[Piece::WR] = mm.makeMove(self.bitboards[Piece::WR], move_str.clone(), 'R');
        self.bitboards[Piece::WQ] = mm.makeMove(self.bitboards[Piece::WQ], move_str.clone(), 'Q'); self.bitboards[Piece::WK] = mm.makeMove(self.bitboards[Piece::WK], move_str.clone(), 'K');
        self.bitboards[Piece::BP] = mm.makeMove(self.bitboards[Piece::BP], move_str.clone(), 'p'); self.bitboards[Piece::BN] = mm.makeMove(self.bitboards[Piece::BN], move_str.clone(), 'n');
        self.bitboards[Piece::BB] = mm.makeMove(self.bitboards[Piece::BB], move_str.clone(), 'b'); self.bitboards[Piece::BR] = mm.makeMove(self.bitboards[Piece::BR], move_str.clone(), 'r');
        self.bitboards[Piece::BQ] = mm.makeMove(self.bitboards[Piece::BQ], move_str.clone(), 'q'); self.bitboards[Piece::BK] = mm.makeMove(self.bitboards[Piece::BK], move_str.clone(), 'k');
        self.bitboards[Piece::WR] = mm.makeMoveCastle(self.bitboards[Piece::WR], wK_cached, move_str.clone(), 'R'); self.bitboards[Piece::BR] = mm.makeMoveCastle(self.bitboards[Piece::BR], bK_cached, move_str.clone(), 'r');
        self.bitboards[Piece::EP] = mm.makeMoveEP(wP_cached|bP_cached, move_str.clone());

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