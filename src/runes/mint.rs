use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PyMint {
    #[pyo3(get)]
    pub deadline: Option<u32>,
    #[pyo3(get)]
    pub limit: Option<u128>,
    #[pyo3(get)]
    pub term: Option<u32>,
}


#[pymethods]
impl PyMint {
    #[new]
    fn new(
        deadline: Option<u32>,
        limit: Option<u128>,
        term: Option<u32>,
    ) -> Self {
        Self {
            deadline,
            limit,
            term,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Mint(deadline={}, limit={}, term={})",
            self.deadline.map(|d| d.to_string()).unwrap_or("None".to_string()),
            self.limit.map(|d| d.to_string()).unwrap_or("None".to_string()),
            self.term.map(|d| d.to_string()).unwrap_or("None".to_string()),
        )
    }
}