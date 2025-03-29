#![allow(dead_code)]

use pyo3::prelude::*;

mod branch;
mod commits;
mod repo;
mod utils;

#[pymodule]
fn rustygit(py: Python, m: &PyModule) -> PyResult<()> {
    // Register top-level classes
    m.add_class::<repo::Repo>()?;
    m.add_class::<branch::Branch>()?;
    m.add_class::<commits::Commit>()?;
    m.add_class::<commits::DiffEntry>()?;
    m.add_class::<commits::BlameLine>()?;

    // Add top-level functions
    m.add_function(wrap_pyfunction!(commits::get_commit_history, m)?)?;
    m.add_function(wrap_pyfunction!(commits::get_file_change_summary, m)?)?;
    m.add_function(wrap_pyfunction!(commits::get_file_blame, m)?)?;

    // `commits` submodule (optional alternative access path)
    let commit_mod = PyModule::new(py, "commits")?;
    commit_mod.add_class::<commits::Commit>()?;
    commit_mod.add_class::<commits::DiffEntry>()?;
    commit_mod.add_class::<commits::BlameLine>()?;
    commit_mod.add_function(wrap_pyfunction!(commits::get_commit_history, commit_mod)?)?;
    commit_mod.add_function(wrap_pyfunction!(
        commits::get_file_change_summary,
        commit_mod
    )?)?;
    commit_mod.add_function(wrap_pyfunction!(commits::get_file_blame, commit_mod)?)?;
    m.add_submodule(commit_mod)?;

    Ok(())
}
