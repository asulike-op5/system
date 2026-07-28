use pyo3::prelude::*;

#[pyclass]
pub struct PyStructuralSystem {
    inner: crate::StructuralSystem,
    dim: usize,
}

#[pymethods]
impl PyStructuralSystem {
    #[new]
    fn new(dim: usize) -> Self {
        Self { inner: crate::StructuralSystem::new(dim), dim }
    }

    /// Accept a Python list of floats (batch of embeddings).
    /// In production this accepts numpy.ndarray via PyReadonlyArray2.
    fn step(&mut self, py: Python, batch: Vec<Vec<f64>>) -> PyResult<()> {
        let rows = batch.len();
        if rows == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err("Batch must have at least one row"));
        }
        // Basic dimension validation
        for row in &batch {
            if row.len() != self.dim {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    format!("Expected dim {}, got {}", self.dim, row.len())
                ));
            }
        }
        // Production: convert to ndarray::Array2 and call inner.step()
        let _ = (py, batch);
        Ok(())
    }
}

#[pymodule]
fn structural_system(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyStructuralSystem>()?;
    Ok(())
}
