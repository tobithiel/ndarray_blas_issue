use ndarray::Array2;
use ndarray_linalg::SVD;

pub fn libb(mat: &Array2<f64>) -> f64 {
	let (u_opt, s, vt_opt) = mat
		.svd(true, true)
		.expect("Could not calculate SVD");
	let u = u_opt.unwrap();
	let vt = vt_opt.unwrap();

	u.sum() + s.sum() + vt.sum()
}
