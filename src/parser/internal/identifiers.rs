use crate::lexer::token::TokenKind;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::error::ParseError;
use crate::parser::error::ParseResult;
use crate::parser::state::State;

use crate::peek_token;

pub fn ident_of(state: &mut State, kinds: &[&str]) -> ParseResult<SimpleIdentifier> {
    let ident = ident(state)?;

    let name = ident.name.to_string();

    if kinds.contains(&name.as_str()) {
        Ok(ident)
    } else {
        Err(ParseError::ExpectedIdentifier(
            kinds.iter().map(|s| s.to_string()).collect(),
            name,
            state.current.span,
        ))
    }
}

/// Expect an unqualified identifier such as Foo or Bar.
pub fn ident(state: &mut State) -> ParseResult<SimpleIdentifier> {
    if let TokenKind::Identifier(name) = state.current.kind.clone() {
        let span = state.current.span;

        state.next();

        Ok(SimpleIdentifier { span, name })
    } else {
        Err(ParseError::ExpectedToken(
            vec!["an identifier".to_owned()],
            Some(state.current.kind.to_string()),
            state.current.span,
        ))
    }
}

/// Expect an unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
pub fn name(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let name = peek_token!([
        TokenKind::Identifier(name) | TokenKind::QualifiedIdentifier(name) => {
            name.clone()
        },
    ], state, "an identifier");

    let span = state.current.span;
    state.next();

    Ok(SimpleIdentifier { span, name })
}

/// Expect an optional unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
pub fn optional_name(state: &mut State) -> Option<SimpleIdentifier> {
    let ident = match &state.current.kind {
        TokenKind::Identifier(name) | TokenKind::QualifiedIdentifier(name) => {
            Some(SimpleIdentifier {
                span: state.current.span,
                name: name.clone(),
            })
        }
        _ => None,
    };

    if ident.is_some() {
        state.next();
    }

    ident
}

/// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar.
pub fn full_name(state: &mut State) -> ParseResult<SimpleIdentifier> {
    let name = peek_token!([
            TokenKind::Identifier(name) | TokenKind::QualifiedIdentifier(name) | TokenKind::FullyQualifiedIdentifier(name) => {
                name.clone()
            },
        ], state, "an identifier");

    let span = state.current.span;
    state.next();

    Ok(SimpleIdentifier { span, name })
}

pub fn ident_maybe_reserved(state: &mut State) -> ParseResult<SimpleIdentifier> {
    match state.current.kind {
        _ if is_reserved_ident(&state.current.kind) => {
            let name = state.current.kind.to_string().into();

            let span = state.current.span;
            state.next();

            Ok(SimpleIdentifier { span, name })
        }
        _ => ident(state),
    }
}

pub fn ident_maybe_soft_reserved(state: &mut State) -> ParseResult<SimpleIdentifier> {
    match state.current.kind {
        _ if is_soft_reserved_ident(&state.current.kind) => {
            let name = state.current.kind.to_string().into();
            let span = state.current.span;
            state.next();

            Ok(SimpleIdentifier { span, name })
        }
        _ => ident(state),
    }
}

pub fn is_ident_maybe_soft_reserved(kind: &TokenKind) -> bool {
    if let TokenKind::Identifier(_) = kind {
        return true;
    }

    is_soft_reserved_ident(kind)
}

pub fn is_ident_maybe_reserved(kind: &TokenKind) -> bool {
    if let TokenKind::Identifier(_) = kind {
        return true;
    }

    is_reserved_ident(kind)
}

pub fn is_soft_reserved_ident(kind: &TokenKind) -> bool {
    matches!(kind, |TokenKind::Parent| TokenKind::Self_
        | TokenKind::True
        | TokenKind::False
        | TokenKind::Null
        | TokenKind::Enum
        | TokenKind::From
        | TokenKind::Readonly)
}

pub fn is_reserved_ident(kind: &TokenKind) -> bool {
    if is_soft_reserved_ident(kind) {
        return true;
    }

    matches!(
        kind,
        TokenKind::Static
            | TokenKind::Abstract
            | TokenKind::Final
            | TokenKind::For
            | TokenKind::Private
            | TokenKind::Protected
            | TokenKind::Public
            | TokenKind::Include
            | TokenKind::IncludeOnce
            | TokenKind::Eval
            | TokenKind::Require
            | TokenKind::RequireOnce
            | TokenKind::LogicalOr
            | TokenKind::LogicalXor
            | TokenKind::LogicalAnd
            | TokenKind::Instanceof
            | TokenKind::New
            | TokenKind::Clone
            | TokenKind::Exit
            | TokenKind::If
            | TokenKind::ElseIf
            | TokenKind::Else
            | TokenKind::EndIf
            | TokenKind::Echo
            | TokenKind::Do
            | TokenKind::While
            | TokenKind::EndWhile
            | TokenKind::EndFor
            | TokenKind::Foreach
            | TokenKind::EndForeach
            | TokenKind::Declare
            | TokenKind::EndDeclare
            | TokenKind::As
            | TokenKind::Try
            | TokenKind::Catch
            | TokenKind::Finally
            | TokenKind::Throw
            | TokenKind::Use
            | TokenKind::Insteadof
            | TokenKind::Global
            | TokenKind::Var
            | TokenKind::Unset
            | TokenKind::Isset
            | TokenKind::Empty
            | TokenKind::Continue
            | TokenKind::Goto
            | TokenKind::Function
            | TokenKind::Const
            | TokenKind::Return
            | TokenKind::Print
            | TokenKind::Yield
            | TokenKind::List
            | TokenKind::Switch
            | TokenKind::EndSwitch
            | TokenKind::Case
            | TokenKind::Default
            | TokenKind::Break
            | TokenKind::Array
            | TokenKind::Callable
            | TokenKind::Extends
            | TokenKind::Implements
            | TokenKind::Namespace
            | TokenKind::Trait
            | TokenKind::Interface
            | TokenKind::Class
            | TokenKind::ClassConstant
            | TokenKind::TraitConstant
            | TokenKind::FunctionConstant
            | TokenKind::MethodConstant
            | TokenKind::LineConstant
            | TokenKind::FileConstant
            | TokenKind::DirConstant
            | TokenKind::NamespaceConstant
            | TokenKind::HaltCompiler
            | TokenKind::Fn
            | TokenKind::Match
    )
}
