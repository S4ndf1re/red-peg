use std::fmt;

#[derive(Clone, fmt::Debug)]
pub struct Token {
    pub line: usize,
    pub column: usize,
    pub content: String,
    pub eof: bool,
}

#[derive(fmt::Debug)]
pub struct CodeTokenizer {
    code: String,
    tokens: Vec<Token>,
    states: Vec<usize>,
    eof_token: Token,
}

impl CodeTokenizer {
    pub fn new(code: &str) -> CodeTokenizer {
        let mut tokenizer = CodeTokenizer {
            code: String::from(code),
            tokens: Vec::new(),
            states: vec![0usize],
            eof_token: Token {
                line: 0,
                column: 0,
                content: String::new(),
                eof: true,
            },
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
                        eof: false,
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
                eof: false,
            });
        }
        return tokenizer;
    }
    pub fn next_token(&mut self) -> &Token {
        let index = self.states.last_mut().expect("No state left!");
        if self.tokens.len() > *index {
            *index += 1;
            self.tokens.get(*index - 1).expect("Unable to fetch token!")
        } else {
            &self.eof_token
        }
    }

    pub fn push_state(&mut self) {
        self.states
            .push(self.states.first().expect("No current state!").clone())
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
    GroupBegin,
    GroupEnd,
    ZeroOrMore,
    OneOrMore,
    Ordering,
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
        let mut in_regex = false;
        for c in iter {
            // in_regex and c.is_whitespace must be separate, because they
            // need a tokenizer.append_last() call, while not appending any token to the
            // tokenizer other than an expression
            if in_regex {
                last_string.push(c);
                if c == ']' && last != '\\' {
                    in_regex = false;
                    tokenizer.append_last(last_string);
                    last_string = String::new();
                }
            } else {
                let expr = match c {
                    '(' => Some(ExpressionToken::GroupBegin),
                    ')' => Some(ExpressionToken::GroupEnd),
                    '[' => {
                        in_regex = true;
                        None
                    }
                    '+' => Some(ExpressionToken::OneOrMore),
                    '*' => Some(ExpressionToken::ZeroOrMore),
                    '/' => Some(ExpressionToken::Ordering),
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
                }
            }
        }

        tokenizer.append_last(last_string);
        return tokenizer;
    }

    fn append_last(&mut self, last_string: String) {
        if last_string.len() > 0 {
            self.tokens
                .push(ExpressionToken::Expression(last_string.trim().to_string()));
        }
    }

    pub fn tokens_len(&self) -> usize {
        return self.tokens.len();
    }

    pub fn next(&mut self) -> Option<ExpressionToken> {
        if self.current < self.tokens_len() {
            self.current += 1;
            return Some(self.tokens[self.current - 1].clone());
        }
        return None;
    }
}
