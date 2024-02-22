
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

    pub fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
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
