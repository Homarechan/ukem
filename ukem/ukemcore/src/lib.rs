use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn hello(name: &str) -> String {
    format!("Hello, {}", name.to_string())
}

#[pymodule]
fn ukemcore(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(hello))?;

    Ok(())
}
