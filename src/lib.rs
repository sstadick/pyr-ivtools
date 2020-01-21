use ivtools::{Bits, Interval, IvStore, Lapper, ScAIList};
use pyo3::prelude::*;
use pyo3::types::PyList;
// use pyo3::wrap_pyfunction;

#[pyclass(module = "pyr_lapper")]
struct PyrInterval {
    iv: Interval<PyObject>,
}

#[pymethods]
impl PyrInterval {
    #[new]
    fn new(start: u32, stop: u32, val: PyObject) -> Self {
        PyrInterval {
            iv: Interval {
                start: start,
                stop: stop,
                val: val,
            },
        }
    }
}

#[pyclass]
struct PyrLapper {
    lapper: Lapper<PyObject>,
}

#[pymethods]
impl PyrLapper {
    #[new]
    fn new(ivs: &PyList) -> Self {
        PyrLapper {
            lapper: Lapper::new(
                ivs.into_iter()
                    .map(|iv| match iv.downcast_ref::<PyrInterval>() {
                        Ok(iv) => iv.iv,
                        _ => panic!(),
                    })
                    .collect(),
            ),
        }
    }

    fn find(&self, py: Python<'_>, start: u32, stop: u32) -> PyResult<Vec<PyrInterval>> {
        let mut result = vec![];
        for iv in self.lapper.find(start, stop) {
            result.push(PyrInterval::new(iv.start, iv.stop, iv.val))
        }
        Ok(result)
    }
}

#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// This module is a python module implemented in Rust.PartialEq
#[pymodule]
fn pyr_lapper(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyrLapper>()?;
    m.add_class::<PyrInterval>()?;
    // m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    Ok(())
}
