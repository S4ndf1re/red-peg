use crate::code_tokenizer::CodeTokenizer;
use crate::expression_tokenizer::{ExpressionToken, ExpressionTokenizer};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::ops;

pub struct ASTNode {}
pub struct ParsingResult<T> {
    pub parsed_string_start: usize,
    pub parsed_string_end: usize,
    pub sub_results: Vec<ParsingResult<T>>,
    pub selected_choice: Option<usize>,
    pub rule_result: Option<T>,
}

impl<T> ParsingResult<T> {
    pub fn flatten(self) -> Vec<ParsingResult<T>> {
        let mut v = Vec::new();
        self.flatten_rec(&mut v);
        return v;
    }

    fn flatten_rec(mut self, to: &mut Vec<ParsingResult<T>>) {
        if self.sub_results.len() == 0 {
            return;
        }
        let sub_results_cpy = self.sub_results;
        self.sub_results = Vec::new();
        if self.selected_choice.is_some() || self.rule_result.is_some() {
            to.push(self);
        }
        for r in sub_results_cpy {
            to.push(r);
        }
    }
}

impl<T> ops::Index<usize> for ParsingResult<T> {
    type Output = ParsingResult<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sub_results.get(index).unwrap()
    }
}

type RuleCallback<T> = Box<dyn Fn(ParsingResult<T>, &CodeTokenizer) -> T>;
pub struct Rule<T> {
    expression: Box<dyn ParsingExpression<T>>,
    callback: Option<RuleCallback<T>>,
}

pub struct ParsingInformation<'a, T> {
    rules: &'a HashMap<String, Rule<T>>,
    tokenizer: &'a mut CodeTokenizer,
}

pub trait ParsingExpression<T> {
    fn dump(&self) -> String {
        return String::from("ParsingExpression");
    }
    fn matches(&self, tokenizer: &mut ParsingInformation<T>) -> Option<ParsingResult<T>>;
}

pub enum TerminalType {
    SIMPLE(String),
    REGEX(Regex),
}

pub struct TerminalParsingExpression<T> {
    content: TerminalType,
    _marker: PhantomData<T>,
}

impl<T: 'static> TerminalParsingExpression<T> {
    pub fn new(p_name: &str) -> Box<dyn ParsingExpression<T>> {
        Box::new(TerminalParsingExpression {
            content: TerminalType::SIMPLE(String::from(p_name)),
            _marker: Default::default(),
        })
    }
    pub fn new_from_regex(p_name: &str) -> Box<dyn ParsingExpression<T>> {
        Box::new(TerminalParsingExpression {
            content: TerminalType::REGEX(Regex::new(p_name).unwrap()),
            _marker: Default::default(),
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

    fn matches(&self, info: &mut ParsingInformation<T>) -> Option<ParsingResult<T>> {
        let start = info.tokenizer.push_state();
        let does_match = match &self.content {
            TerminalType::SIMPLE(str) => {
                info.tokenizer.match_string(str.as_ref())
            }
            TerminalType::REGEX(reg) => {
                info.tokenizer.match_regex(reg)
            }
        };
        if does_match {
            Some(ParsingResult {
                parsed_string_start: start,
                parsed_string_end: info.tokenizer.update_state(),
                sub_results: Vec::new(),
                selected_choice: None,
                rule_result: None,
            })
        } else {
            info.tokenizer.pop_state();
            None
        }
    }
}

pub struct NonTerminalParsingExpression<T> {
    name: String,
    _marker: PhantomData<T>,
}

impl<T: 'static> NonTerminalParsingExpression<T> {
    pub fn new(p_name: &str) -> Box<dyn ParsingExpression<T>> {
        Box::new(NonTerminalParsingExpression {
            name: String::from(p_name),
            _marker: Default::default(),
        })
    }
}

impl<T> ParsingExpression<T> for NonTerminalParsingExpression<T> {
    fn dump(&self) -> String {
        return String::from(format!("{}", self.name));
    }
    fn matches(&self, mut info: &mut ParsingInformation<T>) -> Option<ParsingResult<T>> {
        let rule = info
            .rules
            .get(&self.name)
            .expect("No rule for this non-terminal!");
        match rule.expression.matches(&mut info) {
            Some(res) => {
                let mut res_cpy = ParsingResult{
                    parsed_string_start: res.parsed_string_start,
                    parsed_string_end: res.parsed_string_end,
                    sub_results: vec![],
                    selected_choice: res.selected_choice,
                    rule_result: None
                };
                if let Some(ref callback) = rule.callback {
                    res_cpy.rule_result = Some(callback(res, &info.tokenizer));
                }
                Some(res_cpy)
            }
            None => None,
        }
    }
}

pub struct SequenceParsingExpression<T: 'static> {
    children: Vec<Box<dyn ParsingExpression<T>>>,
    _marker: PhantomData<T>,
}

