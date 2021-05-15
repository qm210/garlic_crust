use super::*;

#[derive(Copy, Clone)]
pub struct Edge {
    array: BlockArray,
    function: Option<PlayFunc>, // hm. is it good to have fn(globaltime, playhead) instead of just fn(playhead) ?
    is_const: bool,
}

pub type PlayFunc = fn(TimeFloat) -> AmpFloat;

impl Edge {
    pub fn function(function: PlayFunc) -> Edge { // HAVE NO IDEA ABOUT THIS YET..!!
        Edge {
            array: EMPTY_BLOCKARRAY,
            function: Some(function),
            is_const: false,
        }
    }

    pub fn array(block: BlockArray) -> Edge {
        Edge {
            array: block,
            function: None,
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
            function: None,
            is_const: true,
        }
    }

    pub fn zero() -> Edge {
        Edge::array(EMPTY_BLOCKARRAY)
    }

    pub fn put_at(&mut self, pos: usize, value: AmpFloat) {
        self.function = None;
        self.array[pos] = value;
    }

    pub fn evaluate(&self, pos: usize) -> AmpFloat {
        if let Some(func) = self.function {
            // no idea whether this somehow works or rather will be garbage
            return func(pos as TimeFloat / SAMPLERATE);
        }
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
            function: None,
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
            function: None,
            is_const: self.is_const && multiply.is_const && add.is_const
        }
    }
}