mod number_list;

use std::collections::{HashMap, HashSet};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{pyfunction, wrap_pyfunction, PyResult};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use crate::number_list::NumbersList;

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

#[pyfunction]
fn data_types(py: Python<'_>) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("int", 42)?;
    dict.set_item("float", 3.14)?;
    dict.set_item("str", "Hello, World!")?;
    dict.set_item("bool", true)?;
    dict.set_item("list", vec![1, 2, 3])?;
    dict.set_item("tuple", (1, 2, 3))?;
    dict.set_item("set", vec![1, 2, 3].into_iter().collect::<HashSet<_>>())?;
    dict.set_item(
        "dict",
        vec![(1, "one"), (2, "two")]
            .into_iter()
            .collect::<HashMap<_, _>>(),
    )?;
    Ok(dict.into())
}

#[pymodule]
fn digits_pi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_pi, m)?)?;
    m.add_function(wrap_pyfunction!(data_types, m)?)?;
    m.add_class::<NumbersList>()?;
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
