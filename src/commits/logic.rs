use super::model::{Commit, DiffEntry};
use crate::utils::git_err_to_py_err;
use git2::{DiffOptions, Repository};
use pyo3::prelude::*;

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
