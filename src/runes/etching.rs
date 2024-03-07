use pyo3::prelude::*;

use super::mint;
use super::rune;

/// :type divisibility: int
/// :type mint: typing.Optional[Mint], optional
/// :type rune: typing.Optional[Rune], optional
/// :type spacers: int
/// :type symbol: typing.Optional[str], optional
#[pyclass(name="Etching")]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PyEtching {
    /// :rtype: int
    #[pyo3(get)]
    pub divisibility: u8,

    /// :rtype: typing.Optional[Mint]
    #[pyo3(get)]
    pub mint: Option<mint::PyMint>,

    /// :rtype: typing.Optional[Rune]
    #[pyo3(get)]
    pub rune: Option<rune::PyRune>,

    /// :rtype: int
    #[pyo3(get)]
    pub spacers: u32,

    /// :rtype: typing.Optional[str]
    #[pyo3(get)]
    pub symbol: Option<char>,
}


#[pymethods]
impl PyEtching {
    #[new]
    fn new(
        divisibility: u8,
        spacers: u32,
        mint: Option<mint::PyMint>,
        rune: Option<rune::PyRune>,
        symbol: Option<char>,
    ) -> Self {
        Self {
            divisibility,
            mint,
            rune,
            spacers,
            symbol,
        }
    }
}
