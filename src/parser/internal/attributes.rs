use crate::lexer::token::TokenKind;
use crate::parser::ast::attributes::Attribute;
use crate::parser::ast::attributes::AttributeGroup;
use crate::parser::error::ParseResult;
use crate::parser::expressions;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn gather_attributes(state: &mut State) -> ParseResult<bool> {
    state.gather_comments();

    if state.current.kind != TokenKind::Attribute {
        return Ok(false);
    }

    let start = state.current.span;
    let mut members = vec![];

    state.next();

    while state.current.kind != TokenKind::RightBracket {
        let start = state.current.span;
        let expression = expressions::lowest_precedence(state)?;
        let end = state.current.span;

        members.push(Attribute {
            start,
            expression,
            end,
        });

        if state.current.kind == TokenKind::Comma {
            state.next();
        } else {
            break;
        }
    }

    let end = utils::skip_right_bracket(state)?;

    state.attribute(AttributeGroup {
        start,
        members,
        end,
    });

    // recursive, looking for multiple attribute brackets after each other.
    gather_attributes(state).map(|_| true)
}
