use std::env;

use cube::cube::Cube;
use cube::algorithm::Algorithm;
use cube::solver::Solver;

fn main() {
    let alg = env::args().skip(1).next().unwrap();
    println!("Solving scramble: {alg}");

    let mut cube = Cube::new();
    let alg = Algorithm::from_str(&alg[..]).unwrap();
    cube.execute_mut(&alg);

    let solver = Solver::new();
    let (alg, _) = solver.solve(&cube).unwrap();

    println!("{}", alg.to_string());
}
