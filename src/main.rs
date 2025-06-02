mod cube;
mod solver;

use cube::move_sequence::Sequence;

use cube::Cube;

fn main() {
    let scramble = Sequence::random_scramble(5);
    println!("Finding solution for {}", scramble);

    let solution = solver::solve(Cube::create_solved().apply(&scramble));
    if let Some(sequence) = solution {
        println!("Found solution: {}", sequence);
        print_link(scramble, sequence);
    } else {
        println!("Didn't find any solution");
    }
}

fn print_link(scramble: Sequence, solution: Sequence) {
    let setup: String = scramble.to_string().replace("'", "-").replace(" ", "_");
    let alg: String = solution.to_string().replace("'", "-").replace(" ", "_");
    println!("https://alg.cubing.net/?setup={setup}&alg={alg}");
}