impl<T: 'static> SequenceParsingExpression<T> {
    pub fn new(p_children: Vec<Box<dyn ParsingExpression<T>>>) -> Box<dyn ParsingExpression<T>> {
        Box::new(SequenceParsingExpression {
            children: p_children,
            _marker: Default::default(),
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
    fn matches(&self, info: &mut ParsingInformation<T>) -> Option<ParsingResult<T>> {
        let start = info.tokenizer.push_state();
        let mut result = ParsingResult {
            parsed_string_start: start,
            parsed_string_end: 0,
            sub_results: Vec::new(),
            selected_choice: None,
            rule_result: None,
        };
        for child in &self.children {
            let child_result = child.matches(info);
            if child_result.is_none() {
                info.tokenizer.pop_state();
                return None;
            }
            result.sub_results.push(child_result.unwrap());
        }
        result.parsed_string_end = info.tokenizer.update_state();
        return Some(result);
    }
}

pub struct ChoiceParsingExpression<T: 'static> {
    children: Vec<Box<dyn ParsingExpression<T>>>,
    _marker: PhantomData<T>,
}

impl<T: 'static> ChoiceParsingExpression<T> {
    pub fn new(p_children: Vec<Box<dyn ParsingExpression<T>>>) -> Box<dyn ParsingExpression<T>> {
        Box::new(ChoiceParsingExpression {
            children: p_children,
            _marker: Default::default(),
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
    fn matches(&self, mut info: &mut ParsingInformation<T>) -> Option<ParsingResult<T>> {
        let mut i = 0usize;
        for child in &self.children {
            let start = info.tokenizer.push_state();

            match child.matches(&mut info) {
                Some(child_res) => {
                    return Some(ParsingResult {
                        parsed_string_start: start,
                        parsed_string_end: info.tokenizer.update_state(),
                        sub_results: vec![child_res],
                        selected_choice: Some(i),
                        rule_result: None,
                    })
                }
                None => info.tokenizer.pop_state(),
            }
            i += 1;
        }
        return None;
    }
}

pub struct OneOrMoreParsingExpression<T: 'static> {
    child: Box<dyn ParsingExpression<T>>,
}

impl<T: 'static> OneOrMoreParsingExpression<T> {
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
    fn matches(&self, mut info: &mut ParsingInformation<T>) -> Option<ParsingResult<T>> {
        let mut res = ParsingResult {
            parsed_string_start: info.tokenizer.get_state(),
            parsed_string_end: 0,
            sub_results: Vec::new(),
            selected_choice: None,
            rule_result: None,
        };
        match self.child.matches(&mut info) {
            Some(child_res) => res.sub_results.push(child_res),
            None => return None,
        }
        loop {
            match self.child.matches(&mut info) {
                Some(child_res) => res.sub_results.push(child_res),
                None => break,
            }
        }
        res.parsed_string_end = info.tokenizer.get_state();
        Some(res)
    }
}

pub struct ZeroOrMoreParsingExpression<T: 'static> {
    child: Box<dyn ParsingExpression<T>>,
}

impl<T: 'static> ZeroOrMoreParsingExpression<T> {
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
    fn matches(&self, mut info: &mut ParsingInformation<T>) -> Option<ParsingResult<T>> {
        let mut res = ParsingResult {
            parsed_string_start: info.tokenizer.get_state(),
            parsed_string_end: 0,
            sub_results: Vec::new(),
            selected_choice: None,
            rule_result: None,
        };
        loop {
            match self.child.matches(&mut info) {
                Some(child_res) => res.sub_results.push(child_res),
                None => break,
            }
        }
        res.parsed_string_end = info.tokenizer.get_state();
        Some(res)
    }
}

