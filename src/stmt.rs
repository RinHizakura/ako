#[derive(Debug)]
pub enum OpType {
    OpAdd,
    OpSubtract,
    OpMul,
    OpDiv,
    OpUnknown,
}

#[derive(Debug)]
pub struct InfixExpression {
    pub op: OpType,
    pub left: Option<Box<Expression>>,
    pub right: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct AssignExpression {
    pub target: Option<Box<Expression>>,
    pub value: Option<Box<Expression>>,
}

#[derive(Debug)]
pub enum Expression {
    Int(i32),
    Ident(String),
    Infix(InfixExpression),
    Assign(AssignExpression),
}

impl Expression {
    pub fn int(i: i32) -> Self {
        Expression::Int(i)
    }

    pub fn ident(s: String) -> Self {
        Expression::Ident(s)
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

    pub fn assign(target: Option<Expression>, value: Option<Expression>) -> Self {
        let target = target.map(|o| Box::new(o));
        let value = value.map(|o| Box::new(o));
        Expression::Assign(AssignExpression {
            target: target,
            value: value,
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
