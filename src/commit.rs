use git2;
use pyo3::prelude::*;

#[pyclass]
pub struct Commit {
    id: String,
    message: Option<String>,
    author_name: Option<String>,
    author_email: Option<String>,
    time: i64,
}

#[pymethods]
impl Commit {
    #[new]
    #[pyo3(signature = (id, message=None, author_name=None, author_email=None, time=0))]
    fn new(
        id: String,
        message: Option<String>,
        author_name: Option<String>,
        author_email: Option<String>,
        time: i64,
    ) -> Self {
        Self {
            id,
            message,
            author_name,
            author_email,
            time,
        }
    }

    /// Get the commit ID (SHA)
    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(self.id.clone())
    }

    /// Get the commit message
    #[getter]
    fn message(&self) -> PyResult<Option<String>> {
        Ok(self.message.clone())
    }

    /// Get the author name
    #[getter]
    fn author_name(&self) -> PyResult<Option<String>> {
        Ok(self.author_name.clone())
    }

    /// Get the author email
    #[getter]
    fn author_email(&self) -> PyResult<Option<String>> {
        Ok(self.author_email.clone())
    }

    /// Get the commit time (as Unix timestamp)
    #[getter]
    fn time(&self) -> PyResult<i64> {
        Ok(self.time)
    }
}

impl Commit {
    /// Create a Commit from a git2::Commit object (for internal Rust use only)
    pub fn from_git_commit(commit: &git2::Commit) -> Self {
        let id = commit.id().to_string();
        let message = commit.message().map(|s| s.to_string());
        let author_name = commit.author().name().map(|s| s.to_string());
        let author_email = commit.author().email().map(|s| s.to_string());
        let time = commit.time().seconds();

        Self {
            id,
            message,
            author_name,
            author_email,
            time,
        }
    }
}
