use git2::Commit as GitCommit;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
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

#[pyclass]
#[derive(Clone, Debug)]
pub struct DiffEntry {
    #[pyo3(get)]
    pub path: String,
    #[pyo3(get)]
    pub additions: usize,
    #[pyo3(get)]
    pub deletions: usize,
}

impl DiffEntry {
    pub fn new(path: String, additions: usize, deletions: usize) -> Self {
        DiffEntry {
            path,
            additions,
            deletions,
        }
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct BlameLine {
    #[pyo3(get)]
    pub line_number: usize,
    #[pyo3(get)]
    pub content: String,
    #[pyo3(get)]
    pub commit_hash: String,
    #[pyo3(get)]
    pub author: String,
    #[pyo3(get)]
    pub author_email: String,
    #[pyo3(get)]
    pub author_time: i64,
    #[pyo3(get)]
    pub committer: String,
    #[pyo3(get)]
    pub commit_time: i64,
    #[pyo3(get)]
    pub summary: String,
}

impl BlameLine {
    pub fn new(
        line_number: usize,
        content: String,
        commit_hash: String,
        author: String,
        author_email: String,
        author_time: i64,
        committer: String,
        commit_time: i64,
        summary: String,
    ) -> Self {
        BlameLine {
            line_number,
            content,
            commit_hash,
            author,
            author_email,
            author_time,
            committer,
            commit_time,
            summary,
        }
    }
}
