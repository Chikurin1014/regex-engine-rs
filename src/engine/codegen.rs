//! 抽象構文木を命令列に変換

use std::{error::Error, fmt::Display};

use super::{parser::Ast, Instruction};
use crate::helper::safe_add;

#[derive(Debug)]
pub enum CodeGenError {
    PcOverFlow,
    FailPlus,
    FailStar,
    FailQuestion,
    FailOr,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for CodeGenError {}

pub fn gen_code(ast: &Ast) -> Result<Vec<Instruction>, CodeGenError> {
    let mut generator = Generator::default();
    generator.gen_code(ast)?;
    Ok(generator.insts)
}

#[derive(Default, Debug)]
struct Generator {
    pc: usize,
    insts: Vec<Instruction>,
}

impl Generator {
    fn inc_pc(&mut self) -> Result<(), CodeGenError> {
        safe_add(&mut self.pc, &1, || CodeGenError::PcOverFlow)
    }

    fn gen_code(&mut self, ast: &Ast) -> Result<(), CodeGenError> {
        self.gen_inst(ast)?;
        self.inc_pc()?;
        self.insts.push(Instruction::Match);
        Ok(())
    }

    fn gen_inst(&mut self, ast: &Ast) -> Result<(), CodeGenError> {
        match ast {
            Ast::Char(c) => self.gen_char(*c)?,
            Ast::Plus(e) => self.gen_plus(e)?,
            Ast::Star(e) => self.gen_star(e)?,
            Ast::Question(e) => self.gen_question(e)?,
            Ast::Or(e1, e2) => self.gen_or(e1, e2)?,
            Ast::Seq(v) => self.gen_seq(v)?,
        }
        Ok(())
    }

    fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
        let inst = Instruction::Char(c);
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    fn gen_plus(&mut self, e: &Ast) -> Result<(), CodeGenError> {
        // L1: e
        //     split L1 L2
        // L2:

        const DUMMY_ADDR: usize = 0;

        // L1: e
        let l1 = self.pc;
        self.gen_inst(e)?;

        // split L1 L2 (L2 is dummy)
        let split_addr = self.pc; // L2への代入に使用
        self.insts.push(Instruction::Split(l1, DUMMY_ADDR));
        self.inc_pc()?;

        // L2 <- pc
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailPlus);
        }

        Ok(())
    }

    fn gen_star(&mut self, e: &Ast) -> Result<(), CodeGenError> {
        //     split L1 L2
        // L1: e
        //     jump L1
        // L2:

        const DUMMY_ADDR: usize = 0;

        // split L1 L2 (L2 is dummy)
        let split_addr = self.pc; // L2への代入に使用
        self.inc_pc()?;
        let l1 = self.pc;
        self.insts.push(Instruction::Split(l1, DUMMY_ADDR));

        // L1: e
        self.gen_inst(e)?;

        // jump L1
        self.insts.push(Instruction::Jump(l1));
        self.inc_pc()?;

        // L2 <- pc
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailStar);
        }

        Ok(())
    }

    fn gen_question(&mut self, e: &Ast) -> Result<(), CodeGenError> {
        //     split L1 L2
        // L1: e
        // L2:

        const DUMMY_ADDR: usize = 0;

        // split L1 L2 (L2 is dummy)
        let split_addr = self.pc; // L2への代入に使用
        self.inc_pc()?;
        let l1 = self.pc;
        self.insts.push(Instruction::Split(l1, DUMMY_ADDR));

        // L1: e
        self.gen_inst(e)?;

        // L2 <- pc
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailQuestion);
        }

        Ok(())
    }

    fn gen_or(&mut self, e1: &Ast, e2: &Ast) -> Result<(), CodeGenError> {
        //     split L1 L2
        // L1: e1
        //     jump L3
        // L2: e2
        // L3:

        const DUMMY_ADDR: usize = 0;

        // split L1 L2 (L2 is dummy)
        let split_addr = self.pc; // L2への代入に使用
        self.inc_pc()?;
        let l1 = self.pc;
        self.insts.push(Instruction::Split(l1, DUMMY_ADDR));

        // L1: e1
        self.gen_inst(e1)?;

        // jump L3 (L3 is dummy)
        let jump_addr = self.pc; // L3への代入に使用
        self.insts.push(Instruction::Jump(DUMMY_ADDR));
        self.inc_pc()?;

        // L2 <- pc
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        // L2: e2
        self.gen_inst(e2)?;

        // L3 <- pc
        if let Some(Instruction::Jump(l3)) = self.insts.get_mut(jump_addr) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        Ok(())
    }

    fn gen_seq(&mut self, v: &[Ast]) -> Result<(), CodeGenError> {
        for e in v {
            self.gen_inst(e)?;
        }
        Ok(())
    }
}
