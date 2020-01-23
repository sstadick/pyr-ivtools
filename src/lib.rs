use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;
mod pyrbits;
mod pyrinterval;
mod pyrlapper;
mod pyrscailist;

pub use self::pyrbits::PyrBits;
pub use self::pyrinterval::PyrInterval;
pub use self::pyrlapper::PyrLapper;
pub use self::pyrscailist::PyrScailist;

/// This module is a python module implemented in Rust.PartialEq
#[pymodule]
fn pyr_lapper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyrLapper>()?;
    m.add_class::<PyrInterval>()?;
    m.add_class::<PyrScailist>()?;
    m.add_class::<PyrBits>()?;
    Ok(())
}
