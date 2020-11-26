#[cfg(test)]
mod code_tokenizer {
    use red_peg::code_tokenizer::*;
    use regex::Regex;

    #[test]
    #[should_panic]
    fn state_system_panic() {
        let mut t = CodeTokenizer::new("Hallo Welt!");
        t.push_state();
        t.pop_state();
        t.pop_state();
    }

    #[test]
    fn tokenize1() {
        let mut t = CodeTokenizer::new("Hallo Welt!");
        t.push_state();
        assert!(t.match_string("Hallo"));
        t.pop_state();
        assert!(t.match_string("Hallo"));
        assert!(t.match_string("Welt!"));
    }

    #[test]
    fn tokenize2() {
        let mut t = CodeTokenizer::new("52 Number");
        t.push_state();
        assert!(t.match_regex(&Regex::new(r"[\d]").unwrap()));
        assert!(t.match_regex(&Regex::new(r"[\d]").unwrap()));
        t.push_state();
        assert!(t.match_string("Number"));
        t.pop_state();
        assert!(t.match_string("Number"));
        t.pop_state();
        assert!(!t.match_regex(&Regex::new(r"[a-z]").unwrap()));
        assert!(t.match_regex(&Regex::new(r"[\d]").unwrap()));
        assert!(t.match_regex(&Regex::new(r"[\d]").unwrap()));
    }
}