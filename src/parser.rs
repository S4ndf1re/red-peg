use crate::tokenizer::{CodeTokenizer, ExpressionToken, ExpressionTokenizer};
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
            TerminalType::REGEX(reg) => reg.to_string(),
        }
    }

    fn validate(&self, info: &mut ParsingInformation) -> bool {
        match &self.content {
            TerminalType::SIMPLE(str) => {
                info.tokenizer.push_state();
                let token = info.tokenizer.next_token();
                if token.eof {
                    info.tokenizer.pop_state();
                    return false;
                }
                if token.content == *str {
                    info.tokenizer.update_state();
                    true
                } else {
                    info.tokenizer.pop_state();
                    false
                }
            }
            TerminalType::REGEX(reg) => todo!(),
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
        return info
            .rules
            .get(&self.name)
            .expect("No rule for this non-terminal!")
            .validate(&mut info);
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
        info.tokenizer.push_state();
        for child in &self.children {
            if !child.validate(info) {
                info.tokenizer.pop_state();
                return false;
            }
        }
        info.tokenizer.update_state();
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
                info.tokenizer.update_state();
                return true;
            } else {
                info.tokenizer.pop_state();
            }
        }
        return false;
    }
}

pub struct OneOrMoreParsingExpression {
    child: Box<dyn ParsingExpression>,
}

impl OneOrMoreParsingExpression {
    pub fn new(child: Box<dyn ParsingExpression>) -> Box<dyn ParsingExpression> {
        Box::new(OneOrMoreParsingExpression { child })
    }
}
impl ParsingExpression for OneOrMoreParsingExpression {
    fn matches(&self, _tokenizer: &mut CodeTokenizer) -> ParsingResult {
        ParsingResult {}
    }
    fn dump(&self) -> String {
        let mut ret = self.child.dump();
        ret.push('+');
        return ret;
    }
    fn validate(&self, mut info: &mut ParsingInformation) -> bool {
        if !self.child.validate(&mut info) {
            return false;
        }
        while self.child.validate(&mut info) {}
        true
    }
}

pub struct ZeroOrMoreParsingExpression {
    child: Box<dyn ParsingExpression>,
}

impl ZeroOrMoreParsingExpression {
    pub fn new(child: Box<dyn ParsingExpression>) -> Box<dyn ParsingExpression> {
        Box::new(ZeroOrMoreParsingExpression { child })
    }
}
impl ParsingExpression for ZeroOrMoreParsingExpression {
    fn matches(&self, _tokenizer: &mut CodeTokenizer) -> ParsingResult {
        ParsingResult {}
    }
    fn dump(&self) -> String {
        let mut ret = self.child.dump();
        ret.push('*');
        return ret;
    }
    fn validate(&self, mut info: &mut ParsingInformation) -> bool {
        while self.child.validate(&mut info) {}
        return true;
    }
}

pub struct OptionalParsingExpression {
    child: Box<dyn ParsingExpression>,
}

impl OptionalParsingExpression {
    pub fn new(child: Box<dyn ParsingExpression>) -> Box<dyn ParsingExpression> {
        Box::new(OptionalParsingExpression { child })
    }
}
impl ParsingExpression for OptionalParsingExpression {
    fn matches(&self, _tokenizer: &mut CodeTokenizer) -> ParsingResult {
        ParsingResult {}
    }
    fn dump(&self) -> String {
        let mut ret = self.child.dump();
        ret.push('?');
        return ret;
    }
    fn validate(&self, mut info: &mut ParsingInformation) -> bool {
        self.child.validate(&mut info);
        return true;
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
        assert!(tokenizer.only_one_state_left());
        rule_result && tokenizer.is_empty()
    }

    pub fn add_rule_str(&mut self, left_side: &str, right_side: &str) {
        self.add_rule(
            left_side,
            Self::parse_rule(&mut ExpressionTokenizer::new(right_side)),
        );
    }

    fn parse_rule(tokenizer: &mut ExpressionTokenizer) -> Box<dyn ParsingExpression> {
        let mut sequence = Vec::new();
        let mut ordering = Vec::new();
        loop {
            if let Some(token) = tokenizer.next() {
                let expr = match token {
                    ExpressionToken::GroupBegin => Some(Self::parse_rule(tokenizer)),
                    ExpressionToken::GroupEnd => {
                        if ordering.len() > 0 {
                            ordering.push(
                                Self::vec_to_expression(sequence).expect("Invalid PEG grammar"),
                            );
                            return ChoiceParsingExpresion::new(ordering);
                        } else {
                            return Self::vec_to_expression(sequence).expect("Invalid PEG grammar");
                        }
                    }
                    ExpressionToken::Expression(val) => {
                        Some(NonTerminalParsingExpression::new(val.as_str()))
                    }
                    ExpressionToken::TerminalExpression(val) => {
                        Some(TerminalParsingExpression::new(val.as_str()))
                    }
                    ExpressionToken::Ordering => {
                        ordering
                            .push(Self::vec_to_expression(sequence).expect("Invalid PEG grammar"));
                        sequence = Vec::new();
                        None
                    }
                    ExpressionToken::ZeroOrMore => {
                        let child = sequence.remove(sequence.len() - 1); // Panics if invalid grammar
                        sequence.push(ZeroOrMoreParsingExpression::new(child));
                        None
                    }
                    ExpressionToken::OneOrMore => {
                        let child = sequence.remove(sequence.len() - 1); // Panics if invalid grammar
                        sequence.push(OneOrMoreParsingExpression::new(child));
                        None
                    }
                    ExpressionToken::Optional => {
                        let child = sequence.remove(sequence.len() - 1); // Panics if invalid grammar
                        sequence.push(OptionalParsingExpression::new(child));
                        None
                    }
                    _ => None,
                };

                if let Some(val) = expr {
                    sequence.push(val);
                }
            } else {
                break;
            }
        }

        return if ordering.len() >= 1 {
            ordering.push(Self::vec_to_expression(sequence).expect("Invalid PEG grammar"));
            ChoiceParsingExpresion::new(ordering)
        } else {
            Self::vec_to_expression(sequence).expect("Invalid PEG grammar")
        };
    }

    fn vec_to_expression(
        mut vec: Vec<Box<dyn ParsingExpression>>,
    ) -> Option<Box<dyn ParsingExpression>> {
        if !vec.is_empty() {
            if vec.len() > 1 {
                return Some(SequenceParsingExpression::new(vec));
            } else {
                return Some(vec.remove(0));
            }
        }
        return None;
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
