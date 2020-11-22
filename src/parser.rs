use crate::tokenizer::{CodeTokenizer, ExpressionToken, ExpressionTokenizer};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

pub struct ASTNode {}
pub struct ParsingResult {}

pub struct Rule<T> {
    expression: Box<dyn ParsingExpression<T>>,
    callback: Option<Box<dyn Fn(ParsingResult) -> T>>,
}

pub struct ParsingInformation<'a, T> {
    rules: &'a HashMap<String, Rule<T>>,
    tokenizer: &'a mut CodeTokenizer,
}

pub trait ParsingExpression<T> {
    fn dump(&self) -> String {
        return String::from("ParsingExpression");
    }
    fn matches(&self, tokenizer: &mut ParsingInformation<T>) -> bool;
}

pub enum TerminalType {
    SIMPLE(String),
    REGEX(Regex),
}

pub struct TerminalParsingExpression<T> {
    content: TerminalType,
    _marker: PhantomData<T>
}

impl<T : 'static> TerminalParsingExpression<T> {
    pub fn new(p_name: &str) -> Box<dyn ParsingExpression<T>> {
        Box::new(TerminalParsingExpression {
            content: TerminalType::SIMPLE(String::from(p_name)),
            _marker: Default::default()
        })
    }
}

impl<T> ParsingExpression<T> for TerminalParsingExpression<T> {
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

    fn matches(&self, info: &mut ParsingInformation<T>) -> bool {
        match &self.content {
            TerminalType::SIMPLE(str) => {
                info.tokenizer.push_state();
                if let Some(token) = info.tokenizer.next_token() {
                    if token.content == *str {
                        info.tokenizer.update_state();
                        true
                    } else {
                        info.tokenizer.pop_state();
                        false
                    }
                } else {
                    info.tokenizer.pop_state();
                    return false;
                }
            }
            TerminalType::REGEX(reg) => todo!(),
        }
    }
}

pub struct NonTerminalParsingExpression<T> {
    name: String,
    _marker: PhantomData<T>
}

impl<T : 'static> NonTerminalParsingExpression<T> {
    pub fn new(p_name: &str) -> Box<dyn ParsingExpression<T>> {
        Box::new(NonTerminalParsingExpression {
            name: String::from(p_name),
            _marker: Default::default()
        })
    }
}

impl<T> ParsingExpression<T> for NonTerminalParsingExpression<T> {
    fn dump(&self) -> String {
        return String::from(format!("{}", self.name));
    }
    fn matches(&self, mut info: &mut ParsingInformation<T>) -> bool {
        return info
            .rules
            .get(&self.name)
            .expect("No rule for this non-terminal!")
            .expression
            .matches(&mut info);
    }
}

pub struct SequenceParsingExpression<T : 'static> {
    children: Vec<Box<dyn ParsingExpression<T>>>,
    _marker: PhantomData<T>
}

impl<T : 'static> SequenceParsingExpression<T> {
    pub fn new(p_children: Vec<Box<dyn ParsingExpression<T>>>) -> Box<dyn ParsingExpression<T>> {
        Box::new(SequenceParsingExpression {
            children: p_children,
            _marker: Default::default()
        })
    }
}

impl<T> ParsingExpression<T> for SequenceParsingExpression<T> {
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
    fn matches(&self, info: &mut ParsingInformation<T>) -> bool {
        info.tokenizer.push_state();
        for child in &self.children {
            if !child.matches(info) {
                info.tokenizer.pop_state();
                return false;
            }
        }
        info.tokenizer.update_state();
        return true;
    }
}

pub struct ChoiceParsingExpression<T : 'static> {
    children: Vec<Box<dyn ParsingExpression<T>>>,
    _marker: PhantomData<T>
}

impl<T : 'static> ChoiceParsingExpression<T> {
    pub fn new(p_children: Vec<Box<dyn ParsingExpression<T>>>) -> Box<dyn ParsingExpression<T>> {
        Box::new(ChoiceParsingExpression {
            children: p_children,
            _marker: Default::default()
        })
    }
}

impl<T> ParsingExpression<T> for ChoiceParsingExpression<T> {
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
    fn matches(&self, info: &mut ParsingInformation<T>) -> bool {
        for child in &self.children {
            info.tokenizer.push_state();
            if child.matches(info) {
                info.tokenizer.update_state();
                return true;
            } else {
                info.tokenizer.pop_state();
            }
        }
        return false;
    }
}

pub struct OneOrMoreParsingExpression<T : 'static> {
    child: Box<dyn ParsingExpression<T>>,
}

