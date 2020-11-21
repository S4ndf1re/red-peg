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
    pub fn is_empty(&self) -> bool {
        let index = self.states.last().expect("No state left!");
        self.tokens.len() <= *index
    }

    pub fn push_state(&mut self) {
        self.states
            .push(self.states.last().expect("No current state!").clone())
    }
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
