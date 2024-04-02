use pyo3::prelude::*;

pub mod rune;
pub mod runestone;
pub mod edict;
pub mod etching;
pub mod terms;
pub mod rune_id;
pub mod cenotaph;
pub mod flaw;


pub fn register(m: &PyModule) -> PyResult<()> {
    m.add_class::<rune::PyRune>()?;
    m.add_class::<runestone::PyRunestone>()?;
    m.add_class::<edict::PyEdict>()?;
    m.add_class::<rune_id::PyRuneId>()?;
    m.add_class::<terms::PyTerms>()?;
    m.add_class::<etching::PyEtching>()?;

    m.add_class::<cenotaph::PyCenotaph>()?;
    m.add_class::<flaw::PyFlaw>()?;

    Ok(())
}
