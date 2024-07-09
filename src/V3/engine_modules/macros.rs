//! Holds all project macros


/// Macro to convert the 4 characters of a move string as u32
#[macro_export]
macro_rules! move_to_u32s {
    ($str:expr) => {{
        if $str.chars().nth(3).unwrap().is_numeric() { // regular move
            let r1: u32 = $str.chars().nth(0).unwrap().to_digit(10).unwrap();
            let c1: u32 = $str.chars().nth(1).unwrap().to_digit(10).unwrap();
            let r2: u32 = $str.chars().nth(2).unwrap().to_digit(10).unwrap();
            let c2: u32 = $str.chars().nth(3).unwrap().to_digit(10).unwrap();
            (r1, c1, r2, c2)
        } else { // pawn promo or enpassant
            let c1: u32 = $str.chars().nth(0).unwrap().to_digit(10).unwrap();
            let c2: u32 = $str.chars().nth(1).unwrap().to_digit(10).unwrap();
            (c1, c2, 0, 0)
        }
    }};
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

