
#![allow(warnings, unused, non_snake_case, non_camel_case_types)]
use regex::Regex;
use lazy_static::lazy_static;
use graph_viz::GraphVizNode;

    fn fact(n: i64) -> i64 {
        if n < 0 {
            panic!("Factorial less than zero");
        }
        let mut s: i64 = 1;
        for i in 1..n + 1 {
            s *= i;
        }
        s
    }

    fn comb(n: i64, k: i64) -> i64 {
        let n_fact = fact(n);
        let k_fact = fact(k);
        let n_k_fact = fact(n - k);
        n_fact / (k_fact * n_k_fact)
    }


#[derive(Debug)]
pub enum Token {
Num(String),
Mul(String),
Plus(String),
Minus(String),
Div(String),
Comb(String),
Lb(String),
Rb(String),
Eof,
}

lazy_static!(
static ref Num_regex: Regex = Regex::new("(-?)[1-9]([0-9]*)").unwrap(); 
static ref Mul_regex: Regex = Regex::new("\\*").unwrap(); 
static ref Plus_regex: Regex = Regex::new("\\+").unwrap(); 
static ref Minus_regex: Regex = Regex::new("\\-").unwrap(); 
static ref Div_regex: Regex = Regex::new("/").unwrap(); 
static ref Comb_regex: Regex = Regex::new("\\$").unwrap(); 
static ref Lb_regex: Regex = Regex::new("\\(").unwrap(); 
static ref Rb_regex: Regex = Regex::new("\\)").unwrap(); 
);

lazy_static!(
static ref SKIP_REGEX: Vec<Regex> = vec!(Regex::new("\n").unwrap(),Regex::new("\r").unwrap(),Regex::new(" ").unwrap(),);
);

pub struct Tokenizer {
     input: String,
     pointer: usize,
}
#[derive(Debug)]
pub struct ParseError {
    pub position: usize,
    pub message: String,
}
impl Tokenizer {
fn match_token(&mut self) -> Option<Token> {
if let Some(m) = Num_regex.find_at(&self.input, self.pointer) {
if m.start() == self.pointer {
self.pointer = m.end();
return Some(Token::Num(self.input[m.start() .. m.end()].to_string()))}}
if let Some(m) = Mul_regex.find_at(&self.input, self.pointer) {
if m.start() == self.pointer {
self.pointer = m.end();
return Some(Token::Mul(self.input[m.start() .. m.end()].to_string()))}}
if let Some(m) = Plus_regex.find_at(&self.input, self.pointer) {
if m.start() == self.pointer {
self.pointer = m.end();
return Some(Token::Plus(self.input[m.start() .. m.end()].to_string()))}}
if let Some(m) = Minus_regex.find_at(&self.input, self.pointer) {
if m.start() == self.pointer {
self.pointer = m.end();
return Some(Token::Minus(self.input[m.start() .. m.end()].to_string()))}}
if let Some(m) = Div_regex.find_at(&self.input, self.pointer) {
if m.start() == self.pointer {
self.pointer = m.end();
return Some(Token::Div(self.input[m.start() .. m.end()].to_string()))}}
if let Some(m) = Comb_regex.find_at(&self.input, self.pointer) {
if m.start() == self.pointer {
self.pointer = m.end();
return Some(Token::Comb(self.input[m.start() .. m.end()].to_string()))}}
if let Some(m) = Lb_regex.find_at(&self.input, self.pointer) {
if m.start() == self.pointer {
self.pointer = m.end();
return Some(Token::Lb(self.input[m.start() .. m.end()].to_string()))}}
if let Some(m) = Rb_regex.find_at(&self.input, self.pointer) {
if m.start() == self.pointer {
self.pointer = m.end();
return Some(Token::Rb(self.input[m.start() .. m.end()].to_string()))}}
None
}
}


impl Tokenizer {
        fn new(input: String) -> Tokenizer {
            Tokenizer{
                input,
                pointer: 0,
            }
        }

        pub fn tokenize(input: String) -> Result<Vec<Token>, ParseError> {
            Tokenizer::new(input).run_tokenizing()
        }

        fn skip(&mut self) -> bool {
            for regex in SKIP_REGEX.iter() {
                if let Some(m) = regex.find_at(&self.input, self.pointer) {
                     let s = m.start();
                     if s == self.pointer {
                         self.pointer = m.end();
                         return true;
                     }
                }
            }
            false
        }

        fn run_tokenizing(&mut self) -> Result<Vec<Token>, ParseError> {
            let mut tokens: Vec<Token> = Vec::new();
            while self.pointer < self.input.len() {
                if self.skip() {
                    continue;
                }
                if let Some(token) = self.match_token() {
                   tokens.push(token);
                   continue;
                }
                return Err(
                    ParseError {
                        position: self.pointer,
                        message: "Expected token".to_string(),
                    }
                )
            }
            tokens.push(Token::Eof);
            Ok(tokens)
        }
}



struct Parser {
    tokens: Vec<Token>,
    pointer: usize,
    counter: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser{tokens, pointer: 0, counter: 0}
    }
}

