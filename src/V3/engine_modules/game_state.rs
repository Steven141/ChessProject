//! Module to keeps track of game state and game progressing functions


use pyo3::prelude::*;
use crate::castle_rights::CastleRights;
use crate::special_bitboards::SpecialBitBoards;
use crate::moves::Moves;
use crate::piece::Piece;


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct GameState {
    board: [[char; 8]; 8],
    pub bitboards: [i64; 13],
    pub castle_rights: [bool; 4],
    pub whites_turn: bool,
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
            castle_rights: [true; 4],
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
        self.bitboards = [0; 13];
        self.castle_rights = [false; 4];
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
                'K' => self.castle_rights[CastleRights::CWK] = true,
                'Q' => self.castle_rights[CastleRights::CWQ] = true,
                'k' => self.castle_rights[CastleRights::CBK] = true,
                'q' => self.castle_rights[CastleRights::CBQ] = true,
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


    fn makeMove(&mut self, mm: &Moves, move_str: String) {
        if move_str.chars().nth(3).unwrap() == 'E' {
            self.recent_piece_captured = if self.whites_turn {'p'} else {'P'};
            self.recent_piece_moved = if self.whites_turn {'P'} else {'p'};
        } else if move_str.chars().nth(3).unwrap() == 'P' {
            let (_, c2, _, _) = move_to_u32s!(move_str);
            self.recent_piece_captured = self.board[if self.whites_turn {0} else {7}][c2 as usize];
            self.recent_piece_moved = if self.whites_turn {'P'} else {'p'};
        } else {
            let (r1, c1, r2, c2) = move_to_u32s!(move_str);
            self.recent_piece_captured = self.board[r2 as usize][c2 as usize];
            self.recent_piece_moved = self.board[r1 as usize][c1 as usize];
        }

        self.move_log.push_str(&move_str);
        let bitboards_cached: [i64; 13] = self.bitboards;
        self.bitboards = mm.getUpdatedBitboards(&move_str, self.bitboards);
        self.castle_rights = mm.getUpdatedCastleRights(&move_str, self.castle_rights, bitboards_cached);

        self.whites_turn = !self.whites_turn;
        self.updateBoardArray();
    }
}