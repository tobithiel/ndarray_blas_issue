use ndarray::Array1;

pub fn liba(arr1: &Array1<f64>, arr2: &Array1<f64>) -> f64 {
	arr1.dot(arr2)
}
