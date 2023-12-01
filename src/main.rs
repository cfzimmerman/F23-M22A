use rulinalg::{
    matrix::{BaseMatrix, Matrix},
    vector::Vector,
};

const TEMPS: [i32; 23] = [
    53, 75, 57, 58, 63, 70, 70, 66, 67, 67, 67, 68, 69, 70, 70, 72, 73, 76, 76, 78, 79, 80, 81,
];
const FAILURES: [i32; 23] = [
    3, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn construct_n(num_rows: usize) -> (Matrix<f64>, Vector<f64>) {
    assert!(num_rows <= TEMPS.len() && num_rows <= FAILURES.len());
    let mut a_input: Vec<f64> = Vec::with_capacity(num_rows * 2);
    for num in TEMPS[0..num_rows].iter() {
        a_input.push((*num).into());
        a_input.push(1.0);
    }
    let a_mtx = Matrix::new(num_rows, 2, a_input);
    let b_vec = Vector::new(
        FAILURES[0..num_rows]
            .iter()
            .map(|num| (*num).into())
            .collect::<Vec<f64>>(),
    );
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

/* OUTPUT

first seven: [
    -0.025393419170243226,
    3.0464949928469256,
]

all: [
    -0.060333333333333246,
    4.644999999999994,
]

*/

/// Currently configured to find the line of best fit for two hard coded vectors.
fn main() {
    let (seven_a, seven_b) = construct_n(7);
    let least_squares_seven = solve_least_squares(seven_a, seven_b).unwrap();

    let (all_a, all_b) = construct_n(TEMPS.len());
    let least_squares_all = solve_least_squares(all_a, all_b).unwrap();
    println!("\nfirst seven: {:#?}", least_squares_seven.data());
    println!("\nall seven: {:#?}", least_squares_all.data());
}
