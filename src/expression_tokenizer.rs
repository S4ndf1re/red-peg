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
    AndPredicate,
    NotPredicate,
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
                    '!' => Some(ExpressionToken::NotPredicate),
                    '&' => Some(ExpressionToken::AndPredicate),
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
                        last_string.trim().to_string(),
                    ));
                } else {
                    self.tokens.push(ExpressionToken::TerminalExpression(
                        last_string[1..last_string.len() - 1].to_string(),
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
