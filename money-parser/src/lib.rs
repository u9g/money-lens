use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;
use types::{Expression, Line};

#[derive(Parser)]
#[grammar = "format.pest"]
pub struct MoneyLineParser;

mod convert;
mod types;

fn parse_line<'a>(input: &'a str) -> Option<Line<'a>> {
    let cst = MoneyLineParser::parse(Rule::line, input).ok()?;

    Some(cst.into())
}

fn evaluate_expression(expr: &Expression, previous_number: i32) -> i32 {
    match expr {
        Expression::Literal(lit) => *lit,
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
    fn test_evaluate_simple_lines() {
        assert_eq!(
            evaluate(vec![Line {
                name: "jason",
                expression: Expression::Literal(5)
            }]),
            HashMap::from_iter(vec![("jason".into(), 5)])
        )
    }
}
