use ndarray::Array1;
use liba::liba;

fn main() {
	println!(
		"result {}",
		liba(
			&Array1::from(vec![1., 2., 3.]),
			&Array1::from(vec![2., 4., 6.]),
		),
	);
}
