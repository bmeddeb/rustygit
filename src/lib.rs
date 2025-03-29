#![allow(dead_code)]

use pyo3::prelude::*;

mod branch;
mod commit;
mod repo;
mod utils;

#[pymodule]
fn rustygit(py: Python, m: &PyModule) -> PyResult<()> {
    // Register Repo and Branch classes at the top level
    m.add_class::<repo::Repo>()?;
    m.add_class::<branch::Branch>()?;
    m.add_class::<commit::Commit>()?;

    // Add top-level functions
    m.add_function(wrap_pyfunction!(commit::get_commit_history, m)?)?;

    // `commit` submodule (for backward compatibility)
    let commit = PyModule::new(py, "commits")?;
    commit::commits(py, commit)?; // calls #[pymodule] fn commits()
    m.add_submodule(commit)?;

    Ok(())
}
