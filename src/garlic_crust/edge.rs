use super::*;

#[derive(Copy, Clone)]
pub struct Edge {
    array: BlockArray,
    is_const: bool,
}

pub type PlayFunc = fn(TimeFloat) -> AmpFloat;

impl Edge {
    pub fn array(block: BlockArray) -> Edge {
        Edge {
            array: block,
            is_const: false,
        }
    }

    pub fn constant(value: f32) -> Edge {
        let mut array = EMPTY_BLOCKARRAY;
        for pos in 0 .. BLOCK_SIZE {
            array[pos] = value;
        }
        Edge {
            array,
            is_const: true,
        }
    }

    pub fn zero() -> Edge {
        Edge {
            array: EMPTY_BLOCKARRAY,
            is_const: true,
        }
    }

    pub fn put_at(&mut self, pos: usize, value: AmpFloat) {
        self.is_const = false;
        self.array[pos] = value;
    }

    pub fn evaluate(&self, pos: usize) -> AmpFloat {
        if self.is_const {
            return self.array[0];
        }
        return self.array[pos % BLOCK_SIZE];
    }

    pub fn times(&mut self, other: &Edge) -> Edge {
        let mut array = EMPTY_BLOCKARRAY;
        for pos in 0 .. BLOCK_SIZE {
            array[pos] = other.evaluate(pos) * self.evaluate(pos);
        }
        Edge {
            array,
            is_const: self.is_const && other.is_const
        }
    }

    pub fn mad(&self, multiply: &Edge, add: &Edge) -> Edge {
        let mut array = EMPTY_BLOCKARRAY;
        for pos in 0 .. BLOCK_SIZE {
            array[pos] = multiply.evaluate(pos) * self.evaluate(pos) + add.evaluate(pos);
        }
        Edge {
            array,
            is_const: self.is_const && multiply.is_const && add.is_const
        }
    }
}
