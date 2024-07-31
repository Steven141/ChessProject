//! Chess Engine Library


#![allow(non_snake_case)]


pub mod engine_modules;


use pyo3::prelude::*;
use engine_modules::*;


/// A Python module implemented in Rust.
#[pymodule]
fn ChessProject(_py: Python, m: &PyModule) -> PyResult<()> {
    add_classes!(
        m,
        special_bitboards::SpecialBitBoards,
        game_state::GameState,
        moves::Moves,
        perft::Perft,
        best_move_finder::BestMoveFinder,
        zobrist::Zobrist,
        trans_table::TransTable,
        opening_book::OpeningBook
    );
    Ok(())
}
