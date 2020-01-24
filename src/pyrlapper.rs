use crate::pyrinterval::PyrInterval;
use ivtools::{IntervalLike, IvStore, Lapper};
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass]
pub struct PyrLapper {
    lapper: Lapper<u32, PyrInterval>,
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
                            Some(PyrInterval {
                                start: iv.start(),
                                stop: iv.stop(),
                                val: *iv.val(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
        }
    }

    fn find(&self, _py: Python<'_>, start: u32, stop: u32) -> PyResult<Vec<&PyrInterval>> {
        Ok(self
            .lapper
            .find(start, stop)
            // .map(|fiv| PyrInterval {
            //     start: fiv.start(),
            //     stop: fiv.stop(),
            //     val: *fiv.val(),
            // })
            .collect())
    }

    fn intersect<'a>(&'a self, _py: Python<'_>, other: &'a Self) -> PyResult<Vec<&'a PyrInterval>> {
        let mut cursor = 0;
        Ok(self
            .lapper
            .iter()
            .flat_map(|iv| {
                let found: Vec<&PyrInterval> = other
                    .lapper
                    .seek(iv.start(), iv.stop(), &mut cursor)
                    // .map(|fiv| PyrInterval {
                    //     start: fiv.start(),
                    //     stop: fiv.stop(),
                    //     val: *fiv.val(),
                    // })
                    .collect();
                found
            })
            .collect())
    }
}
