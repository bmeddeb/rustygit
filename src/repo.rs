use crate::utils::git_err_to_py_err;
use futures::future;
use git2;
use pyo3::prelude::*;
use pyo3_asyncio::tokio as pyo3_tokio;
use std::path::Path;
use tokio::task;

#[pyclass(unsendable)]
pub struct Repo {
    inner: git2::Repository,
    path: String,
}

#[pymethods]
impl Repo {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let repo = git2::Repository::open(Path::new(path)).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to open repository: {}",
                e
            ))
        })?;

        Ok(Repo {
            inner: repo,
            path: path.to_string(),
        })
    }

    #[staticmethod]
    fn init(path: &str) -> PyResult<Self> {
        let repo = git2::Repository::init(Path::new(path)).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to initialize repository: {}",
                e
            ))
        })?;

        Ok(Repo {
            inner: repo,
            path: path.to_string(),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (url, path=None, username=None, token=None))]
    fn clone(
        url: &str,
        path: Option<&str>,
        username: Option<&str>,
        token: Option<&str>,
    ) -> PyResult<Self> {
        let target_path = path.map_or_else(
            || url.split('/').last().unwrap().replace(".git", ""),
            String::from,
        );

        let mut callbacks = git2::RemoteCallbacks::new();
        if let (Some(user), Some(tok)) = (username, token) {
            callbacks.credentials(move |_, _, _| git2::Cred::userpass_plaintext(user, tok));
        }

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let repo = git2::build::RepoBuilder::new()
            .fetch_options(fetch_options)
            .clone(url, Path::new(&target_path))
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Clone failed: {}", e))
            })?;

        Ok(Repo {
            inner: repo,
            path: target_path,
        })
    }

    #[getter]
    fn path(&self) -> PyResult<String> {
        Ok(self.path.clone())
    }

    fn is_bare(&self) -> bool {
        self.inner.is_bare()
    }

    fn fetch_updates(
        &self,
        remote_name: Option<&str>,
        branch: Option<&str>,
        username: Option<&str>,
        token: Option<&str>,
    ) -> PyResult<()> {
        let mut callbacks = git2::RemoteCallbacks::new();
        if let (Some(user), Some(tok)) = (username, token) {
            callbacks.credentials(move |_, _, _| git2::Cred::userpass_plaintext(user, tok));
        }

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut remote = self
            .inner
            .find_remote(remote_name.unwrap_or("origin"))
            .map_err(git_err_to_py_err)?;

        remote
            .fetch(&[branch.unwrap_or("main")], Some(&mut fetch_options), None)
            .map_err(git_err_to_py_err)?;

        Ok(())
    }

    fn list_remotes(&self) -> PyResult<Vec<String>> {
        let remotes = self
            .inner
            .remotes()
            .map_err(git_err_to_py_err)?
            .iter()
            .filter_map(|name| name.map(String::from))
            .collect();

        Ok(remotes)
    }

    fn status(&self) -> PyResult<Vec<String>> {
        let statuses = self
            .inner
            .statuses(None)
            .map_err(git_err_to_py_err)?
            .iter()
            .filter_map(|entry| entry.path().map(String::from))
            .collect();

        Ok(statuses)
    }
    #[staticmethod]
    fn clone_multiple_async(
        py: Python,
        urls: Vec<String>,
        base_dir: Option<String>,
        username: Option<String>,
        token: Option<String>,
    ) -> PyResult<&PyAny> {
        pyo3_tokio::future_into_py(py, async move {
            let base_dir = base_dir.unwrap_or_else(|| ".".to_string());

            let tasks = urls.into_iter().map(|url| {
                let username = username.clone();
                let token = token.clone();
                let target_path = format!(
                    "{}/{}",
                    base_dir,
                    url.split('/').last().unwrap().replace(".git", "")
                );

                task::spawn_blocking(move || {
                    let mut callbacks = git2::RemoteCallbacks::new();
                    if let (Some(user), Some(tok)) = (username, token) {
                        callbacks.credentials(move |_, _, _| {
                            git2::Cred::userpass_plaintext(&user, &tok)
                        });
                    }

                    let mut fetch_options = git2::FetchOptions::new();
                    fetch_options.remote_callbacks(callbacks);

                    git2::build::RepoBuilder::new()
                        .fetch_options(fetch_options)
                        .clone(&url, Path::new(&target_path))
                        .map(|_| target_path)
                        .map_err(|e| format!("Failed to clone {}: {}", url, e))
                })
            });

            let results = future::join_all(tasks).await;

            let cloned_paths: Vec<String> = results
                .into_iter()
                .filter_map(|res| res.ok().and_then(|inner| inner.ok()))
                .collect();

            Ok(cloned_paths)
        })
    }
}
