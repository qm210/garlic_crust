use super::*;

#[derive(Copy, Clone)]
pub struct Edge {
    array: BlockArray,
    is_const: bool,
}

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
        return self.array[pos];
    }

    pub fn multiply(&mut self, other: &Edge) -> Edge {
        if other.is_const {
            let other_value = other.array[0];
            if self.is_const {
                self.array[0] *= other_value;
                return *self;
            }
            for pos in 0 .. BLOCK_SIZE {
                self.array[pos] = self.array[pos] * other_value;
            }
            self.is_const = false;
            return *self;
        }
        for pos in 0 .. BLOCK_SIZE {
            self.array[pos] *= other.array[pos];
        }
        return *self;
    }

    pub fn mad(&self, multiply: &Edge, add: &Edge) -> Edge {
        let mut array = EMPTY_BLOCKARRAY; // probably expensive
        for pos in 0 .. BLOCK_SIZE {
            array[pos] = multiply.evaluate(pos) * self.evaluate(pos) + add.evaluate(pos);
        }
        Edge {
            array,
            is_const: self.is_const && multiply.is_const && add.is_const
        }
    }

    pub fn clone_scaled(&self, factor: AmpFloat) -> Edge {
        let mut array = self.array.clone(); // probably expensive
        for pos in 0 .. BLOCK_SIZE {
            array[pos] = factor * array[pos];
        }
        Edge {
            array,
            is_const: self.is_const,
        }
    }
}

impl Default for Edge {
    fn default() -> Edge {
        Edge::zero()
    }
}