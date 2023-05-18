use crate::cli::instructions::Instruction::*;
use crate::cli::instructions::InstructionAst::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Trigger,
    Step,
    Probe,
}

#[derive(Debug, Eq, PartialEq)]
pub enum InstructionAst {
    Instruction(Instruction),
    Sequence(Vec<InstructionAst>),
    Repeat(Box<InstructionAst>, usize),
}

pub fn parse(s: &str) -> Option<InstructionAst> {
    parse_sequence(s).and_then(|(ast, rest)| {
        if rest.trim().is_empty() {
            Some(ast)
        } else {
            None
        }
    })
}

//step 10, step 5
//Repeat(Step, 10) ", step 5"
fn parse_sequence(s: &str) -> Option<(InstructionAst, &str)> {
    let mut res = Vec::new();
    let (ast, mut rest) = parse_repeat(s)?;
    res.push(ast);

    while let Some(s) = parse_literal(rest, ",") {
        let (ast, s) = parse_repeat(s)?;
        rest = s;
        res.push(ast);
    }

    Some((Sequence(res), rest))
}

fn parse_repeat(s: &str) -> Option<(InstructionAst, &str)> {
    let (ast, s) = parse_instruction(s)?;
    Some(if let Some((n, s)) = parse_num(s) {
        (Repeat(Box::new(ast), n), s)
    } else {
        (ast, s)
    })
}

fn parse_instruction(s: &str) -> Option<(InstructionAst, &str)> {
    if let Some(s) = parse_literal(s, "trigger") {
        Some((Instruction(Trigger), s))
    } else if let Some(s) = parse_literal(s, "step") {
        Some((Instruction(Step), s))
    } else if let Some(s) = parse_literal(s, "probe") {
        Some((Instruction(Probe), s))
    } else if let Some(s) = parse_literal(s, "(") {
        let (ast, s) = parse_sequence(s)?;
        let s = parse_literal(s, ")")?;
        Some((ast, s))
    } else {
        None
    }
}

fn parse_num(s: &str) -> Option<(usize, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }
    match s.find(|d: char| !d.is_ascii_digit()) {
        None => Some((s.parse().unwrap(), "")),
        Some(t) => {
            if t == 0 {
                return None;
            }
            Some((s[..t].parse().unwrap(), &s[t..]))
        }
    }
}

fn parse_literal<'a>(s: &'a str, k: &str) -> Option<&'a str> {
    let s = s.trim_start();
    if s.starts_with(k) {
        Some(&s[k.len()..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::instructions::Instruction::*;
    use crate::cli::instructions::InstructionAst::*;
    use crate::cli::instructions::{parse, parse_literal};

    #[test]
    fn test_literal() {
        assert_eq!(parse_literal("abcdef", "abc"), Some("def"));
        assert_eq!(parse_literal("abdef", "abc"), None);
    }

    #[test]
    fn test_ast() {
        assert_eq!(
            parse("trigger, step 10, probe"),
            Some(Sequence(vec![
                Instruction(Trigger),
                Repeat(Box::new(Instruction(Step)), 10),
                Instruction(Probe)
            ]))
        );
    }
}
