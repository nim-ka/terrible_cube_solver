use crate::algorithm::{ Algorithm, Move };
use crate::cube::Cube;
use crate::permutation::{ CornerLoc, EdgeLoc, Permutation };

pub struct Solver {
    
}

impl Solver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn solve(&self, cube: &Cube) -> Option<(Algorithm, Permutation)> {
        let state = cube.state;

        let moves = Vec::from(Move::MOVES_LIST);
        let (eo_stage, state) = Self::search(
            state,
            &moves,
            |perm| perm.eo.iter().all(|&o| o == 0),
            |_| 1
        )?;
        println!("EO: {}", eo_stage.to_string());

        let moves = moves.into_iter().filter(|mov| mov.keeps_eo()).collect();
        let (dr_stage, state) = Self::search(
            state,
            &moves,
            |perm| {
                if perm.co.iter().any(|&o| o != 0) {
                    return false
                }

                let belt_edges = vec![
                    EdgeLoc::BR as usize,
                    EdgeLoc::FR as usize,
                    EdgeLoc::FL as usize,
                    EdgeLoc::BL as usize
                ];

                belt_edges.iter().all(|&e| belt_edges.contains(&perm.ep[e]))
            },
            |_| 1
        )?;
        println!("DR: {}", dr_stage.to_string());

        let moves = moves.into_iter().filter(|mov| mov.keeps_domino()).collect();
        let (ht_stage, state) = Self::search(
            state,
            &moves,
            |perm| {
                perm.cp.iter().all(|&p| CornerLoc::from_usize(p).unwrap().ht_class() == CornerLoc::from_usize(perm.cp[p]).unwrap().ht_class()) &&
                perm.ep.iter().all(|&p| EdgeLoc::from_usize(p).unwrap().ht_class() == EdgeLoc::from_usize(perm.ep[p]).unwrap().ht_class())
            },
            |_| 1
        )?;
        println!("HT: {}", ht_stage.to_string());

        let moves = moves.into_iter().filter(|mov| mov.is_halfturn()).collect();
        let (solve_stage, state) = Self::search(
            state,
            &moves,
            |perm| perm == Permutation::ID,
            |_| 1
        )?;
        println!("Solve: {}", solve_stage.to_string());

        let mut alg = Algorithm::new();

        alg
            .append(&eo_stage)
            .append(&dr_stage)
            .append(&ht_stage)
            .append(&solve_stage);

        Some((alg, state))
    }

    pub fn search(
        state: Permutation,
        moves: &Vec<Move>,
        goal: impl Fn(Permutation) -> bool + 'static,
        heuristic: impl Fn(Permutation) -> usize + 'static
    ) -> Option<(Algorithm, Permutation)> {
        let mut alg = Algorithm::new();
        let mut threshold = heuristic(state);

        loop {
            match Self::search_inner(state, moves, &goal, &heuristic, &mut alg, 0, threshold) {
                SearchResult::Success(state) => return Some((alg, state)),
                SearchResult::Failure => return None,
                SearchResult::UpperBound(bound) => threshold = bound
            }

            dbg!(threshold);
        }
    }

    fn search_inner(
        state: Permutation,
        moves: &Vec<Move>,
        goal: &impl Fn(Permutation) -> bool,
        heuristic: &impl Fn(Permutation) -> usize,
        alg: &mut Algorithm,
        depth: usize,
        threshold: usize
    ) -> SearchResult {
        let estimate = depth + heuristic(state);
        if estimate > threshold {
            return SearchResult::UpperBound(estimate);
        }

        if goal(state) {
            return SearchResult::Success(state);
        }

        let mut res = SearchResult::Failure;

        for mov in moves {
            if let Some(mov2) = alg.last() {
                if mov.is_similar(mov2) {
                    continue;
                }
            }

            alg.push(*mov);

            match Self::search_inner(mov.execute(state), moves, goal, heuristic, alg, depth + 1, threshold) {
                SearchResult::Success(state) => return SearchResult::Success(state),
                SearchResult::UpperBound(bound) if match res {
                    SearchResult::Failure => true,
                    SearchResult::UpperBound(bound2) if bound < bound2 => true,
                    _ => false
                } => res = SearchResult::UpperBound(bound),
                _ => ()
            }

            alg.pop();
        }

        res
    }
}

enum SearchResult {
    Success(Permutation),
    Failure,
    UpperBound(usize),
}
