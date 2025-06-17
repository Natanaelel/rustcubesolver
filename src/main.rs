mod cube;
mod solver;

use std::time::Instant;

use cube::move_sequence::Sequence;

use cube::Cube;

use crate::cube::simple_move::SimpleMove;

type M = SimpleMove;

fn main() {
    // let scramble = Sequence::random_scramble(5);
    let scramble: Sequence<SimpleMove> = String::from("R U R' U' R U").parse().unwrap();
    println!("Finding solution for {}", scramble);

    let start_time = Instant::now();
    let solution = solver::solve(Cube::create_solved().apply(&scramble));
    let time_taken = Instant::now() - start_time; // 6.143426903s --release: 387.960225ms

    if let Some(sequence) = solution {
        println!("Found solution in {:?}: {}", time_taken, sequence);
        print_link(&scramble, &sequence);
        println!("{}", Cube::create_solved().apply(&scramble))
    } else {
        println!("Didn't find any solution");
    }
}

fn print_link(scramble: &Sequence<M>, solution: &Sequence<M>) {
    let setup: String = scramble.to_string().replace("'", "-").replace(" ", "_");
    let alg: String = solution.to_string().replace("'", "-").replace(" ", "_");
    println!("https://alg.cubing.net/?setup={setup}&alg={alg}");
}
