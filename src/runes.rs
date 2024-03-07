use pyo3::prelude::*;

pub mod rune;
pub mod runestone;
pub mod edict;
pub mod etching;
pub mod mint;


pub fn register(m: &PyModule) -> PyResult<()> {
    m.add_class::<rune::PyRune>()?;
    m.add_class::<runestone::PyRunestone>()?;
    m.add_class::<edict::PyEdict>()?;

    // NOTE: mint and etching are not exposed by Ord
    m.add_class::<mint::PyMint>()?;
    m.add_class::<etching::PyEtching>()?;

    Ok(())
}