impl<T : 'static> OneOrMoreParsingExpression<T> {
    pub fn new(child: Box<dyn ParsingExpression<T>>) -> Box<dyn ParsingExpression<T>> {
        Box::new(OneOrMoreParsingExpression { child })
    }
}
impl<T> ParsingExpression<T> for OneOrMoreParsingExpression<T> {
    fn dump(&self) -> String {
        let mut ret = self.child.dump();
        ret.push('+');
        return ret;
    }
    fn matches(&self, mut info: &mut ParsingInformation<T>) -> bool {
        if !self.child.matches(&mut info) {
            return false;
        }
        while self.child.matches(&mut info) {}
        true
    }
}

pub struct ZeroOrMoreParsingExpression<T : 'static> {
    child: Box<dyn ParsingExpression<T>>,
}

impl<T : 'static> ZeroOrMoreParsingExpression<T> {
    pub fn new(child: Box<dyn ParsingExpression<T>>) -> Box<dyn ParsingExpression<T>> {
        Box::new(ZeroOrMoreParsingExpression { child })
    }
}
impl<T> ParsingExpression<T> for ZeroOrMoreParsingExpression<T> {
    fn dump(&self) -> String {
        let mut ret = self.child.dump();
        ret.push('*');
        return ret;
    }
    fn matches(&self, mut info: &mut ParsingInformation<T>) -> bool {
        while self.child.matches(&mut info) {}
        return true;
    }
}

pub struct OptionalParsingExpression<T> {
    child: Box<dyn ParsingExpression<T>>,
}

impl<T : 'static> OptionalParsingExpression<T> {
    pub fn new(child: Box<dyn ParsingExpression<T>>) -> Box<dyn ParsingExpression<T>> {
        Box::new(OptionalParsingExpression { child })
    }
}
impl<T> ParsingExpression<T> for OptionalParsingExpression<T> {
    fn dump(&self) -> String {
        let mut ret = self.child.dump();
        ret.push('?');
        return ret;
    }
    fn matches(&self, mut info: &mut ParsingInformation<T>) -> bool {
        self.child.matches(&mut info);
        return true;
    }
}

pub struct Parser<T> {
    rules: HashMap<String, Rule<T>>,
}

impl<T : 'static> Parser<T> {
    pub fn new() -> Parser<T> {
        Parser {
            rules: HashMap::new(),
        }
    }
    pub fn add_rule(
        &mut self,
        left_side: &str,
        right_side: Box<dyn ParsingExpression<T>>,
        callback: Option<Box<dyn Fn(ParsingResult) -> T>>,
    ) {
        self.rules.insert(
            String::from(left_side),
            Rule {
                expression: right_side,
                callback,
            },
        );
    }
    pub fn validate(&self, start_non_terminal: &str, code: &str) -> bool {
        let mut tokenizer = CodeTokenizer::new(code);
        let rule = self
            .rules
            .get(start_non_terminal)
            .expect("No matching rule for non-terminal!");
        let rule_result = rule.expression.matches(&mut ParsingInformation {
            rules: &self.rules,
            tokenizer: &mut tokenizer,
        });
        assert!(tokenizer.only_one_state_left());
        rule_result && tokenizer.is_empty()
    }

    pub fn add_rule_str(&mut self, left_side: &str, right_side: &str, callback: Option<Box<dyn Fn(ParsingResult) -> T>>) {
        self.add_rule(
            left_side,
            Self::parse_rule(&mut ExpressionTokenizer::new(right_side)),
            callback
        );
    }

    fn parse_rule(tokenizer: &mut ExpressionTokenizer) -> Box<dyn ParsingExpression<T>> {
        let mut sequence = Vec::new();
        let mut ordering = Vec::new();
        loop {
            if let Some(token) = tokenizer.next_token() {
                let expr = match token {
                    ExpressionToken::GroupBegin => Some(Self::parse_rule(tokenizer)),
                    ExpressionToken::GroupEnd => {
                        if ordering.len() > 0 {
                            ordering.push(
                                Self::vec_to_expression(sequence).expect("Invalid PEG grammar"),
                            );
                            return ChoiceParsingExpression::new(ordering);
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
            ChoiceParsingExpression::new(ordering)
        } else {
            Self::vec_to_expression(sequence).expect("Invalid PEG grammar")
        };
    }

    fn vec_to_expression(
        mut vec: Vec<Box<dyn ParsingExpression<T>>>,
    ) -> Option<Box<dyn ParsingExpression<T>>> {
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

impl<T> fmt::Display for Parser<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: fmt::Result = fmt::Result::Ok(());
        for (left_side, right_side) in &self.rules {
            res = res.and(write!(
                f,
                "{} -> {}",
                left_side,
                right_side.expression.dump()
            ));
        }
        res
    }
}
