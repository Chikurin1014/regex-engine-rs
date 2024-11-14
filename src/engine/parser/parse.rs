use std::mem::take;

use super::ParseError;

/// 抽象構文木
#[derive(Debug, PartialEq, Eq)]
pub enum Ast {
    Char(char),
    Plus(Box<Self>),
    Question(Box<Self>),
    Star(Box<Self>),
    Or(Box<Self>, Box<Self>),
    Seq(Vec<Self>),
}

fn parse_escape(pos: usize, c: char) -> Result<Ast, ParseError> {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '?' => Ok(Ast::Char(c)),
        _ => Err(ParseError::InvalidEscape(pos, c)),
    }
}

enum Psq {
    Plus,
    Star,
    Question,
}

fn parse_plus_star_question(
    seq: &mut Vec<Ast>,
    quantifier_type: Psq,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = seq.pop() {
        let ast = match quantifier_type {
            Psq::Plus => Ast::Plus(Box::new(prev)),
            Psq::Star => Ast::Star(Box::new(prev)),
            Psq::Question => Ast::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoOperand(pos))
    }
}

fn parse_seq(seq: &mut Vec<Ast>) -> Result<Ast, ParseError> {
    match seq.len() {
        0 => Err(ParseError::Empty),
        1 => Ok(seq.pop().unwrap()),
        _ => Ok(Ast::Seq(take(seq))),
    }
}

fn parse_or(
    seq: &mut Vec<Ast>,
    or_operand: &mut Option<Box<Ast>>,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = or_operand.take() {
        let next = parse_seq(seq).or(Err(ParseError::NoOperand(pos)))?;
        seq.push(make_or(*prev, next, pos)?);
    }
    let prev = parse_seq(seq).or(Err(ParseError::NoOperand(pos)))?;
    or_operand.replace(Box::new(prev));
    Ok(())
}

fn make_or(prev: Ast, next: Ast, pos: usize) -> Result<Ast, ParseError> {
    if let Ast::Seq(seq) = &prev {
        if seq.is_empty() {
            return Err(ParseError::NoOperand(pos));
        }
    }
    if let Ast::Seq(seq) = &next {
        if seq.is_empty() {
            return Err(ParseError::NoOperand(pos));
        }
    }
    Ok(Ast::Or(Box::new(prev), Box::new(next)))
}

enum ParseState {
    Char,
    Escape,
}

pub fn parse(expr: &str) -> Result<Ast, ParseError> {
    let mut seq = Vec::new();
    let mut or_operand = None::<Box<Ast>>;
    let mut stack = Vec::new();
    let mut state = ParseState::Char;
    for (i, c) in expr.chars().enumerate() {
        match &state {
            ParseState::Char => match c {
                '+' => parse_plus_star_question(&mut seq, Psq::Plus, i)?,
                '*' => parse_plus_star_question(&mut seq, Psq::Star, i)?,
                '?' => parse_plus_star_question(&mut seq, Psq::Question, i)?,
                '(' => {
                    let prev = take(&mut seq);
                    stack.push(prev);
                }
                ')' => {
                    if let Some(mut prev) = stack.pop() {
                        let next = take(&mut seq);
                        seq.append(&mut prev);
                        seq.push(Ast::Seq(next));
                    }
                }
                '|' => {
                    parse_or(&mut seq, &mut or_operand, i)?;
                }
                '\\' => state = ParseState::Escape,
                _ => seq.push(Ast::Char(c)),
            },
            ParseState::Escape => {
                let ast = parse_escape(i, c)?;
                seq.push(ast);
                state = ParseState::Char;
            }
        }
    }
    if !stack.is_empty() {
        return Err(ParseError::NoRightParen);
    }

    if let Some(prev) = or_operand.take() {
        if !seq.is_empty() {
            let mut next = take(&mut seq);
            match next.len() {
                0 => return Err(ParseError::NoOperand(expr.len())),
                1 => {
                    seq.push(make_or(*prev, next.pop().unwrap(), expr.len())?);
                }
                _ => {
                    seq.push(make_or(*prev, Ast::Seq(next), expr.len())?);
                }
            }
        }
    }

    match seq.len() {
        0 => Err(ParseError::Empty),
        1 => Ok(seq.pop().unwrap()),
        _ => Ok(Ast::Seq(seq)),
    }
}