pub struct OptionalParsingExpression<T> {
    child: Box<dyn ParsingExpression<T>>,
}

impl<T: 'static> OptionalParsingExpression<T> {
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
    fn matches(&self, mut info: &mut ParsingInformation<T>) -> Option<ParsingResult<T>> {
        match self.child.matches(&mut info) {
            Some(res) => Some(res),
            None => Some(ParsingResult {
                parsed_string_start: info.tokenizer.get_state(),
                parsed_string_end: info.tokenizer.get_state(),
                sub_results: Vec::new(),
                selected_choice: None,
                rule_result: None,
            }),
        }
    }
}

pub struct Parser<T> {
    rules: HashMap<String, Rule<T>>,
}

impl<T: 'static> Parser<T> {
    pub fn new() -> Parser<T> {
        Parser {
            rules: HashMap::new(),
        }
    }
    pub fn add_rule(
        &mut self,
        left_side: &str,
        right_side: Box<dyn ParsingExpression<T>>,
        callback: Option<RuleCallback<T>>,
    ) {
        assert!(!self.rules.contains_key(left_side));
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
        let rule_result = NonTerminalParsingExpression::new(start_non_terminal).matches(
            &mut ParsingInformation {
                rules: &self.rules,
                tokenizer: &mut tokenizer,
            },
        );
        assert!(tokenizer.only_one_state_left());
        rule_result.is_some() && tokenizer.is_empty()
    }

    pub fn parse(&self, start_non_terminal: &str, code: &str) -> Result<T, &'static str> {
        let mut tokenizer = CodeTokenizer::new(code);
        let rule_result = NonTerminalParsingExpression::new(start_non_terminal).matches(
            &mut ParsingInformation {
                rules: &self.rules,
                tokenizer: &mut tokenizer,
            },
        );
        assert!(tokenizer.only_one_state_left());

        match rule_result {
            None => Err("There is no result!"),
            Some(parsing_result) => match parsing_result.rule_result {
                Some(rule_result) =>
                    if !tokenizer.is_empty() {
                        Err("There are tokens that haven't been parsed!")
                    } else {
                        Ok(rule_result)
                    },
                None => Err("There is no callback registered for the rule!"),
            },
        }
    }

    pub fn add_rule_str(
        &mut self,
        left_side: &str,
        right_side: &str,
        callback: Option<RuleCallback<T>>,
    ) {
        self.add_rule(
            left_side,
            Self::parse_rule(&mut ExpressionTokenizer::new(right_side)),
            callback,
        );
    }

    fn parse_rule(tokenizer: &mut ExpressionTokenizer) -> Box<dyn ParsingExpression<T>> {
        let mut sequence = Vec::new();
        let mut choices = Vec::new();
        loop {
            if let Some(token) = tokenizer.next_token() {
                let expr = match token {
                    ExpressionToken::GroupBegin => Some(Self::parse_rule(tokenizer)),
                    ExpressionToken::GroupEnd => {
                        if choices.len() > 0 {
                            choices.push(
                                Self::vec_to_expression(sequence).expect("Invalid PEG grammar"),
                            );
                            return ChoiceParsingExpression::new(choices);
                        } else {
                            return Self::vec_to_expression(sequence).expect("Invalid PEG grammar");
                        }
                    }
                    ExpressionToken::Expression(val) => {
                        Some(NonTerminalParsingExpression::new(val.as_str()))
                    }
                    ExpressionToken::TerminalExpression(val) => {
                        Some(TerminalParsingExpression::new(val.as_str()))
                    },
                    ExpressionToken::TerminalRegexExpression(val) => {
                        Some(TerminalParsingExpression::new_from_regex(val.as_str()))
                    }
                    ExpressionToken::Choice => {
                        choices
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
                    ExpressionToken::None => None
                };

                if let Some(val) = expr {
                    sequence.push(val);
                }
            } else {
                break;
            }
        }

        return if choices.len() >= 1 {
            choices.push(Self::vec_to_expression(sequence).expect("Invalid PEG grammar"));
            ChoiceParsingExpression::new(choices)
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
