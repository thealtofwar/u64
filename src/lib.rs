use pyo3::prelude::*;

#[pyclass(eq, ord, name = "u64")]
#[derive(PartialEq, PartialOrd)]
struct U64 {
    inner: u64,
}

#[pymethods]
impl U64 {
    #[new]
    #[inline]
    fn new(value: u64) -> Self {
        U64 { inner: value }
    }

    fn __add__(&self, other: PyRef<'_, U64>) -> U64 {
        Self::new(self.inner + other.inner)
    }

    fn __sub__(&self, other: PyRef<'_, U64>) -> U64 {
        Self::new(self.inner - other.inner)
    }

    fn __floordiv__(&self, other: PyRef<'_, U64>) -> U64 {
        Self::new(self.inner / other.inner)
    }

    fn __int__(&self) -> u64 {
        self.inner
    }

    fn __repr__(&self) -> String {
        format!("u64({})", self.inner)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn u64(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<U64>()?;
    Ok(())
}
