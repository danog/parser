use std::fmt::Display;

use crate::ByteString;

pub type Span = (usize, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum OpenTagKind {
    Full,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    From,
    Print,
    Dollar,
    HaltCompiler,
    Readonly,
    Global,
    Abstract,
    Ampersand,
    AmpersandEquals,
    And,
    AndEqual,
    Array,
    ArrayCast,
    Arrow,
    NullsafeArrow,
    At,
    As,
    Asterisk,
    Attribute,
    Bang,
    BangEquals,
    AngledLeftRight,
    BangDoubleEquals,
    Spaceship,
    BoolCast,
    BooleanCast,
    BooleanAnd,
    BooleanOr,
    Break,
    Callable,
    Caret,
    CaretEquals,
    Case,
    Catch,
    Class,
    ClassConstant,
    Clone,
    MinusEquals,
    CloseTag,
    Coalesce,
    CoalesceEqual,
    AsteriskEqual,
    Colon,
    Comma,
    Comment(ByteString),
    ConcatEqual,
    Const,
    ConstantString(ByteString),
    Continue,
    CurlyOpen,
    Declare,
    Decrement,
    Default,
    DirConstant,
    DivEqual,
    Do,
    DocComment(ByteString),
    DocOpen(ByteString),
    Dot,
    DotEquals,
    DoubleArrow,
    DoubleCast,
    RealCast,
    FloatCast,
    DoubleColon,
    DoubleEquals,
    Echo,
    Ellipsis,
    Else,
    ElseIf,
    Empty,
    EndDeclare,
    EndFor,
    EndForeach,
    EndIf,
    EndSwitch,
    EndWhile,
    Enum,
    Eof,
    Equals,
    Extends,
    False,
    Final,
    Finally,
    Float(f64),
    Fn,
    For,
    Foreach,
    FullyQualifiedIdentifier(ByteString),
    Function,
    Goto,
    GreaterThan,
    GreaterThanEquals,
    Identifier(ByteString),
    If,
    Implements,
    Include,
    IncludeOnce,
    Increment,
    InlineHtml(ByteString),
    Instanceof,
    Int(i64),
    IntCast,
    IntegerCast,
    Interface,
    LeftBrace,
    LeftBracket,
    LeftParen,
    LeftShift,
    LeftShiftEquals,
    RightShift,
    RightShiftEquals,
    LessThan,
    LessThanEquals,
    Match,
    Minus,
    Namespace,
    NamespaceSeparator,
    New,
    Null,
    ObjectCast,
    UnsetCast,
    OpenTag(OpenTagKind),
    Percent,
    PercentEquals,
    Pipe,
    PipeEquals,
    Plus,
    PlusEquals,
    Pow,
    PowEquals,
    Private,
    Protected,
    Public,
    QualifiedIdentifier(ByteString),
    Question,
    QuestionColon,
    Require,
    RequireOnce,
    Return,
    RightBrace,
    RightBracket,
    RightParen,
    SemiColon,
    Slash,
    SlashEquals,
    Static,
    StringCast,
    BinaryCast,
    Switch,
    Throw,
    Trait,
    TripleEquals,
    True,
    Try,
    Use,
    Var,
    Variable(ByteString),
    Yield,
    While,
    BitwiseNot,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::Eof,
            span: (0, 0),
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::From => "from",
            Self::Print => "print",
            Self::BitwiseNot => "~",
            Self::Dollar => "$",
            Self::HaltCompiler => "__halt_compiler",
            Self::Readonly => "readonly",
            Self::AsteriskEqual => "*=",
            Self::ObjectCast => "(object)",
            Self::UnsetCast => "(unset)",
            Self::Abstract => "abstract",
            Self::Ampersand => "&",
            Self::And => "&&",
            Self::AndEqual => "&=",
            Self::Arrow => "->",
            Self::NullsafeArrow => "?->",
            Self::Array => "array",
            Self::ArrayCast => "(array)",
            Self::As => "as",
            Self::Asterisk => "*",
            Self::Attribute => "#[",
            Self::Bang => "!",
            Self::BoolCast => "(bool)",
            Self::BooleanCast => "(boolean)",
            Self::BooleanAnd => "&&",
            Self::BooleanOr => "||",
            Self::Break => "break",
            Self::Callable => "callable",
            Self::Caret => "^",
            Self::CaretEquals => "^=",
            Self::Case => "case",
            Self::Catch => "catch",
            Self::Class => "class",
            Self::ClassConstant => "__CLASS__",
            Self::Clone => "clone",
            Self::CloseTag => "?>",
            Self::Coalesce => "??",
            Self::CoalesceEqual => "??=",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Comment(comment) => {
                return write!(f, "{}", String::from_utf8_lossy(comment));
            }
            Self::ConcatEqual => ".=",
            Self::Const => "const",
            Self::ConstantString(comment) => {
                return write!(f, "{}", String::from_utf8_lossy(comment));
            }
            Self::Continue => "continue",
            Self::IntCast => "(int)",
            Self::IntegerCast => "(integer)",
            Self::CurlyOpen => "{$",
            Self::Declare => "declare",
            Self::Decrement => "--",
            Self::Default => "default",
            Self::DirConstant => "__DIR__",
            Self::DivEqual => "/=",
            Self::Do => "do",
            Self::DocComment(comment) => {
                return write!(f, "{}", String::from_utf8_lossy(comment));
            }
            Self::DocOpen(doc_open) => {
                return write!(f, "{}", String::from_utf8_lossy(doc_open));
            }
            Self::Dot => ".",
            Self::DotEquals => ".=",
            Self::DoubleArrow => "=>",
            Self::DoubleCast => "(double)",
            Self::RealCast => "(real)",
            Self::FloatCast => "(float)",
            Self::DoubleColon => "::",
            Self::DoubleEquals => "==",
            Self::Echo => "echo",
            Self::Ellipsis => "...",
            Self::Else => "else",
            Self::ElseIf => "elseif",
            Self::Empty => "empty",
            Self::EndDeclare => "enddeclare",
            Self::EndFor => "endfor",
            Self::EndForeach => "endforeach",
            Self::EndIf => "endif",
            Self::EndSwitch => "endswitch",
            Self::EndWhile => "endwhile",
            Self::Enum => "enum",
            Self::Eof => "",
            Self::Equals => "=",
            Self::Extends => "extends",
            Self::False => "false",
            Self::Final => "final",
            Self::Finally => "finally",
            Self::Float(_) => "float",
            Self::Fn => "fn",
            Self::For => "for",
            Self::FullyQualifiedIdentifier(id) => {
                return write!(f, "{}", String::from_utf8_lossy(id));
            }
            Self::Function => "function",
            Self::Goto => "goto",
            Self::GreaterThan => ">",
            Self::GreaterThanEquals => ">=",
            Self::Identifier(id) => {
                return write!(f, "{}", String::from_utf8_lossy(id));
            }
            Self::If => "if",
            Self::Implements => "implements",
            Self::Increment => "++",
            Self::InlineHtml(_) => "InlineHtml",
            Self::Int(_) => "int",
            Self::LeftBrace => "{",
            Self::LeftBracket => "[",
            Self::LeftParen => "(",
            Self::LeftShift => "<<",
            Self::LeftShiftEquals => "<<=",
            Self::RightShift => ">>",
            Self::RightShiftEquals => ">>=",
            Self::LessThan => "<",
            Self::LessThanEquals => "<=",
            Self::Match => "match",
            Self::Minus => "-",
            Self::MinusEquals => "-=",
            Self::Namespace => "namespace",
            Self::NamespaceSeparator => "\\",
            Self::New => "new",
            Self::Null => "null",
            Self::OpenTag(kind) => match kind {
                OpenTagKind::Full => "<?php",
            },
            Self::Percent => "%",
            Self::PercentEquals => "%=",
            Self::Pipe => "|",
            Self::PipeEquals => "|=",
            Self::Plus => "+",
            Self::PlusEquals => "+=",
            Self::Pow => "**",
            Self::Private => "private",
            Self::Protected => "protected",
            Self::Public => "public",
            Self::QualifiedIdentifier(id) => {
                return write!(f, "{}", String::from_utf8_lossy(id));
            }
            Self::Question => "?",
            Self::QuestionColon => "?:",
            Self::Require => "require",
            Self::RequireOnce => "require_once",
            Self::Return => "return",
            Self::RightBrace => "}",
            Self::RightBracket => "]",
            Self::RightParen => ")",
            Self::SemiColon => ";",
            Self::Slash => "/",
            Self::SlashEquals => "/=",
            Self::Static => "static",
            Self::StringCast => "(string)",
            Self::BinaryCast => "(binary)",
            Self::Switch => "switch",
            Self::Throw => "throw",
            Self::Trait => "trait",
            Self::TripleEquals => "===",
            Self::True => "true",
            Self::Try => "try",
            Self::Use => "use",
            Self::Var => "var",
            Self::Variable(var) => {
                return write!(f, "{}", String::from_utf8_lossy(var));
            }
            Self::Yield => "yield",
            Self::While => "while",
            Self::Global => "global",
            Self::AngledLeftRight => "<>",
            Self::Spaceship => "<=>",
            Self::LogicalAnd => "and",
            Self::LogicalOr => "or",
            Self::LogicalXor => "xor",
            _ => todo!("format token: {:?}", self),
        };
        write!(f, "{}", s)
    }
}
