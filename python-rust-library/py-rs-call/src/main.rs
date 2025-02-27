use pyo3::ffi::Py_Initialize;
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    unsafe { Py_Initialize(); }
    Python::with_gil(|py| {
        let builtins = PyModule::import(py, "builtins")?;
        let total: i32 = builtins
            .getattr("sum")?
            .call1((vec![1, 2, 3],))?
            .extract()?;
        let _matplotlib = PyModule::import(py, "matplotlib")?;
        println!("Total: {}", total);
        Ok(())
    })
}
