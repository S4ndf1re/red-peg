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
    fn validate() {
        let mut parser = Parser::new();
        parser.add_rule("Start", TerminalParsingExpression::new("a"));
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));


        let mut parser = Parser::new();
        parser.add_rule("Start", SequenceParsingExpression::new(vec![
            TerminalParsingExpression::new("a"),
            TerminalParsingExpression::new("b")
        ]));
        assert!(parser.validate("Start", "a b"));
        assert!(!parser.validate("Start", "a a"));
        assert!(!parser.validate("Start", "b a a"));


        let mut parser = Parser::new();
        parser.add_rule("Start", SequenceParsingExpression::new(vec![
            ChoiceParsingExpresion::new(vec![
                TerminalParsingExpression::new("a"),
                TerminalParsingExpression::new("b")
            ]),
            TerminalParsingExpression::new("c")
        ]));
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(!parser.validate("Start", "a b"));

        let mut parser = Parser::new();
        parser.add_rule("Start", SequenceParsingExpression::new(vec![
            ChoiceParsingExpresion::new(vec![
                TerminalParsingExpression::new("a"),
                NonTerminalParsingExpression::new("Second"),
            ]),
            TerminalParsingExpression::new("c")
        ]));
        parser.add_rule("Second",
            ChoiceParsingExpresion::new(vec![
                SequenceParsingExpression::new(vec![
                    TerminalParsingExpression::new("c"),
                    TerminalParsingExpression::new("d")
                ]),
                TerminalParsingExpression::new("b")
            ]));
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(parser.validate("Start", "c d c"));
        assert!(!parser.validate("Start", "c d b c "));
    }
}
