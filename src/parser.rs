use std::fmt;
use crate::tokenizer::Tokenizer;

pub struct ParsingResult {}

pub trait ParsingExpression {
    fn matches(&self, tokenizer: &mut Tokenizer) -> ParsingResult;
    fn dump(&self) -> String {
        return String::from("ParsingExpression");
    }
}

pub struct TerminalParsingExpression {
    name: String,
}

impl TerminalParsingExpression {
    pub fn new(p_name: &str) -> Box<dyn ParsingExpression> {
        Box::new(TerminalParsingExpression { name: String::from(p_name) })
    }
}

impl ParsingExpression for TerminalParsingExpression {
    fn matches(&self, tokenizer: &mut Tokenizer) -> ParsingResult {
        ParsingResult {}
    }
    fn dump(&self) -> String {
        return String::from(format!("'{}'", self.name));
    }
}

pub struct NonTerminalParsingExpression {
    name: String,
}

impl NonTerminalParsingExpression {
    pub fn new(p_name: &str) -> Box<dyn ParsingExpression> {
        Box::new(NonTerminalParsingExpression { name: String::from(p_name) })
    }
}

impl ParsingExpression for NonTerminalParsingExpression {
    fn matches(&self, tokenizer: &mut Tokenizer) -> ParsingResult {
        ParsingResult {}
    }
    fn dump(&self) -> String {
        return String::from(format!("{}", self.name));
    }
}

pub struct SequenceParsingExpression {
    children: Vec<Box<dyn ParsingExpression>>,
}

impl SequenceParsingExpression {
    pub fn new(p_children: Vec<Box<dyn ParsingExpression>>) -> Box<dyn ParsingExpression> {
        Box::new(SequenceParsingExpression {
            children: p_children,
        })
    }
}

impl ParsingExpression for SequenceParsingExpression {
    fn matches(&self, tokenizer: &mut Tokenizer) -> ParsingResult {
        ParsingResult {}
    }
    fn dump(&self) -> String {
        let mut ret = String::new();
        let mut i = 0;
        for child in &self.children {
            ret.push_str(&child.dump());
            if i < self.children.len() - 1 {
                ret.push_str(" ");
            }
            i += 1;
        }
        return ret;
    }
}

pub struct ChoiceParsingExpresion {
    children: Vec<Box<dyn ParsingExpression>>,
}

impl ChoiceParsingExpresion {
    pub fn new(p_children: Vec<Box<dyn ParsingExpression>>) -> Box<dyn ParsingExpression> {
        Box::new(ChoiceParsingExpresion {
            children: p_children,
        })
    }
}

impl ParsingExpression for ChoiceParsingExpresion {
    fn matches(&self, tokenizer: &mut Tokenizer) -> ParsingResult {
        ParsingResult {}
    }
    fn dump(&self) -> String {
        let mut ret = String::from("(");
        let mut i = 0;
        for child in &self.children {
            ret.push_str(&child.dump());
            if i < self.children.len() - 1 {
                ret.push_str(" | ");
            }
            i += 1;
        }
        ret.push_str(")");
        return ret;
    }
}

pub struct ParseRule {
    pub left_side: String,
    pub right_side: Box<dyn ParsingExpression>,
}

impl fmt::Display for ParseRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.left_side, self.right_side.dump())
    }
}

pub struct Parser {
    rules: Vec<ParseRule>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { rules: Vec::new() }
    }
    pub fn add_rule(&mut self, rule: ParseRule) {
        self.rules.push(rule);
    }
}