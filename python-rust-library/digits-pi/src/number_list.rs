use pyo3::{pyclass, pymethods};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[pyclass]
pub struct NumbersList {
    numbers: Vec<i32>,
}

#[pymethods]
impl NumbersList {
    #[new]
    pub fn new() -> NumbersList {
        NumbersList {
            numbers: Vec::new(),
        }
    }

    pub fn sum(&self) -> i32 {
        if self.numbers.is_empty() {
            0
        } else if self.numbers.len() < 1000 {
            self.numbers.iter().sum()
        } else {
            self.numbers.par_iter().sum()
        }
    }
    
    pub fn clear(&mut self) {
        self.numbers.clear();
    }
    
    pub fn add(&mut self, number: i32) {
        self.numbers.push(number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_list() {
        let mut numbers_list = NumbersList::new();
        assert_eq!(numbers_list.sum(), 0);
        numbers_list.numbers.push(1);
        numbers_list.numbers.push(2);
        numbers_list.numbers.push(3);
        assert_eq!(numbers_list.sum(), 6);
    }
    #[test]
    fn test_numbers_list_parallel() {
        let mut numbers_list = NumbersList::new();
        for i in 0..1000 {
            numbers_list.numbers.push(i);
        }
        assert_eq!(numbers_list.sum(), 499500);
    }
}
