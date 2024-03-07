use pyo3::prelude::*;

use ord::runes::Runestone;

use super::edict::PyEdict;
use super::etching::PyEtching;
use super::mint::PyMint;
use super::rune::PyRune;

/// Runestone
/// :type burn: bool, optional
/// :type claim: typing.Optional[int], optional
/// :type default_output: typing.Optional[int], optional
/// :type edicts: typing.Iterable[Edict], optional
#[pyclass(name="Runestone")]
#[derive(Debug, PartialEq)]
pub struct PyRunestone(pub Runestone);


#[pymethods]
impl PyRunestone {
    #[new]
    #[pyo3(text_signature = "(burn=False, edicts=(), claim=None, default_output=None)")]
    fn new(
        burn: bool,
        edicts: Vec<PyEdict>,
        claim: Option<u128>,
        default_output: Option<u32>,
        // TODO: ord::runes::Etching is not publicly exposed
        //etching: Option<etching::PyEtching>,
    ) -> Self {
        PyRunestone(Runestone {
            burn,
            claim,
            default_output,
            edicts: edicts.into_iter().map(|e| e.0).collect(),
            ..Runestone::default()
        })
    }

    // #[staticmethod]
    // fn from_serialized_transaction(serialized_transaction: &str) -> Self {
    //
    //     PyRunestone(Runestone::from_transaction(tx))
    // }

    fn __repr__(&self) -> String {
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
    fn burn(&self) -> bool {
        self.0.burn
    }

    /// :rtype: typing.Optional[int]
    #[getter]
    fn claim(&self) -> Option<u128> {
        self.0.claim
    }

    /// :rtype: typing.Optional[int]
    #[getter]
    fn default_output(&self) -> Option<u32> {
        self.0.default_output
    }

    /// :rtype: typing.List[Edict]
    #[getter]
    fn edicts(&self) -> Vec<PyEdict> {
        self.0.edicts.iter().map(|e| PyEdict(*e)).collect()
    }

    /// :rtype: typing.Optional[Etching]
    #[getter]
    fn etching(&self) -> Option<PyEtching> {
        self.0.etching.map(|e| PyEtching {
            divisibility: e.divisibility,
            mint: e.mint.map(|m| PyMint {
                deadline: m.deadline,
                limit: m.limit,
                term: m.term,
            }),
            rune: e.rune.map(|r| PyRune(r)),
            spacers: e.spacers,
            symbol: e.symbol,
        })
    }
}
