//! 命令列と文字列を受け取り、評価

use std::{
    error::Error,
    fmt::{self, Display},
};

use super::Instruction;
use crate::helper::safe_add;

#[derive(Debug)]
pub enum EvalError {
    PcOverFlow,
    SpOverFlow,
    InvalidPc,
    InvalidContext,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EvalError: {:?}", self)
    }
}

impl Error for EvalError {}

pub fn eval(inst: &[Instruction], line: &[char], is_depth: bool) -> Result<bool, EvalError> {
    if is_depth {
        eval_depth(inst, line, 0, 0)
    } else {
        eval_width(inst, line)
    }
}

fn eval_depth(
    inst: &[Instruction],
    line: &[char],
    mut pc: usize,
    sp: usize,
) -> Result<bool, EvalError> {
    loop {
        let next = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPc);
        };
        match next {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c {
                        safe_add(&mut pc, &1, || EvalError::PcOverFlow)?;
                    }
                }
            }
            Instruction::Jump(addr) => {
                pc = *addr;
            }
            Instruction::Split(addr1, addr2) => {
                if eval_depth(inst, line, *addr1, sp)? || eval_depth(inst, line, *addr2, sp)? {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            Instruction::Match => {
                return Ok(true);
            }
        }
    }
}

fn eval_width(_inst: &[Instruction], _line: &[char]) -> Result<bool, EvalError> {
    todo!()
}
