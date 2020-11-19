#[cfg(test)]
mod stringify {
    use red_peg::parser::*;
    #[test]
    fn stringify_choice_sequence_terminal() {
        let mut p = Parser::new();
        p.add_rule("Start", ChoiceParsingExpresion::new(vec![
            SequenceParsingExpression::new(vec![
                TerminalParsingExpression::new("A"),
                TerminalParsingExpression::new("B"),
                TerminalParsingExpression::new("C"),
            ]),
            TerminalParsingExpression::new("D"),
        ]));
        assert_eq!(format!("{}", p), "Start -> ('A' 'B' 'C' | 'D')");
        let mut p = Parser::new();
        p.add_rule("XYZ", SequenceParsingExpression::new(vec![
            ChoiceParsingExpresion::new(vec![
                TerminalParsingExpression::new("A"),
                TerminalParsingExpression::new("B"),
                TerminalParsingExpression::new("C"),
            ]),
            TerminalParsingExpression::new("D"),
        ]));
        assert_eq!(format!("{}", p), "XYZ -> ('A' | 'B' | 'C') 'D'");
    }

    #[test]
    fn stringify_non_terminal() {
        let mut p = Parser::new();
        p.add_rule("XYZ", SequenceParsingExpression::new(vec![
                ChoiceParsingExpresion::new(vec![
                    NonTerminalParsingExpression::new("A"),
                    TerminalParsingExpression::new("B"),
                    NonTerminalParsingExpression::new("C"),
                ]),
                TerminalParsingExpression::new("D"),
            ]));
        assert_eq!(format!("{}", p), "XYZ -> (A | 'B' | C) 'D'");
    }
    #[test]
    fn validate() {
        let mut parser = Parser::new();
        parser.add_rule("Start",TerminalParsingExpression::new("a"));
        assert!(parser.validate("Start", "a"));
    }
}
