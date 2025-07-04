#[derive(Debug)]
pub enum TokenGeneratorState {
    Comment,
    LineNumber,
    Other,
}

pub struct TokenGenerator<'a> {
    bytes: &'a [u8],
    len: usize,
    iter: usize,
    output: Vec<u8>,
    state: TokenGeneratorState,
}

impl<'a> TokenGenerator<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            state: TokenGeneratorState::Other,
            bytes,
            len: bytes.len(),
            iter: 0,
            output: Vec::new(),
        }
    }

    pub const fn state(&self) -> &TokenGeneratorState {
        &self.state
    }

    pub const fn set_state(&mut self, value: TokenGeneratorState) {
        self.state = value;
    }

    pub fn drain_output(self) -> Vec<u8> {
        self.output
    }

    pub fn peek(&mut self) -> Option<u8> {
        if self.iter < self.len {
            let index = self.iter;
            Some(self.bytes[index])
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<u8> {
        if self.iter < self.len {
            let index = self.iter;
            self.iter += 1;
            Some(self.bytes[index])
        } else {
            None
        }
    }

    pub fn next_assert(&mut self) -> u8 {
        self.next().unwrap()
    }

    pub fn push(&mut self, value: u8) {
        self.output.push(value);
    }

    pub fn push_next_assert(&mut self) {
        let value = self.next_assert();
        self.push(value);
    }
}
