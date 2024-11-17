use std::array;

use crate::algorithm::Algorithm;
use crate::permutation::{ CornerLoc, EdgeLoc, Permutation };

#[derive(Debug)]
pub struct Cube {
pub    state: Permutation,
}

impl Cube {
    const CORNERS: [Corner; 8] = [
        Corner(Color::WHITE, Color::RED, Color::GREEN),
        Corner(Color::WHITE, Color::GREEN, Color::ORANGE),
        Corner(Color::WHITE, Color::ORANGE, Color::BLUE),
        Corner(Color::WHITE, Color::BLUE, Color::RED),
        Corner(Color::YELLOW, Color::GREEN, Color::RED),
        Corner(Color::YELLOW, Color::ORANGE, Color::GREEN),
        Corner(Color::YELLOW, Color::BLUE, Color::ORANGE),
        Corner(Color::YELLOW, Color::RED, Color::BLUE)
    ];

    const EDGES: [Edge; 12] = [
        Edge(Color::WHITE, Color::RED),
        Edge(Color::WHITE, Color::GREEN),
        Edge(Color::WHITE, Color::ORANGE),
        Edge(Color::WHITE, Color::BLUE),
        Edge(Color::BLUE, Color::RED),
        Edge(Color::GREEN, Color::RED),
        Edge(Color::GREEN, Color::ORANGE),
        Edge(Color::BLUE, Color::ORANGE),
        Edge(Color::YELLOW, Color::RED),
        Edge(Color::YELLOW, Color::GREEN),
        Edge(Color::YELLOW, Color::ORANGE),
        Edge(Color::YELLOW, Color::BLUE)
    ];

    pub fn new() -> Self {
        Self {
            state: Permutation::ID
        }
    }

    pub fn execute(&self, alg: &Algorithm) -> Self {
        Self {
            state: alg.execute(self.state)
        }
    }

    pub fn execute_mut(&mut self, alg: &Algorithm) {
        self.state = alg.execute(self.state);
    }

    pub fn print(&self) {
        let corners: [Corner; 8] = array::from_fn(|i| {
            let (o, p) = self.state.corner_op(CornerLoc::from_usize(i).unwrap()).unwrap();
            let c = &Cube::CORNERS[p as usize];

            match o {
                0 => Corner(c.0, c.1, c.2),
                1 => Corner(c.2, c.0, c.1),
                2 => Corner(c.1, c.2, c.0),
                _ => panic!("Unexpected corner orientation {o}")
            }
        });

        let edges: [Edge; 12] = array::from_fn(|i| {
            let (o, p) = self.state.edge_op(EdgeLoc::from_usize(i).unwrap()).unwrap();
            let e = &Cube::EDGES[p as usize];

            match o {
                0 => Edge(e.0, e.1),
                1 => Edge(e.1, e.0),
                _ => panic!("Unexpected corner orientation {o}")
            }
        });

        let u: [String; 9] = [
            corners[CornerLoc::UBL as usize].0, edges[EdgeLoc::UB as usize].0, corners[CornerLoc::UBR as usize].0,
            edges[EdgeLoc::UL as usize].0,      Color::WHITE,                  edges[EdgeLoc::UR as usize].0,
            corners[CornerLoc::UFL as usize].0, edges[EdgeLoc::UF as usize].0, corners[CornerLoc::UFR as usize].0
        ].map(|color| color.to_string());

        let d: [String; 9] = [
            corners[CornerLoc::DFL as usize].0, edges[EdgeLoc::DF as usize].0, corners[CornerLoc::DFR as usize].0,
            edges[EdgeLoc::DL as usize].0,      Color::YELLOW,                 edges[EdgeLoc::DR as usize].0,
            corners[CornerLoc::DBL as usize].0, edges[EdgeLoc::DB as usize].0, corners[CornerLoc::DBR as usize].0
        ].map(|color| color.to_string());

        let l: [String; 9] = [
            corners[CornerLoc::UBL as usize].1, edges[EdgeLoc::UL as usize].1, corners[CornerLoc::UFL as usize].2,
            edges[EdgeLoc::BL as usize].1,      Color::ORANGE,                 edges[EdgeLoc::FL as usize].1,
            corners[CornerLoc::DBL as usize].2, edges[EdgeLoc::DL as usize].1, corners[CornerLoc::DFL as usize].1
        ].map(|color| color.to_string());

        let r: [String; 9] = [
            corners[CornerLoc::UFR as usize].1, edges[EdgeLoc::UR as usize].1, corners[CornerLoc::UBR as usize].2,
            edges[EdgeLoc::FR as usize].1,      Color::RED,                    edges[EdgeLoc::BR as usize].1,
            corners[CornerLoc::DFR as usize].2, edges[EdgeLoc::DR as usize].1, corners[CornerLoc::DBR as usize].1
        ].map(|color| color.to_string());

        let f: [String; 9] = [
            corners[CornerLoc::UFL as usize].1, edges[EdgeLoc::UF as usize].1, corners[CornerLoc::UFR as usize].2,
            edges[EdgeLoc::FL as usize].0,      Color::GREEN,                  edges[EdgeLoc::FR as usize].0,
            corners[CornerLoc::DFL as usize].2, edges[EdgeLoc::DF as usize].1, corners[CornerLoc::DFR as usize].1
        ].map(|color| color.to_string());

        let b: [String; 9] = [
            corners[CornerLoc::UBR as usize].1, edges[EdgeLoc::UB as usize].1, corners[CornerLoc::UBL as usize].2,
            edges[EdgeLoc::BR as usize].0,      Color::BLUE,                   edges[EdgeLoc::BL as usize].0,
            corners[CornerLoc::DBR as usize].2, edges[EdgeLoc::DB as usize].1, corners[CornerLoc::DBL as usize].1
        ].map(|color| color.to_string());

        let pad = Color::NONE.to_string();

        println!("{}{}{}{}{}{}
{}{}{}{}{}{}
{}{}{}{}{}{}
{}{}{}{}{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}{}{}{}{}
{}{}{}{}{}{}
{}{}{}{}{}{}
{}{}{}{}{}{}",
            pad, pad, pad, u[0], u[1], u[2],
            pad, pad, pad, u[3], u[4], u[5],
            pad, pad, pad, u[6], u[7], u[8],
            l[0], l[1], l[2], f[0], f[1], f[2], r[0], r[1], r[2], b[0], b[1], b[2],
            l[3], l[4], l[5], f[3], f[4], f[5], r[3], r[4], r[5], b[3], b[4], b[5],
            l[6], l[7], l[8], f[6], f[7], f[8], r[6], r[7], r[8], b[6], b[7], b[8],
            pad, pad, pad, d[0], d[1], d[2],
            pad, pad, pad, d[3], d[4], d[5],
            pad, pad, pad, d[6], d[7], d[8]
        );
    }
}

#[derive(Clone, Copy)]
enum Color {
    NONE,
    WHITE,
    GREEN,
    RED,
    BLUE,
    ORANGE,
    YELLOW,
}

impl Color {
    fn to_string(self) -> String {
        format!("\x1b[{}m  \x1b[0m", match self {
            Color::NONE => 0,
            Color::WHITE => 107,
            Color::GREEN => 102,
            Color::RED => 101,
            Color::BLUE => 104,
            Color::ORANGE => 105,
            Color::YELLOW => 103
        })
    }
}

struct Corner(Color, Color, Color);
struct Edge(Color, Color);
