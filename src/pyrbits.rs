use crate::pyrinterval::PyrInterval;
use ivtools::{Bits, IntervalLike};
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass]
pub struct PyrBits {
    bits: Bits,
}

#[pymethods]
impl PyrBits {
    #[new]
    fn new(ivs: Vec<&PyAny>) -> Self {
        PyrBits {
            bits: Bits::new(
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

    fn count(&self, _py: Python<'_>, start: u32, stop: u32) -> PyResult<usize> {
        Ok(self.bits.count(start, stop))
    }
}
