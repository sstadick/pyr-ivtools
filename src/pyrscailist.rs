use crate::pyrinterval::PyrInterval;
use ivtools::{IntervalLike, IvStore, ScAIList};
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass]
pub struct PyrScailist {
    scailist: ScAIList<u32, PyrInterval>,
}

#[pymethods]
impl PyrScailist {
    #[new]
    fn new(ivs: Vec<&PyAny>) -> Self {
        PyrScailist {
            scailist: ScAIList::new(
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

    fn find(&self, _py: Python<'_>, start: u32, stop: u32) -> PyResult<Vec<PyrInterval>> {
        Ok(self
            .scailist
            .find(start, stop)
            .map(|fiv| PyrInterval {
                start: fiv.start(),
                stop: fiv.stop(),
                val: *fiv.val(),
            })
            .collect())
    }

    fn intersect(&self, _py: Python<'_>, other: &Self) -> PyResult<Vec<PyrInterval>> {
        Ok(self
            .scailist
            .iter()
            .flat_map(|iv| {
                let found: Vec<PyrInterval> = other
                    .scailist
                    .find(iv.start(), iv.stop())
                    .map(|fiv| PyrInterval {
                        start: fiv.start(),
                        stop: fiv.stop(),
                        val: *fiv.val(),
                    })
                    .collect();
                found
            })
            .collect())
    }
}
