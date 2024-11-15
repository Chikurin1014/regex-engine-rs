use std::fmt::Display;

use crate::helper::DynError;

pub mod codegen;
pub mod evaluator;
pub mod parser;

#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2) => write!(f, "split {:>04} {:>04}", addr1, addr2),
        }
    }
}

/// 正規表現と文字列をマッチングする
///
/// # 利用例
///
/// ```
/// use regex_engine;
/// regex_engine::do_matching(
///     expr: "abc|(de|cd)+",
///     line: "decddede",
///     is_depth: true
/// );
/// ```
///
/// # 引数
///
/// - expr: &str - 正規表現
/// - line: &str - マッチング対象の文字列
/// - is_depth: bool - 深さ優先探索を行うかどうか
///
/// # 戻り値
///
/// エラーなく実行でき、かつマッチングに**成功**した場合はOk(true)を返し、
/// エラーなく実行できたがマッチングに**失敗**した場合はOk(false)を返す。
///
/// 入力された正規表現や内部的な実装エラーがあった場合はErrを返す。
///
pub fn do_matching(expr: &str, line: &str, is_depth: bool) -> Result<bool, DynError> {
    let ast = parser::parse(expr)?;
    let code = codegen::gen_code(&ast)?;
    let line = line.chars().collect::<Vec<_>>();
    Ok(evaluator::eval(&code, &line, is_depth)?)
}
