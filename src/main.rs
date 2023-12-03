use rulinalg::{
    matrix::{BaseMatrix, Matrix},
    vector::Vector,
};
use std::io::stdin;

/// From x and y returns a matrix A and a vector b such
/// that Ax = b can be solved if the given data's dimensions
/// are correct.
fn setup_matrices(x_nums: &[f64], y_nums: &[f64]) -> (Matrix<f64>, Vector<f64>) {
    let mut a_input: Vec<f64> = Vec::with_capacity(x_nums.len());
    for num in x_nums.iter() {
        a_input.push(*num);
        a_input.push(1.0);
    }
    let a_mtx = Matrix::new(x_nums.len(), 2, a_input);
    let b_vec = Vector::new(y_nums);
    (a_mtx, b_vec)
}

/// Solves a least squares problem for x using the method
/// in the Lay textbook
fn solve_least_squares(
    a_mtx: Matrix<f64>,
    b_vec: Vector<f64>,
) -> Result<Vector<f64>, rulinalg::error::Error> {
    let atransp = a_mtx.transpose();
    let atransp_a = atransp.clone() * a_mtx;
    let atransp_b = atransp * b_vec;
    atransp_a.solve(atransp_b)
}

/// Parses stdin for x,y pairs.
fn pairs_from_stdin() -> (Vec<f64>, Vec<f64>) {
    let mut x_inputs: Vec<f64> = Vec::new();
    let mut y_inputs: Vec<f64> = Vec::new();
    for line in stdin().lines() {
        let safe_line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let splt: Vec<f64> = safe_line
            .split(',')
            .filter_map(|txt| txt.trim().parse::<f64>().ok())
            .collect();
        if let (Some(x), Some(y)) = (splt.first(), splt.get(1)) {
            x_inputs.push(*x);
            y_inputs.push(*y);
        }
    }
    (x_inputs, y_inputs)
}

/// Finds the line of best fit for a series of x,y pairs.
///
/// Ex:
/// cat ch_all.txt | cargo run
fn main() {
    let (x_inputs, y_inputs) = pairs_from_stdin();
    if x_inputs.len() != y_inputs.len() || x_inputs.is_empty() {
        eprintln!(
            "Failed to parse inputs. Stdin looks for x,y pairs \
        of the form `x,y` like `5,9`."
        );
        return;
    }

    let (mtx_a, vec_b) = setup_matrices(x_inputs.as_slice(), y_inputs.as_slice());
    let solved = solve_least_squares(mtx_a, vec_b).unwrap();
    println!("{:?}", solved.data());
}
