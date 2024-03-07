use pyo3::prelude::*;

use super::mint;
use super::rune;

#[pyclass]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PyEtching {
    #[pyo3(get)]
    pub divisibility: u8,
    #[pyo3(get)]
    pub mint: Option<mint::PyMint>,
    #[pyo3(get)]
    pub rune: Option<rune::PyRune>,
    #[pyo3(get)]
    pub spacers: u32,
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
