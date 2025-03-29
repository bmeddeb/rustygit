use crate::commit::Commit;
use crate::utils::git_err_to_py_err;
use git2;
use pyo3::prelude::*;
use rayon::prelude::*;
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

    /// Find all commits that modify specified files, in parallel
    fn find_commits_modifying_files(&self, files: Vec<String>) -> PyResult<Vec<Commit>> {
        // Create a revwalk to iterate through commits
        let mut revwalk = self.inner.revwalk().map_err(|e| git_err_to_py_err(e))?;

        // Configure revwalk to start from HEAD and traverse all commits
        revwalk.push_head().map_err(|e| git_err_to_py_err(e))?;

        // Collect all commit OIDs
        let oids: Vec<_> = revwalk
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| git_err_to_py_err(e))?;

        // Process commits in parallel using Rayon
        let results: Vec<Result<Option<Commit>, PyErr>> = oids
            .par_iter()
            .map(|oid| {
                let commit = self
                    .inner
                    .find_commit(*oid)
                    .map_err(|e| git_err_to_py_err(e))?;

                // If this is not the first commit, get parent to diff against
                if commit.parent_count() > 0 {
                    let parent = commit.parent(0).map_err(|e| git_err_to_py_err(e))?;

                    let parent_tree = parent.tree().map_err(|e| git_err_to_py_err(e))?;

                    let commit_tree = commit.tree().map_err(|e| git_err_to_py_err(e))?;

                    let diff = self
                        .inner
                        .diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), None)
                        .map_err(|e| git_err_to_py_err(e))?;

                    // Check if diff modifies any of the specified files
                    let mut modified_target_file = false;
                    diff.foreach(
                        &mut |_, _| true,
                        &mut |_, _| true,
                        &mut |_, _| true,
                        &mut |file_delta, _| {
                            if let Some(path) = file_delta.new_file().path() {
                                if let Some(path_str) = path.to_str() {
                                    if files.iter().any(|f| path_str.contains(f)) {
                                        modified_target_file = true;
                                        return false; // Stop iteration
                                    }
                                }
                            }
                            true
                        },
                    )
                    .map_err(|e| git_err_to_py_err(e))?;

                    if modified_target_file {
                        // Create a Commit object using our from_git_commit method
                        return Ok(Some(Commit::from_git_commit(&commit)));
                    }
                }

                Ok(None)
            })
            .collect();

        // Filter out None values and handle errors
        let mut commits = Vec::new();
        for result in results {
            match result? {
                Some(commit) => commits.push(commit),
                None => continue,
            }
        }

        Ok(commits)
    }
}
