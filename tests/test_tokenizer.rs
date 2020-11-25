#[cfg(test)]
mod stringify {
    use red_peg::tokenizer::*;
    #[test]
    #[should_panic]
    fn state_system_panic() {
        let mut t = CodeTokenizer::new("Hallo Welt!");
        t.push_state();
        t.pop_state();
        t.pop_state();
    }
}

#[cfg(test)]
mod expr_tokenizing {
    use red_peg::tokenizer::*;
    #[test]
    fn tokenize() {
        let mut tok = ExpressionTokenizer::new("Test ABC [a-zA-Z]+ ((A/B) C)");
        assert_eq!(tok.tokens_len(), 12);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("Test".to_string())
        );
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("ABC".to_string())
        );
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::TerminalRegexExpression("[a-zA-Z]".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::OneOrMore);
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupBegin);
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupBegin);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("A".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::Choice);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("B".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupEnd);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("C".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupEnd);
    }

    #[test]
    fn tokenize2() {
        let mut tok = ExpressionTokenizer::new("Test ABC [a-z/A-Z]+ ((A/B) C)");
        assert_eq!(tok.tokens_len(), 12);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("Test".to_string())
        );
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("ABC".to_string())
        );
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::TerminalRegexExpression("[a-z/A-Z]".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::OneOrMore);
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupBegin);
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupBegin);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("A".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::Choice);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("B".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupEnd);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("C".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupEnd);
    }
}
