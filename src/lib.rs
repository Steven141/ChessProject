use pyo3::prelude::*;


/// EXAMPLE: Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


/// Keeps track of game state, possible moves, and old moves
#[pyclass(module = "ChessProject", get_all, set_all)]
struct GameState {
    board_size: usize,
}


#[pymethods]
impl GameState {
    #[new]
    fn new(board_size: usize) -> Self {
        GameState{board_size}
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Board Size: {}", self.board_size))
    }
}


/// Keeps track of whether the kings have the right to castle
#[pyclass(module = "ChessProject", get_all, set_all)]
struct CastleRights {
    wks: bool,
    bks: bool,
    wqs: bool,
    bqs: bool,
}


#[pymethods]
impl CastleRights {
    #[new]
    fn new(wks: bool, bks: bool, wqs: bool, bqs: bool) -> Self {
        CastleRights{wks, bks, wqs, bqs}
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("wks={} bks={} wqs={} bqs={}", self.wks, self.bks, self.wqs, self.bqs))
    }
}


/// Holds information about a specific move
#[pyclass(module = "ChessProject", get_all, set_all)]
struct Move {
    start_sq: (i32, i32),
    end_sq: (i32, i32),
    is_enpassant_move: bool,
    is_castle_move: bool,
}


#[pymethods]
impl Move {
    #[new]
    fn new(start_sq: (i32, i32), end_sq: (i32, i32), is_enpassant_move: Option<bool>, is_castle_move: Option<bool>) -> Self {
        Move {
            start_sq,
            end_sq,
            is_enpassant_move: is_enpassant_move.unwrap_or(false),
            is_castle_move: is_castle_move.unwrap_or(false),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Start={:?} End={:?} Enpassant={} Castle={}", self.start_sq, self.end_sq, self.is_enpassant_move, self.is_castle_move))
    }
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
    add_classes!(m, GameState, CastleRights, Move);
    Ok(())
}
