use crate::dom;
use std::collections::HashMap;

pub struct Parser {
    pos: usize,
    input: String,
}

/// Parse an HTML document and return the root element.
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }.parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}

impl Parser {
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    /// Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    /// Parse a single node.
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    /// Parse a text node.
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_comment(&mut self) -> dom::Node {
        assert!(self.consume_char() == '!');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '-');
        let value = self.consume_while(|c| c != '-');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '>');
        dom::comment(value)
    }

    /// Parse a single element
    fn parse_element(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        match self.next_char() {
            '!' => return self.parse_comment(),
            _ => (),
        }
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        let children = self.parse_nodes();

        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        dom::elem(tag_name, attrs, children)
    }

    /// Parse a list of name="value" pairs, separated by whitespace.
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        value
    }

    /// Parse a sequence of sibling nodes.
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    /// Parse a tag or attribute name.
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' => true,
            _ => false
        })
    }

    fn consume_while<F>(&mut self, test: F) -> String
            where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

#[test]
fn test_simple_parse() {
    let html = "<html lang='ja' data-theme='light'>Title</html>".to_string();
    let parsed = parse(html);

    let mut attrs = HashMap::new();
    attrs.insert(
        "lang".to_string(),
        "ja".to_string()
    );
    attrs.insert(
        "data-theme".to_string(),
        "light".to_string()
    );
    let mut children = Vec::new();
    let text = dom::text("Title".to_string());
    children.push(text);
    let expected = dom::elem("html".to_string(), attrs, children);

    assert_eq!(expected, parsed);
}

#[test]
fn parse_comment() {
    let html = "<html lang='ja' data-theme='light'><!-- Title --></html>".to_string();
    let parsed = parse(html);

    let mut attrs = HashMap::new();
    attrs.insert(
        "lang".to_string(),
        "ja".to_string()
    );
    attrs.insert(
        "data-theme".to_string(),
        "light".to_string()
    );
    let mut children = Vec::new();
    let comment = dom::comment(" Title ".to_string());
    children.push(comment);
    let expected = dom::elem("html".to_string(), attrs, children);

    assert_eq!(expected, parsed);
}
