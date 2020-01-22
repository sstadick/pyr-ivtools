use ivtools::{Bits, Interval, IvStore, Lapper, ScAIList};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};
// use pyo3::wrap_pyfunction;

#[pyclass]
struct PyrInterval {
    #[pyo3(get, set)]
    start: u32,
    #[pyo3(get, set)]
    stop: u32,
    #[pyo3(get, set)]
    val: String,
}

#[pymethods]
impl PyrInterval {
    #[new]
    fn new(start: u32, stop: u32, val: String) -> Self {
        PyrInterval {
            start: start,
            stop: stop,
            val: val,
        }
    }
}

#[pyclass]
struct PyrLapper {
    lapper: Lapper<String>,
}

#[pymethods]
impl PyrLapper {
    #[new]
    fn new(ivs: Vec<&PyAny>) -> Self {
        PyrLapper {
            lapper: Lapper::new(
                ivs.iter()
                    .flat_map(|iv| {
                        if let Ok(iv) = iv.cast_as::<PyrInterval>() {
                            Some(Interval {
                                start: iv.start,
                                stop: iv.stop,
                                val: iv.val.to_string(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
        }
    }

    fn find(&self, py: Python<'_>, start: u32, stop: u32) -> PyResult<Vec<PyrInterval>> {
        let mut result = vec![];
        for iv in self.lapper.find(start, stop) {
            result.push(PyrInterval::new(iv.start, iv.stop, iv.val.clone()))
        }
        Ok(result)
    }
}

/// This module is a python module implemented in Rust.PartialEq
#[pymodule]
fn pyr_lapper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyrLapper>()?;
    m.add_class::<PyrInterval>()?;
    Ok(())
}
