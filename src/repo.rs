use git2;
use pyo3::prelude::*;
use std::path::Path;

#[pyclass(unsendable)]
pub struct Repository {
    inner: git2::Repository,
}

#[pymethods]
impl Repository {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let repo = match git2::Repository::open(Path::new(path)) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to open repository: {}",
                    e
                )))
            }
        };

        Ok(Repository { inner: repo })
    }

    /// Initialize a new git repository at the given path
    #[staticmethod]
    fn init(path: &str) -> PyResult<Self> {
        let repo = match git2::Repository::init(Path::new(path)) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to initialize repository: {}",
                    e
                )))
            }
        };

        Ok(Repository { inner: repo })
    }

    /// Clone a repository from a URL
    #[staticmethod]
    fn clone(url: &str, path: &str) -> PyResult<Self> {
        let repo = match git2::Repository::clone(url, Path::new(path)) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to clone repository: {}",
                    e
                )))
            }
        };

        Ok(Repository { inner: repo })
    }

    /// Check if the repository is bare
    fn is_bare(&self) -> bool {
        self.inner.is_bare()
    }
}
