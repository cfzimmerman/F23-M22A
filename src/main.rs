use rulinalg::{
    matrix::{BaseMatrix, Matrix},
    vector::Vector,
};
use std::io::stdin;

fn setup_matrices(x_nums: &[f64], y_nums: &[f64]) -> (Matrix<f64>, Vector<f64>) {
    let mut a_input: Vec<f64> = Vec::with_capacity(x_nums.len());
    for num in x_nums.iter() {
        a_input.push((*num).into());
        a_input.push(1.0);
    }
    let a_mtx = Matrix::new(x_nums.len(), 2, a_input);
    let b_vec = Vector::new(y_nums);
    (a_mtx, b_vec)
}

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
    for line in stdin().lines().into_iter() {
        let safe_line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let splt: Vec<f64> = safe_line
            .split(",")
            .filter_map(|txt| txt.trim().parse::<f64>().ok())
            .collect();
        if splt.len() < 2 {
            continue;
        }
        x_inputs.push(splt[0]);
        y_inputs.push(splt[1]);
    }
    (x_inputs, y_inputs)
}

/// Finds the line of best fit for a series of x,y pairs.
/// cat ch_7.txt | cargo run
/// cat ch_all.txt | cargo run
fn main() {
    let (x_inputs, y_inputs) = pairs_from_stdin();
    if x_inputs.len() != y_inputs.len() || x_inputs.len() == 0 {
        eprintln!(
            "Failed to parse inputs. Stdin looks for x,y pairs \
        of the form `x,y` like `5,9`."
        );
    }

    let (mtx_a, vec_b) = setup_matrices(x_inputs.as_slice(), y_inputs.as_slice());
    let solved = solve_least_squares(mtx_a, vec_b).unwrap();
    println!("{:?}", solved.data());
}
