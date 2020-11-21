use crate::tokenizer::CodeTokenizer;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

pub struct ASTNode {}
pub struct ParsingResult {}

pub struct Rule {
    expression: Box<dyn ParsingExpression>,
    callback: Option<Box<dyn FnMut(ParsingResult) -> ASTNode>>,
}

pub struct ParsingInformation<'a> {
    rules: &'a HashMap<String, Rule>,
    tokenizer: &'a mut CodeTokenizer,
}

pub trait ParsingExpression {
    fn dump(&self) -> String {
        return String::from("ParsingExpression");
    }
    fn matches(&self, tokenizer: &mut ParsingInformation) -> bool;
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

    fn matches(&self, info: &mut ParsingInformation) -> bool {
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
    fn dump(&self) -> String {
        return String::from(format!("{}", self.name));
    }
    fn matches(&self, mut info: &mut ParsingInformation) -> bool {
        return info
            .rules
            .get(&self.name)
            .expect("No rule for this non-terminal!")
            .expression
            .matches(&mut info);
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
    fn matches(&self, info: &mut ParsingInformation) -> bool {
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
    fn matches(&self, info: &mut ParsingInformation) -> bool {
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

pub struct OneOrMoreParsingExpression {
    child: Box<dyn ParsingExpression>,
}

impl OneOrMoreParsingExpression {
    pub fn new(child: Box<dyn ParsingExpression>) -> Box<dyn ParsingExpression> {
        Box::new(OneOrMoreParsingExpression { child })
    }
}
impl ParsingExpression for OneOrMoreParsingExpression {
    fn dump(&self) -> String {
        let mut ret = self.child.dump();
        ret.push('+');
        return ret;
    }
    fn matches(&self, mut info: &mut ParsingInformation) -> bool {
        if !self.child.matches(&mut info) {
            return false;
        }
        while self.child.matches(&mut info) {}
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
    fn dump(&self) -> String {
        let mut ret = self.child.dump();
        ret.push('*');
        return ret;
    }
    fn matches(&self, mut info: &mut ParsingInformation) -> bool {
        while self.child.matches(&mut info) {}
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
    fn dump(&self) -> String {
        let mut ret = self.child.dump();
        ret.push('?');
        return ret;
    }
    fn matches(&self, mut info: &mut ParsingInformation) -> bool {
        self.child.matches(&mut info);
        return true;
    }
}

pub struct Parser {
    rules: HashMap<String, Rule>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            rules: HashMap::new(),
        }
    }
    pub fn add_rule(
        &mut self,
        left_side: &str,
        right_side: Box<dyn ParsingExpression>,
        callback: Option<Box<dyn FnMut(ParsingResult) -> ASTNode>>,
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
}

impl fmt::Display for Parser {
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
