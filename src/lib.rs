#![allow(dead_code)]

use pyo3::prelude::*;

mod branch;
mod commit;
mod repo;
mod utils;

/// A Python module implemented in Rust using PyO3
#[pymodule]
fn rustygit(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register repository-related functionality
    m.add_class::<repo::Repository>()?;
    m.add_class::<commit::Commit>()?;
    m.add_class::<branch::Branch>()?;

    // Add other module functions and classes as we go

    Ok(())
}
