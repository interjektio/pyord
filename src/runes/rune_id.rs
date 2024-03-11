use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use ord::runes::RuneId;

/// RuneId
/// :param height: Etching block height
/// :type height: int
/// :param index: Etching transaction index
/// :type index: int
#[pyclass(name="RuneId")]
#[derive(Debug, PartialEq, Copy, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct PyRuneId(pub RuneId);

#[pymethods]
impl PyRuneId {
    #[new]
    pub fn new(height: u32, index: u16) -> Self {
        PyRuneId(RuneId { height, index })
    }

    pub fn __eq__(&self, other: Self) -> bool {
        self.0 == other.0
    }

    /// :rtype: str
    pub fn __repr__(&self) -> String {
        format!("RuneId(height={}, index={})", self.0.height, self.0.index)
    }

    /// Parse the RuneId from a number usable as Edict id
    /// :type num: int
    /// :rtype: RuneId
    #[staticmethod]
    pub fn from_num(num: u128) -> PyResult<Self> {
        RuneId::try_from(num)
            .map(|rune_id| Ok(PyRuneId(rune_id)))
            .unwrap_or_else(|_| Err(PyValueError::new_err("Invalid RuneId")))
    }


    /// :rtype: int
    /// :return: number suitable for use as Edict id
    #[getter]
    pub fn num(&self) -> u128 {
        self.0.into()
    }

    /// :rtype: int
    #[getter]
    pub fn height(&self) -> u32 {
        self.0.height
    }

    /// :rtype: int
    #[getter]
    pub fn index(&self) -> u16 {
        self.0.index
    }
}
