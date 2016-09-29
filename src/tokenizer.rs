pub enum TokenInternal {
    Literal(char),
    Complex(Vec<Token>),
}

use self::TokenInternal::*;

pub struct Token {
    pub internal: TokenInternal,
}

impl Token {
    fn new_literal(c: char) -> Token {
        return Token {internal: Literal(c)};
    }

    fn new_complex() -> Token {
        return Token {internal: Complex(vec![])};
    }

    fn push_token(&mut self, t: Token) {
        match self.internal {
            Literal(_) => assert!(false, "cannot push to literal token"),
            Complex(ref mut v) => v.push(t),
        }
    }

    #[test]
    pub fn print(&self) {
        match self.internal {
            Literal(ref c) => print!("{}", c),
            Complex(ref v) => {
                for t in v {
                    match t.internal {
                        Literal(c) => print!("{}",c),
                        Complex(_) => {
                            print!("(");
                            t.print();
                            print!(")");
                        }
                    }
                }
            }
        }
    }
}

// /// /////////////////////////////////////////////////////////////////////////////
// the closure that makes sure take_while in tokenize grabs enough of the iterator /
// /// /////////////////////////////////////////////////////////////////////////////
fn take_while_closure(mut paren_count: Box<u32>) -> Box<FnMut(&char) -> bool> { 
    Box::new(
        move |c| {
            match *c {
                '(' => *paren_count += 1,
                ')' => *paren_count -= 1,
                _ => {},
            }
            let ret: bool;
            if *paren_count == 0 {
                false
            }
            else {
                true
            }
        }
    )
}

// the 'terminator' is used for recursive calls to the function 
// i.e. determining how far the subregex goes. For a normal call,
// it should be None
pub fn tokenize(regex: &str, terminator: Option<char> ) -> Token {
    let mut root_token = Token::new_complex();    
    let mut iter = regex.chars();

    while let Some(c) = iter.next() {
        match c {
            '(' => { 
                let mut paren_count: Box<u32> = Box::new(1);
                let sub_regex = (&mut iter).take_while(&mut *take_while_closure(paren_count)).collect::<String>();
                root_token.push_token( tokenize(sub_regex.as_str(), Some(')')) );
            }
            _ => { 
                match terminator {
                    Some(x) => {
                        if c==x {
                            break;
                        }
                        else {
                            root_token.push_token( Token::new_literal(c) );
                        }
                    }
                    None => root_token.push_token( Token::new_literal(c) ),
                }
            }
        }
    }
    return root_token;
}

#[ignore]
#[test]
fn test() {
    let token = tokenize("(hello)+.,h+x", None);
    token.print();
    println!("");
}
