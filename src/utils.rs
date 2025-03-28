use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use std::path::Path;

/// Converts a git2 error to a Python error
pub fn git_err_to_py_err(err: git2::Error) -> PyErr {
    match err.class() {
        git2::ErrorClass::Odb | git2::ErrorClass::Reference | git2::ErrorClass::Repository => {
            PyIOError::new_err(format!("Git error: {}", err))
        }
        _ => PyValueError::new_err(format!("Git error: {}", err)),
    }
}

/// Check if a path exists and is a valid git repository
pub fn is_git_repo(path: &str) -> bool {
    if !Path::new(path).exists() {
        return false;
    }

    match git2::Repository::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Extract username and email from a git signature
pub fn signature_to_user_info(sig: git2::Signature) -> (Option<String>, Option<String>) {
    let name = sig.name().map(|s| s.to_string());
    let email = sig.email().map(|s| s.to_string());

    (name, email)
}
