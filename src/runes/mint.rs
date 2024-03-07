use pyo3::prelude::*;
use ord::runes::Mint;

/// :type deadline: typing.Optional[int], optional
/// :type limit: typing.Optional[int], optional
/// :type term: typing.Optional[int], optional
#[pyclass(name="Mint")]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PyMint(pub Mint);

#[pymethods]
impl PyMint {
    #[new]
    pub fn new(
        deadline: Option<u32>,
        limit: Option<u128>,
        term: Option<u32>,
    ) -> Self {
        Self(Mint {
            deadline,
            limit,
            term,
        })
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Mint(deadline={}, limit={}, term={})",
            self.deadline().map(|d| d.to_string()).unwrap_or("None".to_string()),
            self.limit().map(|d| d.to_string()).unwrap_or("None".to_string()),
            self.term().map(|d| d.to_string()).unwrap_or("None".to_string()),
        )
    }

    /// :rtype: typing.Optional[int]
    #[getter]
    pub fn deadline(&self) -> Option<u32> {
        self.0.deadline
    }

    /// :rtype: typing.Optional[int]
    #[getter]
    pub fn limit(&self) -> Option<u128> {
        self.0.limit
    }

    /// :rtype: typing.Optional[int]
    #[getter]
    pub fn term(&self) -> Option<u32> {
        self.0.term
    }
}