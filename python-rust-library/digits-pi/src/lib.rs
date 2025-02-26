use pyo3::prelude::*;
use pyo3::{pyfunction, wrap_pyfunction, PyResult};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

/// Calculate the value of π using the Leibniz with  Shanks transformation formula.
#[pyfunction]
fn calculate_pi(iterations: u32) -> PyResult<f64> {
    let pi = (0..iterations)
        .into_par_iter()
        .map(|i| {
            let i = i as f64;
            let den = (4.0 * i + 1.0) * (4.0 * i + 3.0);
            2.0 / den
        })
        .sum::<f64>()
        * 4.0;
    Ok(pi)
}

#[pymodule]
fn digits_pi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_pi, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_pi() {
        let result = calculate_pi(44).unwrap();
        let expected = 3.14; // Approximate value of π
        assert!((result - expected).abs() < 0.01);
    }
}
