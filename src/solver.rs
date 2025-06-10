use crate::cube::{Cube, r#move::ALL_MOVES, move_sequence::Sequence};

pub fn solve(cube: Cube) -> Option<Sequence> {
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
    for _ in 0..iterations {
        let mut next_iter = Vec::new();
        println!("Searching {} states", iteration.len());
        for snapshot in &iteration {
            for r#move in ALL_MOVES {
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

struct Snapshot {
    cube: Cube,
    sequence: Sequence,
}
