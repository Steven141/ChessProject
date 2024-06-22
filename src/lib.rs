//! Chess Engine Library


#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_assignments)]


use pyo3::prelude::*;


/// EXAMPLE: Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


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
            if (self.wP >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'P';
            }
            if (self.wN >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'N';
            }
            if (self.wB >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'B';
            }
            if (self.wR >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'R';
            }
            if (self.wQ >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'Q';
            }
            if (self.wK >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'K';
            }
            if (self.bP >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'p';
            }
            if (self.bN >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'n';
            }
            if (self.bB >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'b';
            }
            if (self.bR >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'r';
            }
            if (self.bQ >> shift) & 1 == 1 {
                new_board[i / 8][i % 8] = 'q';
            }
            if (self.bK >> shift) & 1 == 1 {
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
        // Rest of FEN not used
    }
}


/// Holds information about all the moves
#[pyclass(module = "ChessProject", get_all, set_all)]
struct Moves {
    castle_rooks: [usize; 4],
    masks: SpecialBitBoards,
}


#[pymethods]
impl Moves {
    #[new]
    fn new() -> Self {
        Moves {
            castle_rooks: [63, 56, 7, 0],
            masks: SpecialBitBoards::new(),
        }
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
    ($lv:expr, $rv:expr, $op:expr) => {
        match $op {
            '+' => $lv.wrapping_add($rv),
            '-' => $lv.wrapping_sub($rv),
            '*' => $lv.wrapping_mul($rv),
            '!' => $lv.wrapping_neg(),
            _ => panic!("Wrapping operation not possible"),
        }
    };
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
    add_functions!(m, sum_as_string);
    add_classes!(m, SpecialBitBoards, GameState, Moves);
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
        // gs.drawGameArray();
        // gs.importFEN(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -"));
        // gs.drawGameArray();

        // gs.bR = usgn_r_shift!(gs.bR, 24);
        // gs.drawGameArray();
        // draw_array!(gs.bR);

        let mut m: Moves = Moves::new();
        let q: i64 = m.possibleHAndVMoves(0);
        draw_array!(q);
        println!("DONE!");
        panic!();
    }
}
