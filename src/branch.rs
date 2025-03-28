use pyo3::prelude::*;

#[pyclass]
pub struct Branch {
    name: String,
    is_remote: bool,
}

#[pymethods]
impl Branch {
    #[new]
    fn new(name: String, is_remote: bool) -> Self {
        Branch { name, is_remote }
    }

    /// Get the branch name
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    /// Check if branch is remote
    #[getter]
    fn is_remote(&self) -> PyResult<bool> {
        Ok(self.is_remote)
    }
}
