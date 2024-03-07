use pyo3::prelude::*;

use ord::runes::Edict;

/// :type id: int
/// :type amount: int
/// :type output: int
#[pyclass(name="Edict")]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PyEdict(pub Edict);


#[pymethods]
impl PyEdict {
    #[new]
    pub fn new(
        id: u128,
        amount: u128,
        output: u128,
    ) -> Self {
        PyEdict(Edict {
            id,
            amount,
            output,
        })
    }

    pub fn __repr__(&self) -> String {
        format!("Edict(id={}, amount={}, output={})", self.id(), self.amount(), self.output())
    }

    /// :rtype: int
    #[getter]
    pub fn id(&self) -> u128 {
        self.0.id
    }

    /// :rtype: int
    #[getter]
    pub fn amount(&self) -> u128 {
        self.0.amount
    }

    /// :rtype: int
    #[getter]
    pub fn output(&self) -> u128 {
        self.0.output
    }
}
