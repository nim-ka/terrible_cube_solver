use std::ops::{Add, Neg, Mul};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CornerLoc {
    UFR,
    UFL,
    UBL,
    UBR,
    DFR,
    DFL,
    DBL,
    DBR,
}

impl CornerLoc {
    pub fn from_usize(n: usize) -> Result<Self, &'static str> {
        match n {
            0 => Ok(Self::UFR),
            1 => Ok(Self::UFL),
            2 => Ok(Self::UBL),
            3 => Ok(Self::UBR),
            4 => Ok(Self::DFR),
            5 => Ok(Self::DFL),
            6 => Ok(Self::DBL),
            7 => Ok(Self::DBR),
            _ => Err("Nonexistent corner index")
        }
    }

    pub fn ht_class(self) -> usize {
        match self {
            CornerLoc::UFR => 0,
            CornerLoc::UFL => 1,
            CornerLoc::UBL => 0,
            CornerLoc::UBR => 1,
            CornerLoc::DFR => 1,
            CornerLoc::DFL => 0,
            CornerLoc::DBL => 1,
            CornerLoc::DBR => 0
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EdgeLoc {
    UR,
    UF,
    UL,
    UB,
    BR,
    FR,
    FL,
    BL,
    DR,
    DF,
    DL,
    DB,
}

impl EdgeLoc {
    pub fn from_usize(n: usize) -> Result<Self, &'static str> {
        match n {
            0 => Ok(Self::UR),
            1 => Ok(Self::UF),
            2 => Ok(Self::UL),
            3 => Ok(Self::UB),
            4 => Ok(Self::BR),
            5 => Ok(Self::FR),
            6 => Ok(Self::FL),
            7 => Ok(Self::BL),
            8 => Ok(Self::DR),
            9 => Ok(Self::DF),
            10 => Ok(Self::DL),
            11 => Ok(Self::DB),
            _ => Err("Nonexistent edge index")
        }
    }

    pub fn ht_class(self) -> usize {
        match self {
            EdgeLoc::UR => 0,
            EdgeLoc::UF => 1,
            EdgeLoc::UL => 0,
            EdgeLoc::UB => 1,
            EdgeLoc::BR => 2,
            EdgeLoc::FR => 2,
            EdgeLoc::FL => 2,
            EdgeLoc::BL => 2,
            EdgeLoc::DR => 0,
            EdgeLoc::DF => 1,
            EdgeLoc::DL => 0,
            EdgeLoc::DB => 1
        }
    }
}

// co[UFR] is the orientation of the piece currently in UFR
// cp[UFR] is the original position index of the piece currently in UFR
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Permutation {
pub    co: [usize; 8],
pub    eo: [usize; 12],
pub    cp: [usize; 8],
pub    ep: [usize; 12],
}

impl Permutation {
    pub const ID: Self = Self {
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [0, 1, 2, 3, 4, 5, 6, 7],
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    };

    pub const U: Self = Self {
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [3, 0, 1, 2, 4, 5, 6, 7],
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
    };

    pub const D: Self = Self {
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [0, 1, 2, 3, 5, 6, 7, 4],
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
    };

    pub const L: Self = Self {
        co: [0, 1, 2, 0, 0, 2, 1, 0],
        cp: [0, 2, 6, 3, 4, 1, 5, 7],
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 7, 3, 4, 5, 2, 10, 8, 9, 6, 11],
    };

    pub const R: Self = Self {
        co: [2, 0, 0, 1, 1, 0, 0, 2],
        cp: [4, 1, 2, 0, 7, 5, 6, 3],
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [5, 1, 2, 3, 0, 8, 6, 7, 4, 9, 10, 11],
    };

    pub const F: Self = Self {
        co: [1, 2, 0, 0, 2, 1, 0, 0],
        cp: [1, 5, 2, 3, 0, 4, 6, 7],
        eo: [0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0],
        ep: [0, 6, 2, 3, 4, 1, 9, 7, 8, 5, 10, 11],
    };

    pub const B: Self = Self {
        co: [0, 0, 1, 2, 0, 0, 2, 1],
        cp: [0, 1, 3, 7, 4, 5, 2, 6],
        eo: [0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1],
        ep: [0, 1, 2, 4, 11, 5, 6, 3, 8, 9, 10, 7],
    };

    pub fn corner_op(&self, i: CornerLoc) -> Result<(usize, CornerLoc), &'static str> {
        Ok((self.co[i as usize], CornerLoc::from_usize(self.cp[i as usize])?))
    }

    pub fn edge_op(&self, i: EdgeLoc) -> Result<(usize, EdgeLoc), &'static str> {
        Ok((self.eo[i as usize], EdgeLoc::from_usize(self.ep[i as usize])?))
    }
}

impl Neg for Permutation {
    type Output = Self;

    fn neg(self) -> Self {
        let mut new = self;

        for i in 0..8 {
            new.co[self.cp[i]] = [0, 2, 1][self.co[i]];
            new.cp[self.cp[i]] = i;
        }

        for i in 0..12 {
            new.eo[self.ep[i]] = self.eo[i];
            new.ep[self.ep[i]] = i;
        }

        new
    }
}

impl Add for Permutation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new = Permutation::ID;

        for i in 0..8 {
            new.co[i] = (self.co[other.cp[i]] + other.co[i]) % 3;
            new.cp[i] = self.cp[other.cp[i]];
        }

        for i in 0..12 {
            new.eo[i] = (self.eo[other.ep[i]] + other.eo[i]) % 2;
            new.ep[i] = self.ep[other.ep[i]];
        }

        new
    }
}

impl Mul<isize> for Permutation {
    type Output = Self;

    fn mul(self, num: isize) -> Self {
        let mut result = Self::ID;

        for _ in 0..num.abs() {
            result = result + self;
        }

        if num < 0 {
            -result
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Permutation;

    #[test]
    fn id_plus_id_is_id() {
        assert_eq!(Permutation::ID + Permutation::ID, Permutation::ID);
    }

    #[test]
    fn four_u_is_id() {
        assert_eq!(Permutation::U + Permutation::U + Permutation::U + Permutation::U, Permutation::ID);
    }

    #[test]
    fn u_plus_u_inv_is_id() {
        assert_eq!(Permutation::U + -Permutation::U, Permutation::ID);
    }

    #[test]
    fn three_u_is_u_inv() {
        assert_eq!(Permutation::U + Permutation::U + Permutation::U, -Permutation::U);
    }
}
