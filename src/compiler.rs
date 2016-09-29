use tokenizer;

#[derive(PartialEq)]
enum Operation {
    CHAR(char),
    MATCH,
    JMP(u32),
    SPLIT(u32)
}

use self::Operation::*;

struct Instruction {
    op: Operation,
}

impl Instruction {
    fn new(op: Operation) -> Instruction {
        return Instruction {op: op};
    }

    #[test]
    fn print(&self) {
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Instruction) -> bool {
        self.op == other.op
    }
}

/// takes a single token as argument. 
/// tokens are recursive, so the entire regex is a single token
fn compile(token: tokenizer::Token) -> Vec<Instruction> {
    let array = vec![];
    return array;
}

#[test]
fn run() {
    let token = tokenizer::tokenize("h(el)lo+", None);
    token.print();
    println!("");
    let array = compile(token);
    for i in array {
        i.print();
    }
}

