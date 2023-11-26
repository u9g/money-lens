use std::collections::HashMap;

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use thiserror::Error;
use types::{Expression, Line};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue, UnwrapThrowExt};

#[derive(Parser)]
#[grammar = "format.pest"]
pub struct MoneyLineParser;

mod convert;
mod types;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("parse failed")]
    ParseFailed(#[from] pest::error::Error<Rule>),
}

fn parse_line<'a>(input: &'a str) -> Result<Line<'a>, EvalError> {
    MoneyLineParser::parse(Rule::line, input)
        .map_err(|e| EvalError::ParseFailed(e))
        .map(Into::into)
}

fn evaluate_expression(expr: &Expression, previous_number: i32) -> i32 {
    match expr {
        Expression::Literal(lit) => previous_number + *lit,
    }
}

fn evaluate(lines: Vec<Line<'_>>) -> HashMap<String, i32> {
    let mut state: HashMap<String, i32> = Default::default();

    for line in lines {
        state.insert(
            line.name.to_string(),
            evaluate_expression(
                &line.expression,
                state.get(line.name).map_or_else(|| 0, |x| *x),
            ),
        );
    }

    state
}

#[wasm_bindgen]
pub fn parse_lines(name: &str) -> JsValue {
    let mut parts = vec![];
    for part in name.split("\n").filter(|x| x != &"") {
        if let Ok(parsed_line) = parse_line(part) {
            parts.push(parsed_line);
        } else {
            return JsValue::NULL;
        }
    }

    serde_wasm_bindgen::to_value(&evaluate(parts)).unwrap()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        evaluate, parse_line,
        types::{Expression, Line},
    };

    #[test]
    fn test_parse_simple_lines() {
        assert_eq!(
            parse_line("jason 5").unwrap(),
            Line {
                name: "jason",
                expression: Expression::Literal(5)
            }
        )
    }

    #[test]
    fn test_eval_simple_lines() {
        assert_eq!(
            evaluate(vec![Line {
                name: "jason",
                expression: Expression::Literal(5)
            }]),
            HashMap::from_iter(vec![("jason".into(), 5)])
        );
    }
}
