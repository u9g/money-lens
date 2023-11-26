use pest::iterators::{Pair, Pairs};

use crate::{
    types::{Expression, Line},
    Rule,
};

impl From<Pair<'_, Rule>> for Expression {
    fn from(value: Pair<'_, Rule>) -> Self {
        match value.as_rule() {
            Rule::number => {
                return Self::Literal(
                    value
                        .as_str()
                        .parse()
                        .expect("it must always parse based on the grammar"),
                )
            }
            _ => unreachable!(),
        }
    }
}

impl<'a> From<Pairs<'a, Rule>> for Line<'a> {
    fn from(value: Pairs<'a, Rule>) -> Self {
        let mut iter = value.into_iter().next().unwrap().into_inner();
        Self {
            name: iter.next().unwrap().as_str(),
            expression: iter.next().unwrap().into(),
        }
    }
}
