use std::{
    error::Error,
    fmt::{self, Display},
};

/// パース時のエラー
#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char), // 誤ったエスケープシーケンス
    InvalidRightParen(usize),   // 開き括弧がないのに閉じ括弧がある
    NoOperand(usize),           // `+`, `|`, `*`, `?`のオペランドに式がない
    NoRightParen,               // 閉じ括弧がないのに開き括弧がある
    Empty,                      // 空の式
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEscape(pos, c) => write!(
                f,
                "ParseError: invalid escape sequence: pos = {pos}, char = `{c}`"
            ),
            Self::InvalidRightParen(pos) => write!(
                f,
                "ParseError: invalid right parenthesis: pos = {pos}",
                pos = pos
            ),
            Self::NoOperand(pos) => write!(
                f,
                "ParseError: no operand expression: pos = {pos}",
                pos = pos
            ),
            Self::NoRightParen => write!(f, "ParseError: no right parenthesis"),
            Self::Empty => write!(f, "ParseError: empty expression"),
        }
    }
}

impl Error for ParseError {}