impl Parser {

    fn Plus(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Plus(s) => Ok((GraphVizNode::new_leaf(id, "Plus".to_string()), s.clone())),
            _ => Err(ParseError{position: pos, message: "Expected Plus".to_string()}),
        }
    }
   
    fn Comb(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Comb(s) => Ok((GraphVizNode::new_leaf(id, "Comb".to_string()), s.clone())),
            _ => Err(ParseError{position: pos, message: "Expected Comb".to_string()}),
        }
    }
   
    fn Num(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Num(s) => Ok((GraphVizNode::new_leaf(id, "Num".to_string()), s.clone())),
            _ => Err(ParseError{position: pos, message: "Expected Num".to_string()}),
        }
    }
   
    fn Minus(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Minus(s) => Ok((GraphVizNode::new_leaf(id, "Minus".to_string()), s.clone())),
            _ => Err(ParseError{position: pos, message: "Expected Minus".to_string()}),
        }
    }
   
    fn Lb(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Lb(s) => Ok((GraphVizNode::new_leaf(id, "Lb".to_string()), s.clone())),
            _ => Err(ParseError{position: pos, message: "Expected Lb".to_string()}),
        }
    }
   
    fn Div(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Div(s) => Ok((GraphVizNode::new_leaf(id, "Div".to_string()), s.clone())),
            _ => Err(ParseError{position: pos, message: "Expected Div".to_string()}),
        }
    }
   
    fn Rb(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Rb(s) => Ok((GraphVizNode::new_leaf(id, "Rb".to_string()), s.clone())),
            _ => Err(ParseError{position: pos, message: "Expected Rb".to_string()}),
        }
    }
   
    fn Mul(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Mul(s) => Ok((GraphVizNode::new_leaf(id, "Mul".to_string()), s.clone())),
            _ => Err(ParseError{position: pos, message: "Expected Mul".to_string()}),
        }
    }
   
    fn Eof(&mut self) -> Result<(String, GraphVizNode), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Eof => Ok(("Eof".to_string(), GraphVizNode::new_leaf(id, "Eof".to_string()))),
            _          => Err(ParseError{position: pos, message: "Expected eof".to_string()})
        }
    }
fn S(&mut self,) -> Result<(GraphVizNode, (i64)),ParseError> {
let mut res:i64;
let id = self.counter.to_string();
let mut children: Vec<GraphVizNode> = Vec::new();
self.counter += 1;
let token = &self.tokens[self.pointer];
match token {
Token::Minus(_)|Token::Num(_)|Token::Lb(_)=> {
let (child, (C0_res)) = self.C()?;
children.push(child);
res = C0_res;
}
_ => return Err(ParseError{position: self.pointer, message: "Can't match rule".to_string()})
}
Ok((GraphVizNode::new_node(id, "S".to_string(), children),(res)))
}

fn C(&mut self,) -> Result<(GraphVizNode, (i64)),ParseError> {
let mut res:i64;
let id = self.counter.to_string();
let mut children: Vec<GraphVizNode> = Vec::new();
self.counter += 1;
let token = &self.tokens[self.pointer];
match token {
Token::Num(_)|Token::Minus(_)|Token::Lb(_)=> {
let (child, (E0_res)) = self.E()?;
children.push(child);
let (child, (Cx1_res)) = self.Cx(E0_res)?;
children.push(child);
res = Cx1_res
}
_ => return Err(ParseError{position: self.pointer, message: "Can't match rule".to_string()})
}
Ok((GraphVizNode::new_node(id, "C".to_string(), children),(res)))
}

fn Cx(&mut self,acc:i64,) -> Result<(GraphVizNode, (i64)),ParseError> {
let mut res:i64;
let id = self.counter.to_string();
let mut children: Vec<GraphVizNode> = Vec::new();
self.counter += 1;
let token = &self.tokens[self.pointer];
match token {
Token::Comb(_)=> {
let (child, (Comb0_ident)) = self.Comb()?;
children.push(child);
let (child, (E1_res)) = self.E()?;
children.push(child);
let (child, (Cx2_res)) = self.Cx(comb(acc, E1_res))?;
children.push(child);
res = Cx2_res
}
Token::Eof=> {
res = acc;
}
_ => return Err(ParseError{position: self.pointer, message: "Can't match rule".to_string()})
}
Ok((GraphVizNode::new_node(id, "Cx".to_string(), children),(res)))
}

fn E(&mut self,) -> Result<(GraphVizNode, (i64)),ParseError> {
let mut res:i64;
let id = self.counter.to_string();
let mut children: Vec<GraphVizNode> = Vec::new();
self.counter += 1;
let token = &self.tokens[self.pointer];
match token {
Token::Lb(_)|Token::Minus(_)|Token::Num(_)=> {
let (child, (T0_res)) = self.T()?;
children.push(child);
let (child, (Ex1_res)) = self.Ex(T0_res)?;
children.push(child);
res = Ex1_res;
}
_ => return Err(ParseError{position: self.pointer, message: "Can't match rule".to_string()})
}
Ok((GraphVizNode::new_node(id, "E".to_string(), children),(res)))
}

