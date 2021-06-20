use super::*;

#[derive(Copy, Clone)]
pub struct Edge {
    array: BlockArray,
    is_const: bool,
    is_mono: bool, // TODO: is_mono is actually not read anywhere yet.
}

impl Edge {
    pub fn array(block: BlockArray) -> Edge {
        Edge {
            array: block,
            is_const: false,
            is_mono: false,
        }
    }

    pub fn constant(value: f32) -> Edge {
        let mut array = EMPTY_BLOCKARRAY;
        for pos in 0 .. BLOCK_SIZE { //TODO: check -- do I need to loop here? not yet, right.
            array[pos] = [value, value];
        }
        Edge {
            array,
            is_const: true,
            is_mono: true,
        }
    }

    pub fn constant_stereo(sample: Sample) -> Edge {
        let mut array = EMPTY_BLOCKARRAY;
        for pos in 0 .. BLOCK_SIZE {
            array[pos] = sample;
        }
        Edge {
            array,
            is_const: true,
            is_mono: false,
        }
    }

    pub fn zero() -> Edge {
        Edge {
            array: EMPTY_BLOCKARRAY,
            is_const: true,
            is_mono: true,
        }
    }

    pub fn put_at(&mut self, pos: usize, value: Sample) {
        self.is_const = false;
        self.array[pos] = value;
    }

    pub fn evaluate(&self, pos: usize) -> Sample {
        if self.is_const {
            return self.array[0];
        }
        return self.array[pos];
    }

    pub fn put_at_mono(&mut self, pos: usize, ch: usize, value: MonoSample) {
        self.is_const = false;
        self.is_mono = false;
        self.array[pos][ch] = value;
    }

    pub fn evaluate_mono(&self, pos: usize, ch: usize) -> MonoSample {
        if self.is_const {
            return self.array[0][ch];
        }
        return self.array[pos][ch];
    }

    pub fn multiply(&mut self, other: &Edge) -> Edge {
        if other.is_const {
            let other_value = other.array[0];
            if self.is_const {
                self.array[0][0] *= other_value[0];
                self.array[0][1] *= other_value[1];
                return *self;
            }
            for pos in 0 .. BLOCK_SIZE {
                for ch in 0 .. 2 {
                    self.array[pos][ch] = self.array[pos][ch] * other_value[ch];
                }
            }
            self.is_const = false;
            return *self;
        }
        for pos in 0 .. BLOCK_SIZE {
            for ch in 0 .. 2 {
                self.array[pos][ch] *= other.array[pos][ch];
            }
        }
        return *self;
    }

    pub fn mad(&mut self, multiply: &Edge, add: &Edge) -> Edge {
        for pos in 0 .. BLOCK_SIZE {
            for ch in 0 .. 2 {
                self.array[pos][ch] = multiply.evaluate_mono(pos, ch) * self.evaluate_mono(pos, ch) + add.evaluate_mono(pos, ch);
            }
        }
        self.is_const = self.is_const && multiply.is_const && add.is_const;
        self.is_mono = self.is_mono && multiply.is_mono && add.is_mono; // TODO: think about whether this has some sense in it

        *self
    }

    pub fn clone_scaled(&self, factor: f32) -> Edge {
        let mut array = self.array.clone(); // probably expensive
        for pos in 0 .. BLOCK_SIZE {
            for ch in 0 .. 2 {
                array[pos][ch] = factor * array[pos][ch];
            }
        }
        Edge {
            array,
            is_const: self.is_const,
            is_mono: self.is_mono,
        }
    }

    pub fn write_to(&self, destination: &mut BlockArray) {
        for pos in 0 .. BLOCK_SIZE {
            destination[pos] = self.array[pos];
        }
    }
}

impl Default for Edge {
    fn default() -> Edge {
        Edge::zero()
    }
}