use pyo3::prelude::*;
use ord::runes::Etching;

use super::mint::PyMint;
use super::rune::PyRune;

/// :type divisibility: int
/// :type mint: typing.Optional[Mint], optional
/// :type rune: typing.Optional[Rune], optional
/// :type spacers: int
/// :type symbol: typing.Optional[str], optional
#[pyclass(name="Etching")]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PyEtching(pub Etching);

#[pymethods]
impl PyEtching {
    #[new]
    pub fn new(
        divisibility: u8,
        spacers: u32,
        mint: Option<PyMint>,
        rune: Option<PyRune>,
        symbol: Option<char>,
    ) -> Self {
        Self(Etching {
            divisibility,
            mint: mint.map(|m| m.0),
            rune: rune.map(|r| r.0),
            spacers,
            symbol,
        })
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Etching(divisibility={}, mint={}, rune={}, spacers={}, symbol={})",
            self.divisibility(),
            self.mint().map(|m| m.__repr__()).unwrap_or("None".to_string()),
            self.rune().map(|r| r.__repr__()).unwrap_or("None".to_string()),
            self.spacers(),
            self.symbol().map(|s| format!("'{}'", s.to_string())).unwrap_or("None".to_string()),
        )
    }

    /// :rtype: int
    #[getter]
    pub fn divisibility(&self) -> u8 {
        self.0.divisibility
    }

    /// :rtype: typing.Optional[Mint]
    #[getter]
    pub fn mint(&self) -> Option<PyMint> {
        self.0.mint.map(|m| PyMint(m))
    }

    /// :rtype: typing.Optional[Rune]
    #[getter]
    pub fn rune(&self) -> Option<PyRune> {
        self.0.rune.map(|r| PyRune(r))
    }

    /// :rtype: int
    #[getter]
    pub fn spacers(&self) -> u32 {
        self.0.spacers
    }

    /// :rtype: typing.Optional[str]
    #[getter]
    pub fn symbol(&self) -> Option<char> {
        self.0.symbol
    }
}
