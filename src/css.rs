
#[derive(Debug, PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, PartialEq)]
pub struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, PartialEq)]
pub enum Unit {
    Px,
}

#[derive(Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const COLOR_NAME: &[&str] = &[
    "red",
];

pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser { pos: 0, input: source };
    Stylesheet { rules: parser.parse_rules() }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() { break }
            rules.push(self.parse_rule());
        }
        rules
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {c} in selector list")
            }
        }
        selectors.sort_by(|a,b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    // universal selector
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break
            }
        }
        selector
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume_char(), '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    fn parse_declaration(&mut self) -> Declaration {
        let property_name = self.parse_identifier();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ';');

        Declaration {
            name: property_name,
            value,
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => {
                let id = self.parse_identifier();
                if COLOR_NAME.contains(&id.as_str()) {
                    Value::ColorValue(Color {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 255
                    })
                } else {
                    Value::Keyword(id)
                }
            }
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });
        s.parse().unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit")
        }
    }

    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255
        })
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos .. self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_while<F>(&mut self, test: F) -> String
            where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
        _ => false,
    }
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

#[test]
fn parse_simple_css() {
    let parsed = parse("body { margin: 8px; }".to_string());
    let mut selectors = Vec::new();
    selectors.push(Selector::Simple(SimpleSelector {
        tag_name: Some("body".to_string()),
        id: None,
        class: Vec::new(),
    }));
    let mut declarations = Vec::new();
    declarations.push(Declaration {
        name: "margin".to_string(),
        value: Value::Length(8.0, Unit::Px),
    });
    let mut rules = Vec::new();
    rules.push(Rule {
        selectors,
        declarations,
    });
    let expected = Stylesheet { rules };
    assert_eq!(expected, parsed);
}

#[test]
fn parse_rgb_color() {
    // #FF0000 = red
    let parsed = parse("body { background: #FF0000; }".to_string());
    let mut selectors = Vec::new();
    selectors.push(Selector::Simple(SimpleSelector {
        tag_name: Some("body".to_string()),
        id: None,
        class: Vec::new(),
    }));
    let mut declarations = Vec::new();
    declarations.push(Declaration {
        name: "background".to_string(),
        value: Value::ColorValue(Color{
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }),
    });
    let mut rules = Vec::new();
    rules.push(Rule {
        selectors,
        declarations,
    });
    let expected = Stylesheet { rules };
    assert_eq!(expected, parsed);
}

#[test]
fn parse_color_name() {
    let parsed = parse("body { background: red; }".to_string());
    let mut selectors = Vec::new();
    selectors.push(Selector::Simple(SimpleSelector {
        tag_name: Some("body".to_string()),
        id: None,
        class: Vec::new(),
    }));
    let mut declarations = Vec::new();
    declarations.push(Declaration {
        name: "background".to_string(),
        value: Value::ColorValue(Color{
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }),
    });
    let mut rules = Vec::new();
    rules.push(Rule {
        selectors,
        declarations,
    });
    let expected = Stylesheet { rules };
    assert_eq!(expected, parsed);
}
