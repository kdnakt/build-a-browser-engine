use std::collections::HashMap;
use crate::css::{
    Rule,
    Selector,
    Selector::Simple,
    SimpleSelector,
    Specificity,
    Value,
};
use crate::dom::{
    ElementData,
    Node,
};

/// Map from CSS property names to values.
type PropertyMap = HashMap<String, Value>;

/// A node with associated style data.
struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

type MatchedRule<'a> = (Specificity, &'a Rule);

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
