#![allow(warnings, unused, non_snake_case, non_camel_case_types)]

use regex::Regex;
use lazy_static::lazy_static;
use graph_viz::GraphVizNode;


use self::SyntaxTree::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Token_ {
    LeftBracket,
    RightBracket,
    Var,
    And,
    Or,
    Xor,
    Not,
    Eof,
    Eps,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum NotTerminal {
    X,
    X_,
    O,
    O_,
    A,
    A_,
    N,
    T,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SyntaxTree {
    Leaf(Token_),
    Tree(NotTerminal, Vec<SyntaxTree>),
}


#[derive(Debug)]
pub enum Token {
    Xor(String),
    Or(String),
    And(String),
    Not(String),
    Var(String),
    LB(String),
    RB(String),
    Eof,
}

lazy_static!(
static ref Xor_regex: Regex = Regex::new("xor").unwrap(); 
static ref Or_regex: Regex = Regex::new("or").unwrap(); 
static ref And_regex: Regex = Regex::new("and").unwrap(); 
static ref Not_regex: Regex = Regex::new("not").unwrap(); 
static ref Var_regex: Regex = Regex::new("[a-zA-Z]").unwrap(); 
static ref LB_regex: Regex = Regex::new("\\(").unwrap(); 
static ref RB_regex: Regex = Regex::new("\\)").unwrap(); 
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
        if let Some(m) = Xor_regex.find_at(&self.input, self.pointer) {
            if m.start() == self.pointer {
                self.pointer = m.end();
                return Some(Token::Xor(self.input[m.start()..m.end()].to_string()));
            }
        }
        if let Some(m) = Or_regex.find_at(&self.input, self.pointer) {
            if m.start() == self.pointer {
                self.pointer = m.end();
                return Some(Token::Or(self.input[m.start()..m.end()].to_string()));
            }
        }
        if let Some(m) = And_regex.find_at(&self.input, self.pointer) {
            if m.start() == self.pointer {
                self.pointer = m.end();
                return Some(Token::And(self.input[m.start()..m.end()].to_string()));
            }
        }
        if let Some(m) = Not_regex.find_at(&self.input, self.pointer) {
            if m.start() == self.pointer {
                self.pointer = m.end();
                return Some(Token::Not(self.input[m.start()..m.end()].to_string()));
            }
        }
        if let Some(m) = Var_regex.find_at(&self.input, self.pointer) {
            if m.start() == self.pointer {
                self.pointer = m.end();
                return Some(Token::Var(self.input[m.start()..m.end()].to_string()));
            }
        }
        if let Some(m) = LB_regex.find_at(&self.input, self.pointer) {
            if m.start() == self.pointer {
                self.pointer = m.end();
                return Some(Token::LB(self.input[m.start()..m.end()].to_string()));
            }
        }
        if let Some(m) = RB_regex.find_at(&self.input, self.pointer) {
            if m.start() == self.pointer {
                self.pointer = m.end();
                return Some(Token::RB(self.input[m.start()..m.end()].to_string()));
            }
        }
        None
    }
}


impl Tokenizer {
    fn new(input: String) -> Tokenizer {
        Tokenizer {
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
            );
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
        Parser { tokens, pointer: 0, counter: 0 }
    }
}

impl Parser {
    fn Xor(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Xor(s) => Ok((GraphVizNode::new_leaf(id, "Xor".to_string()), s.clone())),
            _ => Err(ParseError { position: pos, message: "Expected Xor".to_string() }),
        }
    }
    fn LB(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::LB(s) => Ok((GraphVizNode::new_leaf(id, "LB".to_string()), s.clone())),
            _ => Err(ParseError { position: pos, message: "Expected LB".to_string() }),
        }
    }
    fn Or(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Or(s) => Ok((GraphVizNode::new_leaf(id, "Or".to_string()), s.clone())),
            _ => Err(ParseError { position: pos, message: "Expected Or".to_string() }),
        }
    }
    fn And(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::And(s) => Ok((GraphVizNode::new_leaf(id, "And".to_string()), s.clone())),
            _ => Err(ParseError { position: pos, message: "Expected And".to_string() }),
        }
    }
    fn RB(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::RB(s) => Ok((GraphVizNode::new_leaf(id, "RB".to_string()), s.clone())),
            _ => Err(ParseError { position: pos, message: "Expected RB".to_string() }),
        }
    }
    fn Var(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Var(s) => Ok((GraphVizNode::new_leaf(id, "Var".to_string()), s.clone())),
            _ => Err(ParseError { position: pos, message: "Expected Var".to_string() }),
        }
    }
    fn Not(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Not(s) => Ok((GraphVizNode::new_leaf(id, "Not".to_string()), s.clone())),
            _ => Err(ParseError { position: pos, message: "Expected Not".to_string() }),
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
            _ => Err(ParseError { position: pos, message: "Expected eof".to_string() })
        }
    }
    fn S(&mut self) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> {
        let mut tree: SyntaxTree;
        let id = self.counter.to_string();
        let mut children: Vec<GraphVizNode> = Vec::new();
        self.counter += 1;
        let token = &self.tokens[self.pointer];
        match token {
            Token::Var(_) | Token::Not(_) | Token::LB(_) => {
                let (child, (X0_tree)) = self.X()?;
                children.push(child);
                tree = X0_tree;
            }
            _ => return Err(ParseError { position: self.pointer, message: "Can't match rule".to_string() })
        }
        Ok((GraphVizNode::new_node(id, "S".to_string(), children), (tree)))
    }

    fn X(&mut self) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> {
        let mut tree: SyntaxTree;
        let id = self.counter.to_string();
        let mut children: Vec<GraphVizNode> = Vec::new();
        self.counter += 1;
        let token = &self.tokens[self.pointer];
        match token {
            Token::Not(_) | Token::Var(_) | Token::LB(_) => {
                let (child, (O0_tree)) = self.O()?;
                children.push(child);
                let (child, (Xx1_tree)) = self.Xx()?;
                children.push(child);

                tree = Tree(NotTerminal::X, vec!(O0_tree, Xx1_tree));
            }
            _ => return Err(ParseError { position: self.pointer, message: "Can't match rule".to_string() })
        }
        Ok((GraphVizNode::new_node(id, "X".to_string(), children), (tree)))
    }

    fn Xx(&mut self) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> {
        let mut tree: SyntaxTree;
        let id = self.counter.to_string();
        let mut children: Vec<GraphVizNode> = Vec::new();
        self.counter += 1;
        let token = &self.tokens[self.pointer];
        match token {
            Token::Xor(_) => {
                let (child, (Xor0_ident)) = self.Xor()?;
                children.push(child);
                let (child, (O1_tree)) = self.O()?;
                children.push(child);
                let (child, (Xx2_tree)) = self.Xx()?;
                children.push(child);

                tree = Tree(NotTerminal::X_, vec!(Leaf(Token_::Xor), O1_tree, Xx2_tree));
            }
            Token::RB(_) | Token::Eof => {
                tree = Tree(NotTerminal::X_, vec!(Leaf(Token_::Eps)));
            }
            _ => return Err(ParseError { position: self.pointer, message: "Can't match rule".to_string() })
        }
        Ok((GraphVizNode::new_node(id, "Xx".to_string(), children), (tree)))
    }

    fn O(&mut self) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> {
        let mut tree: SyntaxTree;
        let id = self.counter.to_string();
        let mut children: Vec<GraphVizNode> = Vec::new();
        self.counter += 1;
        let token = &self.tokens[self.pointer];
        match token {
            Token::Var(_) | Token::LB(_) | Token::Not(_) => {
                let (child, (A0_tree)) = self.A()?;
                children.push(child);
                let (child, (Ox1_tree)) = self.Ox()?;
                children.push(child);

                tree = Tree(NotTerminal::O, vec!(A0_tree, Ox1_tree));
            }
            _ => return Err(ParseError { position: self.pointer, message: "Can't match rule".to_string() })
        }
        Ok((GraphVizNode::new_node(id, "O".to_string(), children), (tree)))
    }

    fn Ox(&mut self) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> {
        let mut tree: SyntaxTree;
        let id = self.counter.to_string();
        let mut children: Vec<GraphVizNode> = Vec::new();
        self.counter += 1;
        let token = &self.tokens[self.pointer];
        match token {
            Token::Or(_) => {
                let (child, (Or0_ident)) = self.Or()?;
                children.push(child);
                let (child, (A1_tree)) = self.A()?;
                children.push(child);
                let (child, (Ox2_tree)) = self.Ox()?;
                children.push(child);

                tree = Tree(NotTerminal::O_, vec!(Leaf(Token_::Or), A1_tree, Ox2_tree));
            }
            Token::Xor(_) | Token::Eof | Token::RB(_) => {
                tree = Tree(NotTerminal::O_, vec!(Leaf(Token_::Eps)));
            }
            _ => return Err(ParseError { position: self.pointer, message: "Can't match rule".to_string() })
        }
        Ok((GraphVizNode::new_node(id, "Ox".to_string(), children), (tree)))
    }

    fn A(&mut self) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> {
        let mut tree: SyntaxTree;
        let id = self.counter.to_string();
        let mut children: Vec<GraphVizNode> = Vec::new();
        self.counter += 1;
        let token = &self.tokens[self.pointer];
        match token {
            Token::Var(_) | Token::LB(_) | Token::Not(_) => {
                let (child, (N0_tree)) = self.N()?;
                children.push(child);
                let (child, (Ax1_tree)) = self.Ax()?;
                children.push(child);

                tree = Tree(NotTerminal::A, vec!(N0_tree, Ax1_tree));
            }
            _ => return Err(ParseError { position: self.pointer, message: "Can't match rule".to_string() })
        }
        Ok((GraphVizNode::new_node(id, "A".to_string(), children), (tree)))
    }

    fn Ax(&mut self) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> {
        let mut tree: SyntaxTree;
        let id = self.counter.to_string();
        let mut children: Vec<GraphVizNode> = Vec::new();
        self.counter += 1;
        let token = &self.tokens[self.pointer];
        match token {
            Token::And(_) => {
                let (child, (And0_ident)) = self.And()?;
                children.push(child);
                let (child, (N1_tree)) = self.N()?;
                children.push(child);
                let (child, (Ax2_tree)) = self.Ax()?;
                children.push(child);

                tree = Tree(NotTerminal::A_, vec!(Leaf(Token_::And), N1_tree, Ax2_tree));
            }
            Token::Or(_) | Token::RB(_) | Token::Xor(_) | Token::Eof => {
                tree = Tree(NotTerminal::A_, vec!(Leaf(Token_::Eps)));
            }
            _ => return Err(ParseError { position: self.pointer, message: "Can't match rule".to_string() })
        }
        Ok((GraphVizNode::new_node(id, "Ax".to_string(), children), (tree)))
    }

    fn N(&mut self) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> {
        let mut tree: SyntaxTree;
        let id = self.counter.to_string();
        let mut children: Vec<GraphVizNode> = Vec::new();
        self.counter += 1;
        let token = &self.tokens[self.pointer];
        match token {
            Token::Not(_) => {
                let (child, (Not0_ident)) = self.Not()?;
                children.push(child);
                let (child, (T1_tree)) = self.T()?;
                children.push(child);

                tree = Tree(NotTerminal::N, vec!(Leaf(Token_::Not), T1_tree));
            }
            Token::LB(_) | Token::Var(_) => {
                let (child, (T0_tree)) = self.T()?;
                children.push(child);

                tree = Tree(NotTerminal::N, vec!(T0_tree));
            }
            _ => return Err(ParseError { position: self.pointer, message: "Can't match rule".to_string() })
        }
        Ok((GraphVizNode::new_node(id, "N".to_string(), children), (tree)))
    }

    fn T(&mut self) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> {
        let mut tree: SyntaxTree;
        let id = self.counter.to_string();
        let mut children: Vec<GraphVizNode> = Vec::new();
        self.counter += 1;
        let token = &self.tokens[self.pointer];
        match token {
            Token::Var(_) => {
                let (child, (Var0_ident)) = self.Var()?;
                children.push(child);

                tree = Tree(NotTerminal::T, vec!(Leaf(Token_::Var)));
            }
            Token::LB(_) => {
                let (child, (LB0_ident)) = self.LB()?;
                children.push(child);
                let (child, (X1_tree)) = self.X()?;
                children.push(child);
                let (child, (RB2_ident)) = self.RB()?;
                children.push(child);

                tree = Tree(NotTerminal::T, vec!(Leaf(Token_::LeftBracket), X1_tree, Leaf(Token_::RightBracket)));
            }
            _ => return Err(ParseError { position: self.pointer, message: "Can't match rule".to_string() })
        }
        Ok((GraphVizNode::new_node(id, "T".to_string(), children), (tree)))
    }
}

pub fn parse(input: String) -> Result<(GraphVizNode, (SyntaxTree)), ParseError> { Parser::new(Tokenizer::tokenize(input)?).S() }