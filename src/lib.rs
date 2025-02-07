use pyo3::{
    exceptions::{PyOverflowError, PyZeroDivisionError},
    prelude::*,
    types::*,
};

#[pyclass(eq, ord, name = "u64")]
#[derive(PartialEq, PartialOrd, Clone)]
struct U64 {
    inner: u64,
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
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

    fn __mul__(&self, other: PyRef<'_, U64>) -> U64 {
        Self::new(self.inner * other.inner)
    }

    fn __pow__(&self, other: PyRef<'_, U64>, modulo: Option<PyRef<'_, U64>>) -> PyResult<U64> {
        if self.inner <= 1 {
            return Ok(self.clone());
        }
        match modulo {
            Some(modulo) => Ok(Self::new(mod_pow(self.inner, other.inner, modulo.inner))),
            None => {
                if other.inner > u32::MAX.into() {
                    Err(PyOverflowError::new_err("other is too big"))
                } else {
                    match self.inner.checked_pow(other.inner as u32) {
                        Some(result) => Ok(Self::new(result)),
                        None => Err(PyOverflowError::new_err("result overflowed")),
                    }
                }
            }
        }
    }

    fn __mod__(&self, other: PyRef<'_, U64>) -> PyResult<U64> {
        if other.inner == 0 {
            return Err(PyZeroDivisionError::new_err("integer modulo by zero"));
        }
        Ok(Self::new(self.inner % other.inner))
    }

    fn __divmod__(&self, py: Python, other: PyRef<'_, U64>) -> PyObject {
        let a = self.inner;
        let b = other.inner;
        let result = [a / b, a % b];
        let tuple = PyTuple::new(py, result).unwrap();
        tuple.into()
    }

    fn __floordiv__(&self, other: PyRef<'_, U64>) -> PyResult<U64> {
        if other.inner == 0 {
            return Err(PyZeroDivisionError::new_err("integer modulo by zero"));
        }
        Ok(Self::new(self.inner / other.inner))
    }

    fn __truediv__(&self, other: PyRef<'_, U64>) -> PyResult<U64> {
        if other.inner == 0 {
            return Err(PyZeroDivisionError::new_err("integer modulo by zero"));
        }
        Ok(Self::new(self.inner / other.inner))
    }

    fn __lshift__(&self, other: PyRef<'_, U64>) -> U64 {
        Self::new(self.inner << other.inner)
    }

    fn __rshift__(&self, other: PyRef<'_, U64>) -> U64 {
        Self::new(self.inner >> other.inner)
    }

    fn __and__(&self, other: PyRef<'_, U64>) -> U64 {
        Self::new(self.inner & other.inner)
    }

    fn __xor__(&self, other: PyRef<'_, U64>) -> U64 {
        Self::new(self.inner ^ other.inner)
    }

    fn __or__(&self, other: PyRef<'_, U64>) -> U64 {
        Self::new(self.inner | other.inner)
    }

    fn __abs__(&self) -> U64 {
        self.clone()
    }

    fn __trunc__(&self) -> U64 {
        self.clone()
    }

    fn __bool__(&self) -> bool {
        self.inner != 0
    }

    fn __invert__(&self) -> U64 {
        Self::new(!self.inner)
    }

    fn __ceil__(&self) -> U64 {
        self.clone()
    }

    fn __round__(&self) -> U64 {
        self.clone()
    }

    fn __floor__(&self) -> U64 {
        self.clone()
    }

    fn __index__(&self) -> u64 {
        self.inner
    }

    fn __int__(&self) -> u64 {
        self.inner
    }

    fn __pos__(&self) -> U64 {
        self.clone()
    }

    fn __float__(&self) -> f64 {
        self.inner as f64
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
