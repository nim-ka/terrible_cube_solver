use crate::permutation::Permutation;

pub struct Algorithm {
    moves: Vec<Move>,
}

impl Algorithm {
    pub fn new() -> Algorithm {
        Algorithm {
            moves: vec![]
        }
    }

    pub fn from_str(algorithm: &str) -> Result<Algorithm, &'static str> {
        let mut moves: Vec<Move> = vec![];

        for word in algorithm.split_whitespace() {
            moves.push(Move::from_str(word)?);
        }

        Ok(Algorithm { moves })
    }

    pub fn reset(&mut self) {
        self.moves.clear();
    }

    pub fn push(&mut self, mov: Move) {
        self.moves.push(mov);
    }

    pub fn pop(&mut self) -> Option<Move> {
        self.moves.pop()
    }

    pub fn append(&mut self, other: &Algorithm) -> &mut Algorithm {
        for mov in &other.moves {
            self.push(*mov);
        }

        self
    }

    pub fn last(&self) -> Option<Move> {
        self.moves.last().copied()
    }

    pub fn execute(&self, perm: Permutation) -> Permutation {
        let mut res = perm;

        for mov in &self.moves {
            res = mov.execute(res);
        }

        res
    }

    pub fn inverse(&self) -> Algorithm {
        Algorithm {
            moves: self.moves
                .iter()
                .rev()
                .map(|mov| mov.inverse())
                .collect()
        }
    }

    pub fn to_string(&self) -> String {
        self.moves
            .iter()
            .map(|mov| mov.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Move(MoveBase, isize);

impl Move {
    pub const MOVES_LIST: [Move; 18] = [
        Move(MoveBase::U, 1), Move(MoveBase::U, 2), Move(MoveBase::U, -1),
        Move(MoveBase::D, 1), Move(MoveBase::D, 2), Move(MoveBase::D, -1),
        Move(MoveBase::L, 1), Move(MoveBase::L, 2), Move(MoveBase::L, -1),
        Move(MoveBase::R, 1), Move(MoveBase::R, 2), Move(MoveBase::R, -1),
        Move(MoveBase::F, 1), Move(MoveBase::F, 2), Move(MoveBase::F, -1),
        Move(MoveBase::B, 1), Move(MoveBase::B, 2), Move(MoveBase::B, -1),
    ];

    pub fn from_str(name: &str) -> Result<Move, &'static str> {
        let mut modifier = 1;

        let name = match name.strip_suffix("'") {
            Some(s) => { modifier = -1; s },
            None => name
        };

        let name = match name.strip_suffix("2") {
            Some(s) => { modifier = 2; s },
            None => name
        };

        Ok(Move(MoveBase::from_str(name)?, modifier))
    }

    pub fn inverse(self) -> Move {
        Move(self.0, if self.1 == 2 { 2 } else { -self.1 })
    }

    pub fn is_similar(self, other: Move) -> bool {
        self.0 == other.0
    }

    pub fn keeps_eo(self) -> bool {
        !((self.0 == MoveBase::F || self.0 == MoveBase::B) && self.1 != 2)
    }

    pub fn keeps_domino(self) -> bool {
        self.1 == 2 || self.0 == MoveBase::U || self.0 == MoveBase::D
    }

    pub fn is_halfturn(self) -> bool {
        self.1 == 2
    }

    pub fn execute(self, perm: Permutation) -> Permutation {
        perm + self.permutation()
    }

    pub fn to_string(self) -> String {
        self.0.to_string() + match self.1 {
            1 => "",
            2 => "2",
            -1 => "'",
            _ => panic!("Unrecognized move modifier")
        }
    }

    fn permutation(self) -> Permutation {
        self.0.permutation() * self.1
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum MoveBase {
    U,
    D,
    L,
    R,
    F,
    B,
}

impl MoveBase {
    fn from_str(name: &str) -> Result<MoveBase, &'static str> {
        match name {
            "U" => Ok(MoveBase::U),
            "D" => Ok(MoveBase::D),
            "L" => Ok(MoveBase::L),
            "R" => Ok(MoveBase::R),
            "F" => Ok(MoveBase::F),
            "B" => Ok(MoveBase::B),
            _ => Err("Could not parse move")
        }
    }

    fn to_string(self) -> String {
        String::from(match self {
            MoveBase::U => "U",
            MoveBase::D => "D",
            MoveBase::L => "L",
            MoveBase::R => "R",
            MoveBase::F => "F",
            MoveBase::B => "B"
        })
    }

    fn permutation(self) -> Permutation {
        match self {
            MoveBase::U => Permutation::U,
            MoveBase::D => Permutation::D,
            MoveBase::L => Permutation::L,
            MoveBase::R => Permutation::R,
            MoveBase::F => Permutation::F,
            MoveBase::B => Permutation::B
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Algorithm;
    use super::Permutation;

    #[test]
    fn alg_and_inverse_makes_id() {
        let alg = Algorithm::from_str("R U L' D B D2 F L").unwrap();
        let inv = alg.inverse();
        assert_eq!(inv.execute(alg.execute(Permutation::ID)), Permutation::ID);
    }

    #[test]
    fn alg_and_non_inverse_makes_not_id() {
        let mut alg = Algorithm::from_str("R U L' D B D2 F L").unwrap();
        let inv = alg.inverse();
        alg.pop();
        assert_ne!(inv.execute(alg.execute(Permutation::ID)), Permutation::ID);
    }

    #[test]
    fn superflip() {
        let superflip1 = Algorithm::from_str("U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2").unwrap();
        let superflip2 = Algorithm::from_str("R' U2 B L' F U' B D F U D' L D2 F' R B' D F' U' B' U D'").unwrap();
        assert_eq!(superflip1.execute(Permutation::ID), superflip2.execute(Permutation::ID));
    }
}
