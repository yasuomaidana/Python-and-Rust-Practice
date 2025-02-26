use pyo3::prelude::*;
use pyo3::{pyfunction, wrap_pyfunction, PyResult, Python};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

/// Calculate the value of π using the Leibniz formula.
#[pyfunction]
fn calculate_pi(iterations: u32) -> PyResult<f64> {
    let pi = (0..iterations)
        .into_par_iter()
        .map(|i| {
            let i = i as f64;
            (-1_f64).powf(i) / (2.0 * i + 1.0)
        })
        .sum::<f64>()
        * 4.0;
    Ok(pi)
}

// #[pymodule]
// fn lib_pi_digits(py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(calculate_pi, py)?)?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_pi() {
        let result = calculate_pi(90).unwrap();
        let expected = 3.14; // Approximate value of π
        assert!((result - expected).abs() < 0.01);
    }
}