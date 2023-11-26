#[derive(PartialEq, Debug)]
pub struct Line<'a> {
    pub name: &'a str,
    pub expression: Expression,
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Literal(i32),
}
