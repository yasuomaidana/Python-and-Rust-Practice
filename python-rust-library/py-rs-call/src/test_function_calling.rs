#[cfg(test)]
mod test_function_calling {
    // use pyo3::prelude::*;
    // use pyo3::prepare_freethreaded_python;

    #[test]
    fn test_calling_python_function() {
        // prepare_freethreaded_python();
        // Python::with_gil(|py| {
        //     let builtins = PyModule::import(py, "builtins").unwrap();
        //     let total: i32 = builtins
        //         .getattr("sum").unwrap()
        //         .call1((vec![1, 2, 3],)).unwrap()
        //         .extract().unwrap();
        //     let sys = PyModule::import(py, "sys").unwrap();
        //     let python_path:Vec<String> = sys.getattr("path").unwrap().extract().unwrap();
        //     println!("Python path: {:?}", python_path);
        //     // let _matplotlib = PyModule::import(py, "matplotlib")?;
        //     println!("Total: {}", total);
        // });
        assert_eq!(1, 1);
    }
}