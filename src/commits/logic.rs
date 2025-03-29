use super::model::{BlameLine, Commit, DiffEntry};
use crate::utils::git_err_to_py_err;
use git2::{BlameOptions, DiffOptions, Repository};
use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

#[pyfunction]
pub fn get_file_change_summary(
    path: &str,
    commit1: &str,
    commit2: &str,
) -> PyResult<Vec<DiffEntry>> {
    let repo = Repository::open(path).map_err(git_err_to_py_err)?;

    let oid1 = repo
        .revparse_single(commit1)
        .map_err(git_err_to_py_err)?
        .id();
    let oid2 = repo
        .revparse_single(commit2)
        .map_err(git_err_to_py_err)?
        .id();

    let commit1 = repo.find_commit(oid1).map_err(git_err_to_py_err)?;
    let commit2 = repo.find_commit(oid2).map_err(git_err_to_py_err)?;

    let tree1 = commit1.tree().map_err(git_err_to_py_err)?;
    let tree2 = commit2.tree().map_err(git_err_to_py_err)?;

    let mut options = DiffOptions::new();
    let diff = repo
        .diff_tree_to_tree(Some(&tree1), Some(&tree2), Some(&mut options))
        .map_err(git_err_to_py_err)?;

    let mut results = Vec::new();

    diff.foreach(
        &mut |delta, _| {
            let file_path = delta
                .new_file()
                .path()
                .or_else(|| delta.old_file().path())
                .and_then(|p| p.to_str())
                .unwrap_or("")
                .to_string();

            let mut additions = 0;
            let mut deletions = 0;

            let _ = diff.print(git2::DiffFormat::Patch, |_, _, line| {
                match line.origin() {
                    '+' => additions += 1,
                    '-' => deletions += 1,
                    _ => {}
                }
                true
            });

            results.push(DiffEntry::new(file_path, additions, deletions));
            true
        },
        None,
        None,
        None,
    )
    .map_err(git_err_to_py_err)?;

    Ok(results)
}
#[pyfunction]
pub fn get_file_blame(file_path: &str) -> PyResult<Vec<BlameLine>> {
    // Canonicalize the path (absolute path to file)
    let abs_path = std::fs::canonicalize(file_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

    // Discover the repository from the file's path
    let repo = Repository::discover(&abs_path).map_err(git_err_to_py_err)?;

    // Get the relative path from repo root
    let rel_path = abs_path
        .strip_prefix(repo.path().parent().unwrap())
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to get relative path: {}",
                e
            ))
        })?;

    // Prepare blame options
    let mut options = BlameOptions::new();
    let blame = repo
        .blame_file(rel_path, Some(&mut options))
        .map_err(git_err_to_py_err)?;

    // Read file lines to preserve content
    let file = File::open(&abs_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

    let mut result = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if let Some(hunk) = blame.get_line(i + 1) {
            // Look up the commit to get the summary
            let commit = repo
                .find_commit(hunk.final_commit_id())
                .map_err(git_err_to_py_err)?;
            let summary = commit.summary().unwrap_or("").to_string();

            result.push(BlameLine::new(
                i + 1,
                line.clone(),
                hunk.final_commit_id().to_string(),
                hunk.final_signature().name().unwrap_or("").to_string(),
                hunk.final_signature().email().unwrap_or("").to_string(),
                hunk.final_signature().when().seconds(),
                hunk.orig_signature().name().unwrap_or("").to_string(),
                hunk.orig_signature().when().seconds(),
                summary,
            ));
        }
    }

    Ok(result)
}
