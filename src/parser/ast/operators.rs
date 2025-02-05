use crate::lexer::token::Span;

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOperation {
    Addition {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Subtraction {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Multiplication {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Division {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Modulo {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Exponentiation {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Negation {
        span: Span,
        right: Box<Expression>,
    },
    Identity {
        span: Span,
        right: Box<Expression>,
    },
    PreIncrement {
        span: Span,
        right: Box<Expression>,
    },
    PostIncrement {
        left: Box<Expression>,
        span: Span,
    },
    PreDecrement {
        span: Span,
        right: Box<Expression>,
    },
    PostDecrement {
        left: Box<Expression>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperation {
    Assign {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Addition {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Subtraction {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Multiplication {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Division {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Modulo {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Exponentiation {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Concat {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    BitwiseAnd {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    BitwiseOr {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    BitwiseXor {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    LeftShift {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    RightShift {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Coalesce {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BitwiseOperation {
    And {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Or {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Xor {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    LeftShift {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    RightShift {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Not {
        span: Span,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOperation {
    Equal {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Identical {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    NotEqual {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    AngledNotEqual {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    NotIdentical {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    LessThan {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    GreaterThan {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    LessThanOrEqual {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    GreaterThanOrEqual {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Spaceship {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOperation {
    And {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Or {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    Not {
        span: Span,
        right: Box<Expression>,
    },
    LogicalAnd {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    LogicalOr {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
    LogicalXor {
        left: Box<Expression>,
        span: Span,
        right: Box<Expression>,
    },
}
