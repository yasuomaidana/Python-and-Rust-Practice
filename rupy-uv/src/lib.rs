use pyo3::prelude::*;

#[pyfunction]
fn custom_divide(a: f64, b: f64) -> String {
    println!("Rust function called with arguments: {}, {}", a, b);
    if b == 0.0 {
        return "Division by zero".to_string();
    }
    format!("{}/{}={:.4}", a, b, a / b)
}

#[pyfunction]
/// A simple function that adds two numbers.
/// # Arguments
///
/// * `a` - A floating point number.
/// * `b` - A floating point number.
///
/// # Returns
///
/// * `f64` - The sum of `a` and `b`.
fn custom_sum(a: f64, b: f64) -> f64 {
    println!("Rust sum: {}+{}= {}", a, b, a+b);
    a + b
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(custom_divide, m)?)?;
    m.add_function(wrap_pyfunction!(custom_sum, m)?)?;
    Ok(())
}
