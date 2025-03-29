use crate::utils::git_err_to_py_err;
use git2::{Commit as GitCommit, Oid, Repository};
use pyo3::prelude::*;

#[pyclass]
pub struct Commit {
    #[pyo3(get)]
    pub hash: String,

    #[pyo3(get)]
    pub author: String,
    #[pyo3(get)]
    pub author_email: String,
    #[pyo3(get)]
    pub author_time: i64,

    #[pyo3(get)]
    pub committer: String,
    #[pyo3(get)]
    pub committer_email: String,
    #[pyo3(get)]
    pub commit_time: i64,

    #[pyo3(get)]
    pub message: String,

    #[pyo3(get)]
    pub parents: Vec<String>,
}

impl Commit {
    pub fn from_git_commit(commit: &GitCommit) -> Self {
        Commit {
            hash: commit.id().to_string(),

            author: commit.author().name().unwrap_or("").to_string(),
            author_email: commit.author().email().unwrap_or("").to_string(),
            author_time: commit.author().when().seconds(),

            committer: commit.committer().name().unwrap_or("").to_string(),
            committer_email: commit.committer().email().unwrap_or("").to_string(),
            commit_time: commit.time().seconds(),

            message: commit.message().unwrap_or("").to_string(),
            parents: commit.parent_ids().map(|id| id.to_string()).collect(),
        }
    }
}

#[pyfunction]
pub fn get_commit_history(path: &str) -> PyResult<Vec<Commit>> {
    let repo = Repository::open(path).map_err(git_err_to_py_err)?;
    let mut revwalk = repo.revwalk().map_err(git_err_to_py_err)?;
    revwalk.push_head().map_err(git_err_to_py_err)?;

    let mut commits = Vec::new();

    for oid_result in revwalk {
        let oid = oid_result.map_err(git_err_to_py_err)?;
        let commit = repo.find_commit(oid).map_err(git_err_to_py_err)?;
        commits.push(Commit::from_git_commit(&commit));
    }

    Ok(commits)
}

#[pymodule]
pub fn commits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Commit>()?;
    m.add_function(wrap_pyfunction!(get_commit_history, m)?)?;
    Ok(())
}
