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
        assert_eq!(t.next_token().content, "Hallo");
        t.pop_state();
        assert_eq!(t.next_token().content, "Hallo");
        assert_eq!(t.next_token().content, "Welt");
        assert!(t.next_token().eof);
        t.pop_state();
        assert_eq!(t.next_token().content, "Hallo");
        assert_eq!(t.next_token().content, "Welt");
        assert!(t.next_token().eof);
    }

    #[test]
    fn eof_token() {
        let mut t = CodeTokenizer::new("");
        assert_eq!(t.tokens_len(), 0);
        assert!(t.next_token().eof);
        assert!(t.next_token().eof);
        assert!(t.next_token().eof);
        assert!(t.next_token().eof);
    }
    #[test]
    fn tokenize() {
        let mut t = CodeTokenizer::new("Hallo\n Welt ");
        assert_eq!(t.tokens_len(), 2);
        let token = t.next_token();
        assert_eq!(token.content, "Hallo");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
        let token = t.next_token();
        assert_eq!(token.content, "Welt");
        assert_eq!(token.line, 2);
        assert_eq!(token.column, 2);
    }
}
