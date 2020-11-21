use crate::tokenizer::CodeTokenizer;
use std::collections::HashMap;
use std::fmt;

pub struct ParsingResult {}

pub trait ParsingExpression {
    fn matches(&self, tokenizer: &mut CodeTokenizer) -> ParsingResult;
    fn dump(&self) -> String {
        return String::from("ParsingExpression");
    }
}

pub struct TerminalParsingExpression {
    name: String,
}

impl TerminalParsingExpression {
    pub fn new(p_name: &str) -> Box<dyn ParsingExpression> {
        Box::new(TerminalParsingExpression {
            name: String::from(p_name),
        })
    }
}

impl ParsingExpression for TerminalParsingExpression {
    fn matches(&self, _tokenizer: &mut CodeTokenizer) -> ParsingResult {
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
        Box::new(NonTerminalParsingExpression {
            name: String::from(p_name),
        })
    }
}

impl ParsingExpression for NonTerminalParsingExpression {
    fn matches(&self, _tokenizer: &mut CodeTokenizer) -> ParsingResult {
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
    fn matches(&self, _tokenizer: &mut CodeTokenizer) -> ParsingResult {
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
    fn matches(&self, _tokenizer: &mut CodeTokenizer) -> ParsingResult {
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
pub struct Parser {
    rules: HashMap<String, Box<dyn ParsingExpression>>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            rules: HashMap::new(),
        }
    }
    pub fn add_rule(&mut self, left_side: &str, right_side: Box<dyn ParsingExpression>) {
        self.rules.insert(String::from(left_side), right_side);
    }
    pub fn validate(&self, start_non_terminal: &str, code: &str) -> bool {
        let mut tokenizer = CodeTokenizer::new(code);
        let rule = self
            .rules
            .get(start_non_terminal)
            .expect("No matching rule for non-terminal!");
        rule.validate(tokenizer, &self.rules);
        true
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: fmt::Result = fmt::Result::Ok(());
        for (left_side, right_side) in &self.rules {
            res = res.and(write!(f, "{} -> {}", left_side, right_side.dump()));
        }
        res
    }
}
