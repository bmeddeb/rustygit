use crate::commit::Commit;
use crate::utils::git_err_to_py_err;
use git2;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::path::Path;

#[pyclass(unsendable)]
pub struct Repo {
    inner: git2::Repository,
    path: String,
}

#[pymethods]
impl Repo {
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

        Ok(Repo {
            inner: repo,
            path: path.to_string(),
        })
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

        Ok(Repo {
            inner: repo,
            path: path.to_string(),
        })
    }

    /// Clone a repository from a URL
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the Git repository to clone
    /// * `path` - Optional path where the repository should be cloned.
    ///            If not provided, it will use the repository name in the current directory.
    ///
    /// # Returns
    ///
    /// A new `Repo` instance pointing to the cloned repository.
    ///
    /// # Errors
    ///
    /// Returns an error if the cloning operation fails.
    #[staticmethod]
    #[pyo3(text_signature = "(url, path=None, /)")]
    fn clone(url: &str, path: Option<&str>) -> PyResult<Self> {
        // Determine the clone path
        let target_path = match path {
            Some(p) => p.to_string(),
            None => {
                // Extract repository name from URL and use it in the current directory
                let url_parts: Vec<&str> = url.split('/').collect();
                let repo_name = url_parts
                    .last()
                    .map(|name| {
                        if name.ends_with(".git") {
                            name[..name.len() - 4].to_string()
                        } else {
                            name.to_string()
                        }
                    })
                    .unwrap_or_else(|| "repo".to_string());

                // Use current directory + repo name
                format!("./{}", repo_name)
            }
        };

        // Clone the repository
        let repo = match git2::Repository::clone(url, Path::new(&target_path)) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to clone repository: {}",
                    e
                )))
            }
        };

        Ok(Repo {
            inner: repo,
            path: target_path,
        })
    }

    /// Get the repository path
    #[getter]
    fn path(&self) -> PyResult<String> {
        Ok(self.path.clone())
    }

    /// Check if the repository is bare
    fn is_bare(&self) -> bool {
        self.inner.is_bare()
    }

    /// Find all commits that modify specified files, in parallel using thread-safe methods
    fn find_commits_modifying_files(&self, files: Vec<String>) -> PyResult<Vec<Commit>> {
        // Clone the path for thread safety
        let repo_path = self.inner.path().to_path_buf();

        // Create a revwalk to iterate through commits
        let mut revwalk = self.inner.revwalk().map_err(|e| git_err_to_py_err(e))?;

        // Configure revwalk to start from HEAD and traverse all commits
        revwalk.push_head().map_err(|e| git_err_to_py_err(e))?;

        // Collect all commit OIDs
        let oids: Vec<git2::Oid> = revwalk
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| git_err_to_py_err(e))?;

        // Process commits in parallel using Rayon
        let results: Vec<Result<Option<Commit>, PyErr>> = oids
            .par_iter()
            .map(|oid| {
                // Open a new repository instance for each thread
                let repo = match git2::Repository::open(&repo_path) {
                    Ok(repo) => repo,
                    Err(e) => return Err(git_err_to_py_err(e)),
                };

                let commit = repo.find_commit(*oid).map_err(|e| git_err_to_py_err(e))?;

                // If this is not the first commit, get parent to diff against
                if commit.parent_count() > 0 {
                    let parent = commit.parent(0).map_err(|e| git_err_to_py_err(e))?;

                    let parent_tree = parent.tree().map_err(|e| git_err_to_py_err(e))?;

                    let commit_tree = commit.tree().map_err(|e| git_err_to_py_err(e))?;

                    let diff = repo
                        .diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), None)
                        .map_err(|e| git_err_to_py_err(e))?;

                    // Check if diff modifies any of the specified files
                    let mut modified_target_file = false;

                    // Updated foreach usage according to git2 0.20.x
                    diff.foreach(
                        &mut |delta: git2::DiffDelta, _| {
                            if let Some(path) = delta.new_file().path() {
                                if let Some(path_str) = path.to_str() {
                                    if files.iter().any(|f| path_str.contains(f)) {
                                        modified_target_file = true;
                                        return false; // Stop iteration
                                    }
                                }
                            }
                            true
                        },
                        None,
                        None,
                        None,
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
