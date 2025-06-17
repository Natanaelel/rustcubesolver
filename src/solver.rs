use crate::cube::{Cube, r#move::Move, move_sequence::Sequence};

pub fn solve<T: Move>(cube: Cube) -> Option<Sequence<T>> {
    println!("trying to solve the cube {cube:?}");

    let solved = Cube::create_solved();
    let mut iteration = vec![Snapshot {
        cube: cube.clone(),
        sequence: Sequence::new(),
    }];

    if cube == solved {
        return Some(Sequence::new());
    }

    let iterations = 10;
    for i in 1..=iterations {
        let mut next_iter = Vec::new();
        println!("Searching {} states, move {}", iteration.len(), i);
        for snapshot in &iteration {
            for r#move in T::all_moves() {
                let state = Snapshot {
                    cube: snapshot.cube.apply_move(&r#move),
                    sequence: snapshot.sequence.apply(&r#move),
                };
                if state.cube == solved {
                    return Some(state.sequence);
                }
                next_iter.push(state);
            }
        }
        iteration = next_iter
    }

    None
}

struct Snapshot<T: Move> {
    cube: Cube,
    sequence: Sequence<T>,
}
