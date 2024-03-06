use pyo3::prelude::*;
use ord::runes::Rune;


/// Parse a rune number to text
///
/// :param n: The rune number
/// :type n: int
///
/// :rtype: str
#[pyfunction]
pub fn rune(n: u128) -> PyResult<String> {
    Ok(Rune(n).to_string())
}
