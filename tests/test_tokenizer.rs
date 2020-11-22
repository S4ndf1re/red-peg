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

    #[test]
    fn state_system() {
        let mut t = CodeTokenizer::new("Hallo Welt");
        assert_eq!(t.tokens_len(), 2);
        t.push_state();
        t.push_state();
        assert_eq!(t.next_token().expect("No Tokens!").content, "Hallo");
        t.pop_state();
        assert_eq!(t.next_token().expect("No Tokens!").content, "Hallo");
        assert_eq!(t.next_token().expect("No Tokens!").content, "Welt");
        assert!(t.next_token().is_none());
        t.pop_state();
        assert_eq!(t.next_token().expect("No Tokens!").content, "Hallo");
        assert_eq!(t.next_token().expect("No Tokens!").content, "Welt");
        assert!(t.next_token().is_none());
    }

    #[test]
    fn eof_token() {
        let mut t = CodeTokenizer::new("");
        assert_eq!(t.tokens_len(), 0);
        assert!(t.is_empty());
        assert!(t.next_token().is_none());
        assert!(t.next_token().is_none());
        assert!(t.next_token().is_none());
        assert!(t.next_token().is_none());
    }
    #[test]
    fn tokenize() {
        let mut t = CodeTokenizer::new("Hallo\n Welt ");
        assert_eq!(t.tokens_len(), 2);
        let token = t.next_token();
        assert_eq!(token.expect("No token!").content, "Hallo");
        assert_eq!(token.expect("No token!").line, 1);
        assert_eq!(token.expect("No token!").column, 1);
        let token = t.next_token();
        assert_eq!(token.expect("No token!").content, "Welt");
        assert_eq!(token.expect("No token!").line, 2);
        assert_eq!(token.expect("No token!").column, 2);
        assert!(t.is_empty());
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
            ExpressionToken::TerminalExpression("[a-zA-Z]".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::OneOrMore);
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupBegin);
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupBegin);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("A".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::Ordering);
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
            ExpressionToken::TerminalExpression("[a-z/A-Z]".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::OneOrMore);
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupBegin);
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::GroupBegin);
        assert_eq!(
            tok.next_token().unwrap(),
            ExpressionToken::Expression("A".to_string())
        );
        assert_eq!(tok.next_token().unwrap(), ExpressionToken::Ordering);
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
