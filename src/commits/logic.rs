use crate::utils::git_err_to_py_err;
use git2::{BlameOptions, DiffOptions, Repository};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

use super::model::{BlameLine, Commit, DiffEntry};

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
    let abs_path = std::fs::canonicalize(file_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
    let repo = Repository::discover(&abs_path).map_err(git_err_to_py_err)?;

    let rel_path = get_repository_relative_path(&abs_path, &repo)
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to get relative path: {}",
                e
            ))
        })?;

    let mut options = BlameOptions::new();
    let blame = repo
        .blame_file(&rel_path, Some(&mut options))
        .map_err(git_err_to_py_err)?;

    let file = File::open(&abs_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

    let mut result = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if let Some(hunk) = blame.get_line(i + 1) {
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

#[pyfunction]
pub fn get_blame_for_files(py: Python, file_paths: Vec<String>) -> PyResult<PyObject> {
    let results: Mutex<HashMap<String, Vec<BlameLine>>> = Mutex::new(HashMap::new());
    let errors: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());

    file_paths
        .par_iter()
        .for_each(|file_path| match process_file_blame(file_path) {
            Ok(entries) => {
                results.lock().unwrap().insert(file_path.clone(), entries);
            }
            Err(error_msg) => {
                errors.lock().unwrap().push((file_path.clone(), error_msg));
            }
        });

    let error_list = errors.lock().unwrap();
    if !error_list.is_empty() {
        eprintln!("WARNING: Some files could not be processed for blame:");
        for (file, error) in error_list.iter() {
            eprintln!("  - {}: {}", file, error);
        }
    }

    let pydict = PyDict::new(py);
    for (k, v) in results.lock().unwrap().iter() {
        let items: Vec<PyObject> = v.iter().map(|line| line.clone().into_py(py)).collect();
        let py_list = PyList::new(py, &items);
        pydict.set_item(k, py_list)?;
    }

    Ok(pydict.to_object(py))
}

fn process_file_blame(file_path: &str) -> Result<Vec<BlameLine>, String> {
    let abs_path = std::fs::canonicalize(file_path)
        .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

    let repo = Repository::discover(&abs_path)
        .map_err(|e| format!("Failed to discover repository: {}", e))?;

    let rel_path = get_repository_relative_path(&abs_path, &repo)
        .map_err(|e| format!("Failed to get relative path: {}", e))?;

    let mut options = BlameOptions::new();

    let blame = repo
        .blame_file(&rel_path, Some(&mut options))
        .map_err(|e| format!("Failed to blame file: {}", e))?;

    let file = File::open(&abs_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to read lines: {}", e))?;

    let mut entries = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if let Some(hunk) = blame.get_line(i + 1) {
            let commit = repo
                .find_commit(hunk.final_commit_id())
                .map_err(|e| format!("Failed to find commit: {}", e))?;

            let summary = commit.summary().unwrap_or("").to_string();

            entries.push(BlameLine::new(
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

    Ok(entries)
}

fn get_repository_relative_path(abs_path: &std::path::Path, repo: &Repository) -> Result<std::path::PathBuf, String> {
    let workdir = repo.workdir()
        .ok_or_else(|| "Repository work directory not found (bare repository?)".to_string())?;
    
    // Try to get the path relative to the workdir using a different approach
    let repo_path_buf = workdir.to_path_buf();
    
    // Normalize paths - remove any Windows UNC prefixes
    let mut abs_path_str = abs_path.to_string_lossy().to_string();
    if abs_path_str.starts_with("//?/") || abs_path_str.starts_with("\\\\?\\") {
        abs_path_str = abs_path_str.replace("//?/", "").replace("\\\\?\\", "");
    }
    
    // Create normalized strings
    let abs_path_str_norm = abs_path_str.replace('\\', "/");
    let workdir_str_norm = repo_path_buf.to_string_lossy().to_string().replace('\\', "/");
    
    // Split into components
    let abs_components: Vec<String> = abs_path_str_norm
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
        
    let workdir_components: Vec<String> = workdir_str_norm
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    
    // Find where the paths diverge (ignoring case on Windows)
    let mut common_prefix_len = 0;
    for (a, b) in abs_components.iter().zip(workdir_components.iter()) {
        #[cfg(windows)]
        let components_match = a.to_lowercase() == b.to_lowercase();
        #[cfg(not(windows))]
        let components_match = a == b;
        
        if components_match {
            common_prefix_len += 1;
        } else {
            break;
        }
    }
    
    if common_prefix_len >= workdir_components.len() {
        // Build the relative path from the remaining components
        let rel_components = &abs_components[common_prefix_len..];
        let rel_path = rel_components.join("/");
        
        Ok(std::path::PathBuf::from(rel_path))
    } else {
        Err(format!(
            "File path '{}' is not within repository working directory '{}'",
            abs_path.display(), repo_path_buf.display()
        ))
    }
}