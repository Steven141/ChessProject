use pyo3::prelude::*;

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

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
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
    add_classes!(m, GameState);
    Ok(())
}
