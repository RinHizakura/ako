#[derive(Debug, Clone)]
pub enum OpType {
    OpPlus,
    OpMinus,
    OpMul,
    OpDiv,
    OpUnknown,
}

#[derive(Debug)]
pub struct InfixExpression {
    op: OpType,
    left: Option<Box<Expression>>,
    right: Option<Box<Expression>>,
}

#[derive(Debug)]
pub enum Expression {
    Int(usize),
    Infix(InfixExpression),
}

impl Expression {
    pub fn int(u: usize) -> Self {
        Expression::Int(u)
    }

    pub fn infix(op: OpType, left: Option<Expression>, right: Option<Expression>) -> Self {
        let left = if left.is_none() {
            None
        } else {
            Some(Box::new(left.unwrap()))
        };
        let right = if right.is_none() {
            None
        } else {
            Some(Box::new(right.unwrap()))
        };
        Expression::Infix(InfixExpression {
            op: op,
            left: left,
            right: right,
        })
    }
}

#[derive(Debug)]
pub struct Statement {
    expression: Expression,
}

impl Statement {
    pub fn new(expression: Expression) -> Self {
        Statement { expression }
    }
}
