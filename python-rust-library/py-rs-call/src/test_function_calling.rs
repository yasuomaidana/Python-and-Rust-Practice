#[cfg(test)]
mod test_function_calling {
    use pyo3::ffi::c_str;
    use pyo3::prelude::*;
    use pyo3::prepare_freethreaded_python;

    #[test]
    fn test_calling_python_function() {
        prepare_freethreaded_python();
        let single_function = c_str!(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../to_call/single_function.py"
        )));

        Python::with_gil(|py| {
            let to_call = PyModule::from_code(
                py,
                single_function,
                c_str!("single_function"),
                c_str!("to_call"),
            )
            .unwrap();

            let args = (1, 2);
            let total: i32 = to_call
                .getattr("simple_sum")
                .unwrap()
                .call1(args)
                .unwrap()
                .extract()
                .unwrap();
            assert_eq!(total, 3);
        });
    }

    #[test]
    /* It is MUCH easier to install the desired Python program as a package and then import it
    from the environment. This way, we can avoid the need to pass the code as files
     */
    fn test_complex_python_function() {
        prepare_freethreaded_python();
        let single_function = c_str!(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../to_call/single_function.py"
        )));

        let nested_sum = c_str!(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../to_call/nested_sum.py"
        )));

        let complex_function = c_str!(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../to_call/complex_function.py"
        )));

        Python::with_gil(|py| {
            PyModule::from_code(
                py,
                single_function,
                c_str!("to_call.single_function"),
                c_str!("to_call.single_function"),
            )
            .unwrap();

            PyModule::from_code(
                py,
                nested_sum,
                c_str!("to_call.nested_sum"),
                c_str!("to_call.nested_sum"),
            )
            .unwrap();

            let to_call = PyModule::from_code(
                py,
                complex_function,
                c_str!("complex_function"),
                c_str!("to_call"),
            )
            .unwrap();

            let args = (5,);
            let total: i32 = to_call
                .getattr("complex_sum")
                .unwrap()
                .call1(args)
                .unwrap()
                .extract()
                .unwrap();
            assert_eq!(total, 26);
        });
    }
}
