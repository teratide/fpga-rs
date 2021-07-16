use fpga::xrt::Xrt;
use pyo3::prelude::*;

#[pyclass(unsendable)]
pub struct PyXrt {
    pub xrt: Xrt,
}

#[pyfunction]
fn xrt(device_index: usize) -> PyResult<PyXrt> {
    Ok(PyXrt {
        xrt: Xrt::from_device_index(device_index)?,
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn fpga_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(xrt, m)?)?;
    Ok(())
}