fn Ex(&mut self,acc:i64,) -> Result<(GraphVizNode, (i64)),ParseError> {
let mut res:i64;
let id = self.counter.to_string();
let mut children: Vec<GraphVizNode> = Vec::new();
self.counter += 1;
let token = &self.tokens[self.pointer];
match token {
Token::Plus(_)=> {
let (child, (Plus0_ident)) = self.Plus()?;
children.push(child);
let (child, (T1_res)) = self.T()?;
children.push(child);
let (child, (Ex2_res)) = self.Ex(acc + T1_res)?;
children.push(child);
res = Ex2_res;
}
Token::Minus(_)=> {
let (child, (Minus0_ident)) = self.Minus()?;
children.push(child);
let (child, (T1_res)) = self.T()?;
children.push(child);
let (child, (Ex2_res)) = self.Ex(acc - T1_res)?;
children.push(child);
res = Ex2_res;
}
Token::Rb(_)|Token::Eof|Token::Comb(_)=> {
res = acc;
}
_ => return Err(ParseError{position: self.pointer, message: "Can't match rule".to_string()})
}
Ok((GraphVizNode::new_node(id, "Ex".to_string(), children),(res)))
}

fn T(&mut self,) -> Result<(GraphVizNode, (i64)),ParseError> {
let mut res:i64;
let id = self.counter.to_string();
let mut children: Vec<GraphVizNode> = Vec::new();
self.counter += 1;
let token = &self.tokens[self.pointer];
match token {
Token::Minus(_)|Token::Num(_)|Token::Lb(_)=> {
let (child, (F0_res)) = self.F()?;
children.push(child);
let (child, (Tx1_res)) = self.Tx(F0_res)?;
children.push(child);
res = Tx1_res;
}
_ => return Err(ParseError{position: self.pointer, message: "Can't match rule".to_string()})
}
Ok((GraphVizNode::new_node(id, "T".to_string(), children),(res)))
}

fn Tx(&mut self,acc:i64,) -> Result<(GraphVizNode, (i64)),ParseError> {
let mut res:i64;
let id = self.counter.to_string();
let mut children: Vec<GraphVizNode> = Vec::new();
self.counter += 1;
let token = &self.tokens[self.pointer];
match token {
Token::Mul(_)=> {
let (child, (Mul0_ident)) = self.Mul()?;
children.push(child);
let (child, (F1_res)) = self.F()?;
children.push(child);
let (child, (Tx2_res)) = self.Tx(acc * F1_res)?;
children.push(child);
res = Tx2_res;
}
Token::Div(_)=> {
let (child, (Div0_ident)) = self.Div()?;
children.push(child);
let (child, (F1_res)) = self.F()?;
children.push(child);
let (child, (Tx2_res)) = self.Tx(acc / F1_res)?;
children.push(child);
res = Tx2_res;
}
Token::Plus(_)|Token::Comb(_)|Token::Eof|Token::Rb(_)|Token::Minus(_)=> {
res = acc;
}
_ => return Err(ParseError{position: self.pointer, message: "Can't match rule".to_string()})
}
Ok((GraphVizNode::new_node(id, "Tx".to_string(), children),(res)))
}

fn F(&mut self,) -> Result<(GraphVizNode, (i64)),ParseError> {
let mut res:i64;
let id = self.counter.to_string();
let mut children: Vec<GraphVizNode> = Vec::new();
self.counter += 1;
let token = &self.tokens[self.pointer];
match token {
Token::Minus(_)=> {
let (child, (Minus0_ident)) = self.Minus()?;
children.push(child);
let (child, (F1_res)) = self.F()?;
children.push(child);
res = - F1_res;
}
Token::Lb(_)|Token::Num(_)=> {
let (child, (P0_res)) = self.P()?;
children.push(child);
res = P0_res;
}
_ => return Err(ParseError{position: self.pointer, message: "Can't match rule".to_string()})
}
Ok((GraphVizNode::new_node(id, "F".to_string(), children),(res)))
}

fn P(&mut self,) -> Result<(GraphVizNode, (i64)),ParseError> {
let mut res:i64;
let id = self.counter.to_string();
let mut children: Vec<GraphVizNode> = Vec::new();
self.counter += 1;
let token = &self.tokens[self.pointer];
match token {
Token::Lb(_)=> {
let (child, (Lb0_ident)) = self.Lb()?;
children.push(child);
let (child, (E1_res)) = self.E()?;
children.push(child);
let (child, (Rb2_ident)) = self.Rb()?;
children.push(child);
res = E1_res;
}
Token::Num(_)=> {
let (child, (Num0_ident)) = self.Num()?;
children.push(child);
res = Num0_ident.parse().unwrap();
}
_ => return Err(ParseError{position: self.pointer, message: "Can't match rule".to_string()})
}
Ok((GraphVizNode::new_node(id, "P".to_string(), children),(res)))
}

}pub fn parse(input:String,) -> Result<(GraphVizNode, (i64)),ParseError> { Parser::new(Tokenizer::tokenize(input)?).S() }