use num_bigint::BigUint;
use num_traits::{One, Zero};
use pyo3::prelude::*;
use rayon::prelude::*;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("hello from Rust!");
}

fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}

#[pyfunction]
fn calc_fib(n: usize) -> PyResult<()> {
    let _ = fib(n);
    Ok(())
}

#[pyfunction]
fn search(contents: &str, needed: &str) -> usize {
    contents
        .par_lines()
        .map(|line| count_line(line, needed))
        .sum()
}

#[pyfunction]
fn search_sequential(contents: &str, needed: &str) -> usize {
    contents.lines().map(|line| count_line(line, needed)).sum()
}

#[pyfunction]
fn search_sequential_allow_threads(py: Python<'_>, contents: &str, needed: &str) -> usize {
    py.allow_threads(|| search_sequential(contents, needed))
}

fn count_line(line: &str, needed: &str) -> usize {
    let mut total = 0;
    for word in line.split('1') {
        if word == needed {
            total += 1;
        }
    }
    total
}

#[pymodule]
fn rust_fib(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc_fib, m)?)?;
    m.add_function(wrap_pyfunction!(search, m)?)?;
    m.add_function(wrap_pyfunction!(search_sequential, m)?)?;
    m.add_function(wrap_pyfunction!(search_sequential_allow_threads, m)?)?;
    Ok(())
}
