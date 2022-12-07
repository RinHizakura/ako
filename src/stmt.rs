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
    pub fn int(i: usize) -> Self {
        Expression::Int(i)
    }

    pub fn infix(op: OpType, left: Option<Expression>, right: Option<Expression>) -> Self {
        let left = left.map(|o| Box::new(o));
        let right = right.map(|o| Box::new(o));
        Expression::Infix(InfixExpression {
            op: op,
            left: left,
            right: right,
        })
    }
}

#[derive(Debug)]
pub struct Statement {
    pub expr: Expression,
}

impl Statement {
    pub fn new(expr: Expression) -> Self {
        Statement { expr }
    }
}
