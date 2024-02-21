
pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            pos: 0,
            input,
        }
    }

    pub fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}
