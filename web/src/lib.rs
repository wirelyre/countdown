use std::{
    cmp::max,
    num::Wrapping,
    ops::{Add, Div, Mul, Sub},
};

use bitvec::prelude::*;
use bytemuck::{cast_slice, Pod, Zeroable};
use wasm_bindgen::prelude::wasm_bindgen;

fn compute(instructions: &[Instruction; 5], inputs: [u16; 6]) -> i32 {
    let mut tape = [Wrapping(0); 12];

    tape[1] = Wrapping(inputs[0] as i32);
    tape[2] = Wrapping(inputs[1] as i32);
    tape[3] = Wrapping(inputs[2] as i32);
    tape[4] = Wrapping(inputs[3] as i32);
    tape[5] = Wrapping(inputs[4] as i32);
    tape[6] = Wrapping(inputs[5] as i32);

    for (i, inst) in instructions.iter().enumerate() {
        let (lhs, rhs, op) = inst.unpack();
        let lhs = tape[lhs];
        let rhs = tape[rhs];

        if rhs.0 == 0 {
            return 0;
        }

        let result = match op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => {
                if lhs.0 % rhs.0 != 0 {
                    return 0;
                }
                lhs / rhs
            }
        };
        if result.0 <= 0 {
            return 0;
        }

        tape[i + 7] = result;
    }

    tape[11].0
}

fn show(computation: &[Instruction; 5], inputs: [u16; 6]) -> Ast {
    let mut tape = Vec::new();
    tape.push(Ast::Num(0));
    tape.extend(inputs.iter().map(|i| Ast::Num(*i)));

    for inst in computation {
        let (lhs, rhs, op) = inst.unpack();
        let lhs = tape[lhs].clone();
        let rhs = tape[rhs].clone();

        let result = match op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        };
        tape.push(result);
    }

    tape.pop().unwrap()
}

#[wasm_bindgen]
pub fn reachable(inputs: Vec<u16>) -> Vec<u16> {
    let inputs: [u16; 6] = inputs.try_into().unwrap();
    let instructions: &[[Instruction; 5]] = cast_slice(INSTRUCTIONS);
    let mut results: BitArr!(for 1000) = BitArray::ZERO;

    for computation in instructions {
        let result = compute(computation, inputs);

        if (result >= 100) && (result < 1000) {
            results.set(result as usize, true);
        }
    }

    results.iter_ones().map(|i| i as u16).collect()
}

#[wasm_bindgen]
pub fn computations(inputs: Vec<u16>, target: i32) -> Vec<js_sys::JsString> {
    if target <= 0 {
        return Vec::new();
    }

    let inputs: [u16; 6] = inputs.try_into().unwrap();
    let instructions: &[[Instruction; 5]] = cast_slice(INSTRUCTIONS);
    let mut results: Vec<Ast> = Vec::new();

    for computation in instructions {
        if compute(computation, inputs) == target {
            results.push(show(computation, inputs).canonical());
        }
    }

    results.sort_unstable();
    results.dedup();
    results.sort_by_cached_key(|r| (r.len(), r.depth()));
    results
        .drain(..)
        .map(|ast| format!("{}", ast).into())
        .collect()
}

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
struct Instruction {
    args: u8,
    op: u8,
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Instruction {
    pub fn unpack(self) -> (usize, usize, Op) {
        let op = match self.op {
            0 => Op::Add,
            1 => Op::Sub,
            2 => Op::Mul,
            3 => Op::Div,
            _ => unreachable!(),
        };
        ((self.args & 0xF) as usize, (self.args >> 4) as usize, op)
    }
}

static INSTRUCTIONS: &[u8] = include_bytes!("../computations.bin");

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Ast {
    Add(Vec<Ast>, Vec<Ast>),
    Mul(Vec<Ast>, Vec<Ast>),
    Num(u16),
}

impl Ast {
    pub fn canonical(self) -> Self {
        match self {
            Ast::Num(_) => self,

            Ast::Add(mut pos, mut neg) => {
                let mut pos: Vec<Ast> = pos.drain(..).map(Ast::canonical).collect();
                let mut neg: Vec<Ast> = neg.drain(..).map(Ast::canonical).collect();
                pos.sort_unstable();
                neg.sort_unstable();
                Ast::Add(pos, neg)
            }

            Ast::Mul(mut pos, mut neg) => {
                let mut pos: Vec<Ast> = pos.drain(..).map(Ast::canonical).collect();
                let mut neg: Vec<Ast> = neg.drain(..).map(Ast::canonical).collect();
                pos.sort_unstable();
                neg.sort_unstable();
                Ast::Mul(pos, neg)
            }
        }
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Ast::Num(_))
    }

