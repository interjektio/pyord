mod runes;

use pyo3::prelude::*;

/// Python wrapper for Ordinals
#[pymodule]
fn pyord(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__package__", "pyord")?;

    m.add_function(wrap_pyfunction!(runes::rune, m)?)?;

    Ok(())
}

