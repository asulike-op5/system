use ndarray::Array2;

pub struct Encoder;

impl Encoder {
    pub fn encode(batch: &Array2<f64>) -> Array2<f64> {
        // L2 normalize + tiny noise + re-normalize
        let mut z = batch.clone();
        for mut row in z.rows_mut() {
            let norm = row.dot(&row).sqrt();
            if norm > 1e-6 {
                row /= norm;
                // tiny noise
                for val in row.iter_mut() {
                    *val += (rand::random::<f64>() - 0.5) * 1e-4;
                }
                // re-normalize
                let norm2 = row.dot(&row).sqrt();
                if norm2 > 1e-6 {
                    row /= norm2;
                }
            }
        }
        z
    }
}
