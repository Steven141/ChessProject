//! Holds all project macros


/*
Macro to set a bit in a bitboard.
square is 0..64 where 0 is top-left.
*/
#[macro_export]
macro_rules! set_bit {
    ($bitboard:expr, $square:expr) => {
        $bitboard |= 1u64 << (63 - $square);
    };
}


/*
Macro to set multiple bits in a bitboard.
bits are represented by another bitboard.
*/
#[macro_export]
macro_rules! set_bits {
    ($bitboard:expr, $bits:expr) => {
        $bitboard |= $bits;
    };
}


/*
Macro to remove a bit in a bitboard.
square is 0..64 where 0 is top-left.
*/
#[macro_export]
macro_rules! pop_bit {
    ($bitboard:expr, $square:expr) => {
        $bitboard &= !(1u64 << (63 - $square));
    };
}


/*
Macro to remove multiple bits in a bitboard.
bits are represented by another bitboard.
*/
#[macro_export]
macro_rules! pop_bits {
    ($bitboard:expr, $bits:expr) => {
        $bitboard &= !$bits;
    };
}


/*
Macro to get a bit from bitboard.
square is 0..64 where 0 is top-left.
*/
#[macro_export]
macro_rules! get_bit {
    ($bitboard:expr, $square:expr) => {
        ($bitboard >> (63 - $square)) & 1
    };
}


/// Macro to get the least significant '1' bit from bitboard
#[macro_export]
macro_rules! get_ls1b {
    ($bitboard:expr) => {
        $bitboard & !wrap_op!($bitboard, 1, '-')
    };
}


/*
Macro to bitwise or elements of an array where elements
are selected with an input array of indices
*/
#[macro_export]
macro_rules! or_array_elems {
    ($idxs:expr, $array:expr) => {
        $idxs.iter()
            .map(|&i| $array[i])
            .fold(0, |acc, x| acc | x)
    };
}


/// Macro to draw a bitboard
#[macro_export]
macro_rules! draw_array {
    ($bitboard:expr) => {
        let mut new_board: [[char; 8]; 8] = [['0'; 8]; 8];
        for i in 0..64 {
            let shift = 64 - 1 - i;
            if ($bitboard >> shift) & 1 == 1 {
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


/*
Macro to perform wrapping operations. Used because bits
in overflow positions are not cared about in bitboard context.
Rust lib is compiled in release mode which does not have overflow
checks so we want to make sure no undefined behavior.
*/
#[macro_export]
macro_rules! wrap_op {
    ($lv:expr, $rv:expr, $op:expr) => {
        match $op {
            '+' => $lv.wrapping_add($rv),
            '-' => $lv.wrapping_sub($rv),
            '*' => $lv.wrapping_mul($rv),
            _ => panic!("Wrapping operation not possible"),
        }
    };
}


/*
Macro to get the pieces moved in a move.
Starting square piece is always returned.
If no piece captured then Piece::EP returned.
*/
#[macro_export]
macro_rules! get_move_pieces {
    ($bitboards:expr, $move:expr) => {{
        let (r1, c1, r2, c2) = move_to_u32s!($move);
        let start_sq: u32 = r1 * 8 + c1;
        let end_sq: u32 = r2 * 8 + c2;
        let pieces: [Piece; 12] = Piece::allPieces();
        let mut start_piece: Piece = Piece::EP; // default for no piece captured
        let mut end_piece: Piece = Piece::EP; // default for no piece captured
        for piece in pieces {
            if get_bit!($bitboards[piece], start_sq) == 1 {
                start_piece = piece;
            }
            if get_bit!($bitboards[piece], end_sq) == 1 {
                end_piece = piece;
            }
        }
        (start_piece, end_piece)
    }};
}


/// Macro to convert the 4 characters of a move string as u32 rows and cols
#[macro_export]
macro_rules! move_to_u32s {
    ($str:expr) => {{
        if $str.chars().nth(3).unwrap().is_numeric() { // regular move
            let r1: u32 = $str.chars().nth(0).unwrap().to_digit(10).unwrap();
            let c1: u32 = $str.chars().nth(1).unwrap().to_digit(10).unwrap();
            let r2: u32 = $str.chars().nth(2).unwrap().to_digit(10).unwrap();
            let c2: u32 = $str.chars().nth(3).unwrap().to_digit(10).unwrap();
            (r1, c1, r2, c2)
        } else if $str.chars().nth(3).unwrap() == 'P' { // pawn promo
            let c1: u32 = $str.chars().nth(0).unwrap().to_digit(10).unwrap();
            let c2: u32 = $str.chars().nth(1).unwrap().to_digit(10).unwrap();
            let (r1, r2) = if $str.chars().nth(2).unwrap().is_uppercase() {(1, 0)} else {(6, 7)};
            (r1, c1, r2, c2)
        } else if $str.chars().nth(3).unwrap() == 'E' { // enpassant
            let c1: u32 = $str.chars().nth(0).unwrap().to_digit(10).unwrap();
            let c2: u32 = $str.chars().nth(1).unwrap().to_digit(10).unwrap();
            let (r1, r2) = if $str.chars().nth(2).unwrap() == 'w' {(3, 2)} else {(4, 5)};
            (r1, c1, r2, c2)
        } else {
            panic!("INVALID MOVE TYPE");
        }
    }};
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


/// Macro to convert move string to algebra notation (non-E/P moves)
#[macro_export]
macro_rules! algebra_to_move {
    ($move:expr) => {{
        let mut move_str: String = String::new();
        let idx_to_file_ascii_shift: u8 = 49;
        let move_chars: Vec<char> = $move.chars().collect();
        if move_chars[3] == 'E' || move_chars[3] == 'P' {
            panic!("Cannot Convert for E and P Moves");
        }
        move_str.push((('8' as u8 - move_chars[1] as u8) + '0' as u8) as char);
        move_str.push((move_chars[0] as u8 - idx_to_file_ascii_shift) as char);
        move_str.push((('8' as u8 - move_chars[3] as u8) + '0' as u8) as char);
        move_str.push((move_chars[2] as u8 - idx_to_file_ascii_shift) as char);
        move_str
    }};
}


/// Macro to add classes to PyModule
#[macro_export]
macro_rules! add_classes {
    ($module:ident, $($class:ty),+) => {
        $(
            $module.add_class::<$class>()?;
        )+
    };
}


/// Macro to add functions to PyModule
#[macro_export]
macro_rules! add_functions {
    ($module:ident, $($function:ident),+) => {
        $(
            $module.add_wrapped(wrap_pyfunction!($function))?;
        )+
    };
}

