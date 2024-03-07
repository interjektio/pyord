use pyo3::prelude::*;

use ord::runes::Runestone;

use super::edict::PyEdict;
use super::etching::PyEtching;
use crate::utils::hex_to_bitcoin_tx;

/// Runestone
/// :type burn: bool, optional
/// :type claim: typing.Optional[int], optional
/// :type default_output: typing.Optional[int], optional
/// :type edicts: typing.Iterable[Edict], optional
/// :type etching: typing.Optional[Etching], optional
#[pyclass(name="Runestone")]
#[derive(Debug, PartialEq)]
pub struct PyRunestone(pub Runestone);


#[pymethods]
impl PyRunestone {
    #[new]
    #[pyo3(text_signature = "(burn=False, edicts=(), claim=None, default_output=None, etching=None)")]
    pub fn new(
        burn: bool,
        edicts: Vec<PyEdict>,
        claim: Option<u128>,
        default_output: Option<u32>,
        etching: Option<PyEtching>,
    ) -> Self {
        PyRunestone(Runestone {
            burn,
            claim,
            default_output,
            edicts: edicts.into_iter().map(|e| e.0).collect(),
            etching: etching.map(|e| e.0),
        })
    }

    /// Return a Runestone from a Bitcoin transaction, or None if the transaction contains no
    /// Runestone
    /// :type hex_tx: str
    /// :rtype: typing.Optional[Runestone]
    #[staticmethod]
    pub fn from_hex_tx(hex_tx: &str) -> Option<Self> {
        let tx = hex_to_bitcoin_tx(hex_tx);
        Runestone::from_transaction(&tx).map(|r| PyRunestone(r))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Runestone(burn={}, claim={}, default_output={}, edicts={})",
            self.burn(),
            self.claim().map(|d| d.to_string()).unwrap_or("None".to_string()),
            self.default_output().map(|d| d.to_string()).unwrap_or("None".to_string()),
            self.edicts().len(),
        )
    }

    /// :rtype: bool
    #[getter]
    pub fn burn(&self) -> bool {
        self.0.burn
    }

    /// :rtype: typing.Optional[int]
    #[getter]
    pub fn claim(&self) -> Option<u128> {
        self.0.claim
    }

    /// :rtype: typing.Optional[int]
    #[getter]
    pub fn default_output(&self) -> Option<u32> {
        self.0.default_output
    }

    /// :rtype: typing.List[Edict]
    #[getter]
    pub fn edicts(&self) -> Vec<PyEdict> {
        self.0.edicts.iter().map(|e| PyEdict(*e)).collect()
    }

    /// :rtype: typing.Optional[Etching]
    #[getter]
    pub fn etching(&self) -> Option<PyEtching> {
        self.0.etching.map(|e| PyEtching(e))
    }
}
