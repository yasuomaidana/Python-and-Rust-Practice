mod test_function_calling;

use pyo3::prelude::*;
use pyo3::prepare_freethreaded_python;

fn main() -> PyResult<()> {
    prepare_freethreaded_python();
    Python::with_gil(|py| {
        let builtins = PyModule::import(py, "builtins")?;
        let total: i32 = builtins
            .getattr("sum")?
            .call1((vec![1, 2, 3],))?
            .extract()?;
        let sys = PyModule::import(py, "sys")?;
        let python_path:Vec<String> = sys.getattr("path")?.extract()?;
        println!("Python path: {:?}", python_path);
        // let _matplotlib = PyModule::import(py, "matplotlib")?;
        println!("Total: {}", total);
        Ok(())
    })
}
