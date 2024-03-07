use pyo3::prelude::*;

/// :type deadline: typing.Optional[int], optional
/// :type limit: typing.Optional[int], optional
/// :type term: typing.Optional[int], optional
#[pyclass(name="Mint")]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PyMint {
    /// :rtype: typing.Optional[int]
    #[pyo3(get)]
    pub deadline: Option<u32>,

    /// :rtype: typing.Optional[int]
    #[pyo3(get)]
    pub limit: Option<u128>,

    /// :rtype: typing.Optional[int]
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