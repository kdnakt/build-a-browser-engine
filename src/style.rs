use std::collections::HashMap;
use crate::css::{
    Rule,
    Selector,
    Selector::Simple,
    SimpleSelector,
    Specificity,
    Stylesheet,
    Value,
};
use crate::dom::{
    ElementData,
    Node,
    NodeType::{
        Element,
        Text,
        Comment,
    },
};

/// Map from CSS property names to values.
pub type PropertyMap = HashMap<String, Value>;

/// A node with associated style data.
pub struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyledNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name).unwrap_or_else(|| self.value(fallback_name)
                        .unwrap_or_else(|| default.clone()))
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline
            },
            _ => Display::Inline
        }
    }
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors.iter()
        .find(|s| matches(elem, *s))
        .map(|s| (s.specificity(), rule))
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    let elem_classes = elem.classes();
    if selector.class.iter().any(|class| !elem_classes.contains(&**class)) {
        return false;
    }

    return true;
}

fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    values
}

/// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            Element(ref elem) => specified_values(elem, stylesheet),
            Text(_) => HashMap::new(),
            Comment(_) => HashMap::new(),
        },
        children: root.children.iter().map(|c| style_tree(c, stylesheet)).collect(),
    }
}

#[test]
fn display_block() {
    let root = crate::parser::parse("<div>Hello World!</div>".to_string());
    let stylesheet = crate::css::parse("div { display: block; }".to_string());
    let styled_node = style_tree(&root, &stylesheet);
    assert_eq!(Display::Block, styled_node.display());
}

#[test]
fn display_inline() {
    let root = crate::parser::parse("<div>Hello World!</div>".to_string());
    let stylesheet = crate::css::parse("div { color: red; }".to_string());
    let styled_node = style_tree(&root, &stylesheet);
    assert_eq!(Display::Inline, styled_node.display());
}

#[test]
fn display_none() {
    let root = crate::parser::parse("<div>Hello World!</div>".to_string());
    let stylesheet = crate::css::parse("div { display: none; }".to_string());
    let styled_node = style_tree(&root, &stylesheet);
    assert_eq!(Display::None, styled_node.display());
}
