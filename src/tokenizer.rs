pub enum TokenInternal {
    Literal(char),
    Complex(Vec<Token>),
}

pub enum Modifier {
    PLUS,
    STAR,
    QUES,
}

impl Modifier {
    fn print(&self) {
        match self {
            &Modifier::PLUS => print!("+"),
            &Modifier::STAR => print!("*"),
            &Modifier::QUES => print!("?"),
        }
    }
}

use self::TokenInternal::*;

pub struct Token {
    pub internal: TokenInternal,
    pub modifier: Option<Modifier>,
}

impl Token {
    fn new_literal(c: char, m: Option<Modifier>) -> Token {
        return Token {internal: Literal(c), modifier: m};
    }

    fn new_complex() -> Token {
        return Token {internal: Complex(vec![]), modifier: None};
    }

    fn push_token(&mut self, t: Token) {
        match self.internal {
            Literal(_) => assert!(false, "cannot push to literal token"),
            Complex(ref mut v) => v.push(t),
        }
    }

    fn set_modifier(&mut self, modifier: Option<Modifier>) {
        self.modifier = modifier; 
    }

    #[test]
    pub fn print(&self) {
        match self.internal {
            Literal(ref c) => print!("{}", c),
            Complex(ref v) => {
                for t in v {
                    match t.internal {
                        Literal(c) => { 
                            print!("{}",c);
                            match t.modifier {
                                Some(ref m) => m.print(),
                                _ => {},
                            }   
                        }
                        Complex(_) => {
                            print!("(");
                            t.print();
                            print!(")");
                            match t.modifier {
                                Some(ref m) => m.print(),
                                _ => {},
                            }   
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


fn is_modifier_char(o: &Option<&char> ) -> bool {
    match o {
        &Some(c) => {
                match c {
                &'+' | &'*' | &'?' => true,
                _ => false,
            }
        }
        &None => false,
    }
}

fn get_modifier(o: &Option<char>) -> Option<Modifier> {
    match o {
        &Some(c) => {
            match c {
                '?' => Some(Modifier::QUES),
                '+' => Some(Modifier::PLUS),
                '*' => Some(Modifier::STAR),
                _ => None,
            }
        }
        &None => None,
    }
}


// the 'terminator' is used for recursive calls to the function 
// i.e. determining how far the subregex goes. For a normal call,
// it should be None
pub fn tokenize(regex: &str, terminator: Option<char> ) -> Token {
    let mut root_token = Token::new_complex();    
    let mut iter = regex.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '(' => { 
                let mut paren_count: Box<u32> = Box::new(1);
                let sub_regex = (&mut iter).take_while(&mut *take_while_closure(paren_count)).collect::<String>();
                root_token.push_token( tokenize(sub_regex.as_str(), Some(')')) );
            }
            _ => { 
                let modifier: Option<Modifier>;
                if is_modifier_char(&iter.peek()) {
                    modifier = get_modifier(&iter.next());
                }
                else {
                    modifier = None;
                }
                match terminator {
                    Some(x) => {
                        if c==x {
                            break;
                        }
                        else {
                            root_token.push_token( Token::new_literal(c, modifier) );
                        }
                    }
                    None => root_token.push_token( Token::new_literal(c, modifier) ),
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
