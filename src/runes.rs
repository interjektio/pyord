use pyo3::prelude::*;
use ord::runes::Rune;


/// Rune
/// :param num: The rune number
/// :type num: int
#[pyclass(name="Rune")]
struct PyRune(Rune);


#[pymethods]
impl PyRune {
    #[new]
    fn new(num: u128) -> Self {
        PyRune(Rune(num))
    }

    /// get the number (id) of the rune
    /// :rtype: int
    #[getter]
    fn num(&self) -> u128 {
        self.0 .0
    }

    /// the name of the rune as string
    /// :rtype: str
    #[getter]
    fn name(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Rune(num={}, name='{}')", self.num(), self.name())
    }

    /// convert the string representation of the rune to a rune
    /// :param s: the string representation of the rune
    /// :type s: str
    /// :rtype: Rune
    #[staticmethod]
    fn from_str(s: &str) -> Self {
        Self(s.parse::<Rune>().unwrap())
    }
}


pub fn register(m: &PyModule) -> PyResult<()> {
    m.add_class::<PyRune>()?;
    Ok(())
}
