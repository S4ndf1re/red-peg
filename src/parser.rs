use crate::tokenizer::CodeTokenizer;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

pub struct ParsingResult {}

pub struct ParsingInformation<'a> {
    rules: &'a HashMap<String, Box<dyn ParsingExpression>>,
    tokenizer: &'a mut CodeTokenizer,
}

pub trait ParsingExpression {
    fn matches(&self, tokenizer: &mut CodeTokenizer) -> ParsingResult;
    fn dump(&self) -> String {
        return String::from("ParsingExpression");
    }
    fn validate(&self, tokenizer: &mut ParsingInformation) -> bool;
}

pub enum TerminalType {
    SIMPLE(String),
    REGEX(Regex),
}

pub struct TerminalParsingExpression {
    content: TerminalType,
}

impl TerminalParsingExpression {
    pub fn new(p_name: &str) -> Box<dyn ParsingExpression> {
        Box::new(TerminalParsingExpression {
            content: TerminalType::SIMPLE(String::from(p_name)),
        })
    }
}

impl ParsingExpression for TerminalParsingExpression {
    fn matches(&self, _tokenizer: &mut CodeTokenizer) -> ParsingResult {
        ParsingResult {}
    }
    fn dump(&self) -> String {
        match &self.content {
            TerminalType::SIMPLE(str) => {
                let mut ret = String::from('\'');
                ret.push_str(str);
                ret.push('\'');
                ret
            }
            TerminalType::REGEX(reg) => {
                reg.to_string()
            }
        }
    }

    fn validate(&self, info: &mut ParsingInformation) -> bool {
        match &self.content {
            TerminalType::SIMPLE(str) => {
                let token = info.tokenizer.next_token();
                token.content == *str
            }
            TerminalType::REGEX(reg) => {
                todo!()
            }
        }
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
    fn validate(&self, mut info: &mut ParsingInformation) -> bool {
        return info.rules.get(&self.name).expect("No rule for this non-terminal!").validate(&mut info);
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
    fn validate(&self, info: &mut ParsingInformation) -> bool {
        for child in &self.children {
            if !child.validate(info) {
                return false;
            }
        }
        return true;
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
    fn validate(&self, info: &mut ParsingInformation) -> bool {
        for child in &self.children {
            info.tokenizer.push_state();
            if child.validate(info) {
                return true;
            }
            info.tokenizer.pop_state();
        }
        return false;
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
        let rule_result = rule.validate(&mut ParsingInformation {
            rules: &self.rules,
            tokenizer: &mut tokenizer,
        });
        rule_result && tokenizer.is_empty()
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
