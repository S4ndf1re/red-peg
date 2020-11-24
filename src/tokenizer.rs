use std::fmt;
use regex::Regex;

#[derive(Clone, fmt::Debug)]
pub struct Token {
    pub line: usize,
    pub column: usize,
    pub content: String,
}

#[derive(fmt::Debug)]
pub struct CodeTokenizer {
    code: String,
    tokens: Vec<Token>,
    states: Vec<usize>,
}

impl CodeTokenizer {
    pub fn new(code: &str) -> CodeTokenizer {
        let mut tokenizer = CodeTokenizer {
            code: String::from(code),
            tokens: Vec::new(),
            states: vec![0usize],
        };
        let mut line = 1usize;
        let mut column = 1usize;
        let mut token_start = 0usize;
        let mut just_added_token = false;
        for (i, c) in tokenizer.code.chars().enumerate() {
            if c.is_whitespace() {
                if !just_added_token {
                    tokenizer.tokens.push(Token {
                        line,
                        column: column - (i - token_start),
                        content: String::from(&tokenizer.code[token_start..i]),
                    });
                    just_added_token = true;
                }
                if c == '\n' {
                    column = 1;
                    line += 1;
                    continue;
                }
            } else if just_added_token {
                token_start = i;
                just_added_token = false;
            }
            column += 1;
        }
        if !just_added_token
            && !tokenizer.code[token_start..tokenizer.code.len()]
                .trim()
                .is_empty()
        {
            tokenizer.tokens.push(Token {
                line,
                column,
                content: String::from(&tokenizer.code[token_start..tokenizer.code.len()]),
            });
        }
        return tokenizer;
    }
    pub fn next_token(&mut self) -> Option<&Token> {
        let index = self.states.last_mut().expect("No state left!");
        if self.tokens.len() > *index {
            *index += 1;
            Some(self.tokens.get(*index - 1).expect("Unable to fetch token!"))
        } else {
            None
        }
    }
    pub fn is_empty(&self) -> bool {
        let index = self.states.last().expect("No state left!");
        self.tokens.len() <= *index
    }

    pub fn push_state(&mut self) -> usize {
        self.states
            .push(self.states.last().expect("No current state!").clone());
        *self.states.last().unwrap()
    }
    /* Takes the topmost value in the stack, saves it, pops it off the stack
     * and writes it to the new top-entry.
     */
    pub fn update_state(&mut self) -> usize {
        let current_state = *self.states.last().expect("Nu current state!");
        self.states.pop();
        let state_below = self
            .states
            .last_mut()
            .expect("No state below the current one!");
        *state_below = current_state;
        current_state
    }

    pub fn get_token_sublist(&self, start: usize, end: usize) -> &[Token] {
        return &self.tokens[start..end];
    }
    pub fn get_state(&self) -> usize {
        *self.states.last().expect("No state left!")
    }

    pub fn only_one_state_left(&self) -> bool {
        self.states.len() == 1
    }

    pub fn pop_state(&mut self) {
        self.states.pop().expect("No state left to pop!");
        if self.states.is_empty() {
            panic!("You can't pop the last state!");
        }
    }

    pub fn tokens_len(&self) -> usize {
        self.tokens.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionToken {
    Expression(String),
    TerminalExpression(String),
    TerminalRegexExpression(String),
    GroupBegin,
    GroupEnd,
    ZeroOrMore,
    OneOrMore,
    Optional,
    Choice,
    None, // For ignoring the token
}

#[derive(Debug)]
pub struct ExpressionTokenizer {
    tokens: Vec<ExpressionToken>,
    current: usize,
}

impl ExpressionTokenizer {
    pub fn new(tokenstring: &str) -> Self {
        let mut tokenizer = Self {
            tokens: Vec::new(),
            current: 0,
        };
        let mut last_string = String::new();
        let iter = tokenstring.chars().into_iter();
        let mut last = '\0';
        let mut last_last = '\0'; // NOTE: this is necessary, because of the case \\] in regex parsing
        let mut in_terminal = false;
        let mut terminal_char = '\0';
        for c in iter {
            // last and last_last are needed to prevent \] from getting accepted and to allow \\] getting accepted
            if in_terminal {
                last_string.push(c);
                if c == terminal_char && (last != '\\' || (last == last_last && last == '\\')) {
                    in_terminal = false;
                    tokenizer.append_last(last_string);
                    last_string = String::new();
                }
            } else {
                let expr = match c {
                    '(' => Some(ExpressionToken::GroupBegin),
                    ')' => Some(ExpressionToken::GroupEnd),
                    '[' | '\'' | '\"' => {
                        terminal_char = match c {
                            '[' => ']',
                            val => val,
                        };
                        in_terminal = true;
                        None
                    }
                    '?' => Some(ExpressionToken::Optional),
                    '+' => Some(ExpressionToken::OneOrMore),
                    '*' => Some(ExpressionToken::ZeroOrMore),
                    '/' | '|' => Some(ExpressionToken::Choice),
                    _ if c.is_whitespace() => Some(ExpressionToken::None),
                    _ => None,
                };
                if let Some(ex) = expr {
                    tokenizer.append_last(last_string);
                    last_string = String::new();
                    if ex != ExpressionToken::None {
                        tokenizer.tokens.push(ex);
                    }
                } else {
                    last_string.push(c);
                    last = c;
                    last_last = last;
                }
            }
        }

        tokenizer.append_last(last_string);
        return tokenizer;
    }

    fn append_last(&mut self, last_string: String) {
        if last_string.len() > 0 {
            if Self::is_terminal(last_string.as_str()) {
                if Self::is_regex(last_string.as_str()) {
                    self.tokens.push(ExpressionToken::TerminalRegexExpression(
                        last_string.trim().to_string()
                    ));
                } else {
                    self.tokens.push(ExpressionToken::TerminalExpression(
                        last_string[1..last_string.len() - 1].to_string()
                    ));
                }
            } else {
                self.tokens
                    .push(ExpressionToken::Expression(last_string.trim().to_string()));
            }
        }
    }

    pub fn tokens_len(&self) -> usize {
        return self.tokens.len();
    }

    pub fn next_token(&mut self) -> Option<ExpressionToken> {
        if self.current < self.tokens_len() {
            self.current += 1;
            return Some(self.tokens[self.current - 1].clone());
        }
        return None;
    }

    pub fn peek_token(&mut self) -> Option<&ExpressionToken> {
        if self.current + 1 < self.tokens.len() {
            return Some(&self.tokens[self.current + 1]);
        }
        return None;
    }

    fn is_terminal(expr: &str) -> bool {
        let first = expr.chars().nth(0).unwrap();
        if first == '\'' || first == '\"' || first == '[' {
            return true;
        }
        return false;
    }

    fn is_regex(expr: &str) -> bool {
        let first = expr.chars().nth(0).unwrap();
        return first == '[';
    }
}
