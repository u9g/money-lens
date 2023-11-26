use pest::Parser;
use pest_derive::Parser;
use types::Line;

#[derive(Parser)]
#[grammar = "format.pest"]
pub struct MoneyLineParser;

mod convert;
mod types;

fn parse_line<'a>(input: &'a str) -> Option<Line<'a>> {
    let cst = MoneyLineParser::parse(Rule::line, input).ok()?;

    Some(cst.into())
}

#[cfg(test)]
mod test {
    use crate::{
        parse_line,
        types::{Expression, Line},
    };

    #[test]
    fn test_simple_lines() {
        assert_eq!(
            parse_line("jason 5").unwrap(),
            Line {
                name: "jason",
                expression: Expression::Literal(5)
            }
        )
    }
}
