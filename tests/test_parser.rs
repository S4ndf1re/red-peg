#[cfg(test)]
mod parser {
    use red_peg::parser::*;
    #[test]
    fn stringify_choice_sequence_terminal() {
        let mut p = Parser::new();
        p.add_rule(
            "Start",
            ChoiceParsingExpresion::new(vec![
                SequenceParsingExpression::new(vec![
                    TerminalParsingExpression::new("A"),
                    TerminalParsingExpression::new("B"),
                    TerminalParsingExpression::new("C"),
                ]),
                TerminalParsingExpression::new("D"),
            ]),
        );
        assert_eq!(format!("{}", p), "Start -> ('A' 'B' 'C' | 'D')");
        let mut p = Parser::new();
        p.add_rule(
            "XYZ",
            SequenceParsingExpression::new(vec![
                ChoiceParsingExpresion::new(vec![
                    TerminalParsingExpression::new("A"),
                    TerminalParsingExpression::new("B"),
                    TerminalParsingExpression::new("C"),
                ]),
                TerminalParsingExpression::new("D"),
            ]),
        );
        assert_eq!(format!("{}", p), "XYZ -> ('A' | 'B' | 'C') 'D'");
    }

    #[test]
    fn stringify_non_terminal() {
        let mut p = Parser::new();
        p.add_rule(
            "XYZ",
            SequenceParsingExpression::new(vec![
                ChoiceParsingExpresion::new(vec![
                    NonTerminalParsingExpression::new("A"),
                    TerminalParsingExpression::new("B"),
                    NonTerminalParsingExpression::new("C"),
                ]),
                TerminalParsingExpression::new("D"),
            ]),
        );
        assert_eq!(format!("{}", p), "XYZ -> (A | 'B' | C) 'D'");
    }
    #[test]
    fn stringify_quantifiers() {
        let mut p = Parser::new();
        p.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                OptionalParsingExpression::new(ChoiceParsingExpresion::new(vec![
                    OneOrMoreParsingExpression::new(NonTerminalParsingExpression::new("A")),
                    TerminalParsingExpression::new("B"),
                    ZeroOrMoreParsingExpression::new(NonTerminalParsingExpression::new("C")),
                ])),
                TerminalParsingExpression::new("D"),
            ]),
        );
        assert_eq!(format!("{}", p), "Start -> (A+ | 'B' | C*)? 'D'");
    }
    #[test]
    fn validate() {
        let mut parser = Parser::new();
        parser.add_rule("Start", TerminalParsingExpression::new("a"));
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));

        let mut parser = Parser::new();
        parser.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                TerminalParsingExpression::new("a"),
                TerminalParsingExpression::new("b"),
            ]),
        );
        assert!(parser.validate("Start", "a b"));
        assert!(!parser.validate("Start", "a a"));
        assert!(!parser.validate("Start", "b a a"));

        let mut parser = Parser::new();
        parser.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                ChoiceParsingExpresion::new(vec![
                    TerminalParsingExpression::new("a"),
                    TerminalParsingExpression::new("b"),
                ]),
                TerminalParsingExpression::new("c"),
            ]),
        );
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(!parser.validate("Start", "a b"));

        let mut parser = Parser::new();
        parser.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                ChoiceParsingExpresion::new(vec![
                    TerminalParsingExpression::new("a"),
                    NonTerminalParsingExpression::new("Second"),
                ]),
                TerminalParsingExpression::new("c"),
            ]),
        );
        parser.add_rule(
            "Second",
            ChoiceParsingExpresion::new(vec![
                SequenceParsingExpression::new(vec![
                    TerminalParsingExpression::new("c"),
                    TerminalParsingExpression::new("d"),
                ]),
                TerminalParsingExpression::new("b"),
            ]),
        );
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(parser.validate("Start", "c d c"));
        assert!(!parser.validate("Start", "c d b c "));
    }
    #[test]
    fn validate_quantifiers() {
        let mut parser = Parser::new();
        parser.add_rule(
            "Start",
            OptionalParsingExpression::new(TerminalParsingExpression::new("a")),
        );
        assert!(parser.validate("Start", ""));
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));

        let mut parser = Parser::new();
        parser.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                OneOrMoreParsingExpression::new(TerminalParsingExpression::new("a")),
                OneOrMoreParsingExpression::new(TerminalParsingExpression::new("b")),
            ]),
        );
        assert!(parser.validate("Start", "a a a b b b"));
        assert!(parser.validate("Start", "a b b"));
        assert!(parser.validate("Start", "a a b"));
        assert!(!parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "a a b a"));

        // Start -> (a b)* | Second
        // Second -> c | d
        let mut parser = Parser::new();
        parser.add_rule(
            "Start",
            ChoiceParsingExpresion::new(vec![
                OneOrMoreParsingExpression::new(NonTerminalParsingExpression::new("Second")),
                ZeroOrMoreParsingExpression::new(SequenceParsingExpression::new(vec![
                    OneOrMoreParsingExpression::new(TerminalParsingExpression::new("a")),
                    OneOrMoreParsingExpression::new(TerminalParsingExpression::new("b")),
                ])),
            ]),
        );
        parser.add_rule(
            "Second",
            SequenceParsingExpression::new(vec![
                TerminalParsingExpression::new("c"),
                TerminalParsingExpression::new("d"),
            ]),
        );
        assert!(parser.validate("Start", "a a a b b b"));
        assert!(parser.validate("Start", "a b b"));
        assert!(parser.validate("Start", "a a b"));
        assert!(!parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "a a b a"));
        assert!(parser.validate("Start", "a a b a a a b b b a a a b"));
        assert!(!parser.validate("Start", "a a b a a a b b b a a a"));
        assert!(parser.validate("Start", "c d c d"));
        assert!(!parser.validate("Start", "c d c"));
    }

    #[test]
    fn stringify_choice_sequence_terinal_from_str() {
        let mut p = Parser::new();
        p.add_rule_str("Start", "'A' 'B' 'C' | 'D'");
        assert_eq!(format!("{}", p), "Start -> ('A' 'B' 'C' | 'D')");
        let mut p = Parser::new();
        p.add_rule_str("XYZ", "(\'A\' | \'B\' | \'C\') \'D\'");
        assert_eq!(format!("{}", p), "XYZ -> ('A' | 'B' | 'C') 'D'");
    }

    #[test]
    fn stringify_non_terminal_from_str() {
        let mut p = Parser::new();
        p.add_rule_str("XYZ", "(A | 'B' | C) 'D'");
        assert_eq!(format!("{}", p), "XYZ -> (A | 'B' | C) 'D'");
    }

    #[test]
    fn stringify_quantifiers_from_str() {
        let mut p = Parser::new();
        p.add_rule_str("Start", "(A+ | 'B' | C*)? 'D'");
        assert_eq!(format!("{}", p), "Start -> (A+ | 'B' | C*)? 'D'");
    }

    #[test]
    fn validate_from_str() {
        let mut parser = Parser::new();
        parser.add_rule_str("Start", "'a'");
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));

        let mut parser = Parser::new();
        parser.add_rule_str("Start", "'a' 'b'");
        assert!(parser.validate("Start", "a b"));
        assert!(!parser.validate("Start", "a a"));
        assert!(!parser.validate("Start", "b a a"));

        let mut parser = Parser::new();
        parser.add_rule_str("Start", "('a' | 'b') 'c'");
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(!parser.validate("Start", "a b"));

        let mut parser = Parser::new();
        parser.add_rule_str("Start", "('a' | Second) 'c'");
        parser.add_rule_str("Second", "('c' 'd') | 'b'");
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(parser.validate("Start", "c d c"));
        assert!(!parser.validate("Start", "c d b c "));
    }

    #[test]
    fn validate_quantifiers_str() {
        let mut parser = Parser::new();
        parser.add_rule_str("Start", "'a'?");
        assert!(parser.validate("Start", ""));
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));

        let mut parser = Parser::new();
        parser.add_rule_str("Start", "'a'+ 'b'+");
        assert!(parser.validate("Start", "a a a b b b"));
        assert!(parser.validate("Start", "a b b"));
        assert!(parser.validate("Start", "a a b"));
        assert!(!parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "a a b a"));

        // Start -> (a b)* | Second
        // Second -> c | d
        let mut parser = Parser::new();
        parser.add_rule_str("Start", "Second+ | ('a'+ 'b'+)*");
        parser.add_rule_str("Second", "'c' 'd'");
        assert!(parser.validate("Start", "a a a b b b"));
        assert!(parser.validate("Start", "a b b"));
        assert!(parser.validate("Start", "a a b"));
        assert!(!parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "a a b a"));
        assert!(parser.validate("Start", "a a b a a a b b b a a a b"));
        assert!(!parser.validate("Start", "a a b a a a b b b a a a"));
        assert!(parser.validate("Start", "c d c d"));
        assert!(!parser.validate("Start", "c d c"));
    }
}