    pub fn len(&self) -> u8 {
        match self {
            Ast::Num(_) => 1,
            Ast::Add(pos, neg) | Ast::Mul(pos, neg) => {
                pos.iter().map(Ast::len).sum::<u8>() + neg.iter().map(Ast::len).sum::<u8>()
            }
        }
    }

    pub fn depth(&self) -> u8 {
        match self {
            Ast::Num(_) => 1,
            Ast::Add(pos, neg) | Ast::Mul(pos, neg) => {
                max(
                    pos.iter().map(Ast::depth).max().unwrap_or(0),
                    neg.iter().map(Ast::depth).max().unwrap_or(0),
                ) + 1
            }
        }
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn recurse(ast: &Ast, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if ast.is_num() {
                write!(f, "{}", ast)
            } else {
                write!(f, "({})", ast)
            }
        }

        match self {
            Ast::Num(n) => write!(f, "{}", n)?,

            Ast::Add(pos, neg) => {
                let mut pos_iter = pos.iter();
                recurse(pos_iter.next().unwrap(), f)?;

                for p in pos_iter {
                    write!(f, " + ")?;
                    recurse(p, f)?;
                }
                for n in neg.iter() {
                    write!(f, " \u{2212} ")?; // actual minus sign
                    recurse(n, f)?;
                }
            }

            Ast::Mul(pos, neg) => {
                if neg.len() <= 1 {
                    let mut pos_iter = pos.iter();
                    recurse(pos_iter.next().unwrap(), f)?;

                    for p in pos_iter {
                        write!(f, " × ")?;
                        recurse(p, f)?;
                    }

                    for n in neg.iter() {
                        write!(f, " ÷ ")?;
                        recurse(n, f)?;
                    }
                } else {
                    write!(f, "(")?;

                    let mut pos_iter = pos.iter();
                    recurse(pos_iter.next().unwrap(), f)?;
                    for p in pos_iter {
                        write!(f, " × ")?;
                        recurse(p, f)?;
                    }

                    write!(f, ") ÷ (")?;

                    for n in neg.iter() {
                        write!(f, " × ")?;
                        recurse(n, f)?;
                    }

                    write!(f, ")")?;
                }
            }
        }
        Ok(())
    }
}

impl Add<Ast> for Ast {
    type Output = Ast;

    fn add(self, rhs: Ast) -> Self::Output {
        match (self, rhs) {
            (Ast::Num(0), rhs) => rhs,
            (Ast::Add(mut p1, mut n1), Ast::Add(p2, n2)) => {
                p1.extend(p2);
                n1.extend(n2);
                Ast::Add(p1, n1)
            }
            (Ast::Add(mut p1, n1), rhs) => {
                p1.push(rhs);
                Ast::Add(p1, n1)
            }
            (lhs, rhs) => Ast::Add(vec![lhs, rhs], Vec::new()),
        }
    }
}

impl Sub<Ast> for Ast {
    type Output = Ast;

    fn sub(self, rhs: Ast) -> Self::Output {
        match (self, rhs) {
            (Ast::Add(mut p1, mut n1), Ast::Add(p2, n2)) => {
                p1.extend(n2);
                n1.extend(p2);
                Ast::Add(p1, n1)
            }
            (Ast::Add(p1, mut n1), rhs) => {
                n1.push(rhs);
                Ast::Add(p1, n1)
            }
            (lhs, rhs) => Ast::Add(vec![lhs], vec![rhs]),
        }
    }
}

impl Mul<Ast> for Ast {
    type Output = Ast;

    fn mul(self, rhs: Ast) -> Self::Output {
        match (self, rhs) {
            (Ast::Num(1), other) | (other, Ast::Num(1)) => other,
            (Ast::Mul(mut p1, mut n1), Ast::Mul(p2, n2)) => {
                p1.extend(p2);
                n1.extend(n2);
                Ast::Mul(p1, n1)
            }
            (Ast::Mul(mut p1, n1), rhs) => {
                p1.push(rhs);
                Ast::Mul(p1, n1)
            }
            (lhs, rhs) => Ast::Mul(vec![lhs, rhs], Vec::new()),
        }
    }
}

impl Div<Ast> for Ast {
    type Output = Ast;

    fn div(self, rhs: Ast) -> Self::Output {
        match (self, rhs) {
            (lhs, Ast::Num(1)) => lhs,
            (Ast::Mul(mut p1, mut n1), Ast::Mul(p2, n2)) => {
                p1.extend(n2);
                n1.extend(p2);
                Ast::Mul(p1, n1)
            }
            (Ast::Mul(p1, mut n1), rhs) => {
                n1.push(rhs);
                Ast::Mul(p1, n1)
            }
            (lhs, rhs) => Ast::Mul(vec![lhs], vec![rhs]),
        }
    }
}
