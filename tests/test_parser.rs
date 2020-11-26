#[cfg(test)]
mod parser {
    use red_peg::parser::*;
    use red_peg::tokenizer::CodeTokenizer;

    #[test]
    fn stringify_choice_sequence_terminal() {
        let mut p: Parser<()> = Parser::new();
        p.add_rule(
            "Start",
            ChoiceParsingExpression::new(vec![
                SequenceParsingExpression::new(vec![
                    TerminalParsingExpression::new("A"),
                    TerminalParsingExpression::new("B"),
                    TerminalParsingExpression::new("C"),
                ]),
                TerminalParsingExpression::new("D"),
            ]),
            None,
        );
        assert_eq!(format!("{}", p), "Start -> ('A' 'B' 'C' | 'D')");
        let mut p: Parser<()> = Parser::new();
        p.add_rule(
            "XYZ",
            SequenceParsingExpression::new(vec![
                ChoiceParsingExpression::new(vec![
                    TerminalParsingExpression::new("A"),
                    TerminalParsingExpression::new("B"),
                    TerminalParsingExpression::new("C"),
                ]),
                TerminalParsingExpression::new("D"),
            ]),
            None,
        );
        assert_eq!(format!("{}", p), "XYZ -> ('A' | 'B' | 'C') 'D'");
    }

    #[test]
    fn stringify_non_terminal() {
        let mut p: Parser<()> = Parser::new();
        p.add_rule(
            "XYZ",
            SequenceParsingExpression::new(vec![
                ChoiceParsingExpression::new(vec![
                    NonTerminalParsingExpression::new("A"),
                    TerminalParsingExpression::new("B"),
                    NonTerminalParsingExpression::new("C"),
                ]),
                TerminalParsingExpression::new("D"),
            ]),
            None,
        );
        assert_eq!(format!("{}", p), "XYZ -> (A | 'B' | C) 'D'");
    }
    #[test]
    fn stringify_quantifiers() {
        let mut p: Parser<()> = Parser::new();
        p.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                OptionalParsingExpression::new(ChoiceParsingExpression::new(vec![
                    OneOrMoreParsingExpression::new(NonTerminalParsingExpression::new("A")),
                    TerminalParsingExpression::new("B"),
                    ZeroOrMoreParsingExpression::new(NonTerminalParsingExpression::new("C")),
                ])),
                TerminalParsingExpression::new("D"),
            ]),
            None,
        );
        assert_eq!(format!("{}", p), "Start -> (A+ | 'B' | C*)? 'D'");
    }
    #[test]
    fn validate() {
        let mut parser: Parser<()> = Parser::new();
        parser.add_rule("Start", TerminalParsingExpression::new("a"), None);
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));

        let mut parser: Parser<()> = Parser::new();
        parser.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                TerminalParsingExpression::new("a"),
                TerminalParsingExpression::new("b"),
            ]),
            None,
        );
        assert!(parser.validate("Start", "a b"));
        assert!(!parser.validate("Start", "a a"));
        assert!(!parser.validate("Start", "b a a"));

        let mut parser: Parser<()> = Parser::new();
        parser.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                ChoiceParsingExpression::new(vec![
                    TerminalParsingExpression::new("a"),
                    TerminalParsingExpression::new("b"),
                ]),
                TerminalParsingExpression::new("c"),
            ]),
            None,
        );
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(!parser.validate("Start", "a b"));

        let mut parser: Parser<()> = Parser::new();
        parser.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                ChoiceParsingExpression::new(vec![
                    TerminalParsingExpression::new("a"),
                    NonTerminalParsingExpression::new("Second"),
                ]),
                TerminalParsingExpression::new("c"),
            ]),
            None,
        );
        parser.add_rule(
            "Second",
            ChoiceParsingExpression::new(vec![
                SequenceParsingExpression::new(vec![
                    TerminalParsingExpression::new("c"),
                    TerminalParsingExpression::new("d"),
                ]),
                TerminalParsingExpression::new("b"),
            ]),
            None,
        );
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(parser.validate("Start", "c d c"));
        assert!(!parser.validate("Start", "c d b c "));
    }
    #[test]
    fn validate_quantifiers() {
        let mut parser: Parser<()> = Parser::new();
        parser.add_rule(
            "Start",
            OptionalParsingExpression::new(TerminalParsingExpression::new("a")),
            None,
        );
        assert!(parser.validate("Start", ""));
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));

        let mut parser: Parser<()> = Parser::new();
        parser.add_rule(
            "Start",
            SequenceParsingExpression::new(vec![
                OneOrMoreParsingExpression::new(TerminalParsingExpression::new("a")),
                OneOrMoreParsingExpression::new(TerminalParsingExpression::new("b")),
            ]),
            None,
        );
        assert!(parser.validate("Start", "a a a b b b"));
        assert!(parser.validate("Start", "a b b"));
        assert!(parser.validate("Start", "a a b"));
        assert!(!parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "a a b a"));

        // Start -> (a b)* | Second
        // Second -> c | d
        let mut parser: Parser<()> = Parser::new();
        parser.add_rule(
            "Start",
            ChoiceParsingExpression::new(vec![
                OneOrMoreParsingExpression::new(NonTerminalParsingExpression::new("Second")),
                ZeroOrMoreParsingExpression::new(SequenceParsingExpression::new(vec![
                    OneOrMoreParsingExpression::new(TerminalParsingExpression::new("a")),
                    OneOrMoreParsingExpression::new(TerminalParsingExpression::new("b")),
                ])),
            ]),
            None,
        );
        parser.add_rule(
            "Second",
            SequenceParsingExpression::new(vec![
                TerminalParsingExpression::new("c"),
                TerminalParsingExpression::new("d"),
            ]),
            None,
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
    fn simple_parsing() {
        let mut parser: Parser<i32> = Parser::new();
        parser.add_rule(
            "Start",
            OptionalParsingExpression::new(TerminalParsingExpression::new("a")),
            Some(Box::new(|_r, _t| {5})),
        );
        assert!(parser.validate("Start", ""));
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));
        assert_eq!(parser.parse("Start", "a").unwrap(), 5i32);

        let mut broken_calculator: Parser<i32> = Parser::new();
        broken_calculator.add_rule_str(
            "Expr",
            "Sum",
            Some(Box::new(|r: ParsingResult<i32>, _t: &CodeTokenizer| r.rule_result.unwrap())),
        );
        broken_calculator.add_rule_str(
            "Sum",
            "Product (('+' | '-') Product)*",
            Some(Box::new(|r: ParsingResult<i32>, _t: &CodeTokenizer| {
                let mut sum = r[0].rule_result.unwrap();
                for v in &r[1].sub_results {
                    let second_value = v[1].rule_result.unwrap();
                    if v[0].selected_choice.unwrap() == 0 {
                        sum += second_value;
                    } else {
                        sum -= second_value;
                    }
                }
                return sum;
            })),
        );
        broken_calculator.add_rule_str(
            "Product",
            "Value (('*' | '/') Value)*",
            Some(Box::new(|r: ParsingResult<i32>, _t: &CodeTokenizer| {
                let mut sum = r[0].rule_result.unwrap();
                for v in &r[1].sub_results {
                    let second_value = v[1].rule_result.unwrap();
                    if v[0].selected_choice.unwrap() == 0 {
                        sum *= second_value;
                    } else {
                        sum /= second_value;
                    }
                }
                return sum;
            })),
        );
        broken_calculator.add_rule_str(
            "Value",
            r"[\d]+ | ('(' Expr ')')",
            Some(Box::new(|r: ParsingResult<i32>, t: &CodeTokenizer| {
                let choice = r.selected_choice.unwrap();
                match choice {
                    0 => {
                        let digit_str = t.get_substr(r.parsed_string_start, r.parsed_string_end).trim();
                        let i : i32 = digit_str.parse().unwrap();
                        return i;
                    }
                    1 => {
                        return r[0][1].rule_result.unwrap();
                    }
                    _ => {
                        unreachable!();
                    }
                }

            })),
        );
        assert_eq!(broken_calculator.parse("Expr", "2 + 0 + 1 + 2323").unwrap(), 2326i32);
        assert_eq!(broken_calculator.parse("Expr", "2 - 5").unwrap(), -3i32);
        assert_eq!(broken_calculator.parse("Expr", "2 - 3 + 1").unwrap(), 0i32);
        assert_eq!(broken_calculator.parse("Expr", "2 * 4 - 3").unwrap(), 5i32);
        assert_eq!(broken_calculator.parse("Expr", "2 - 4 / 2").unwrap(), 0i32);
        assert_eq!(broken_calculator.parse("Expr", "(2 - 4) / 2").unwrap(), -1i32);
        assert_eq!(broken_calculator.parse("Expr", "3 * (4 - 4)").unwrap(), 0i32);
        assert_eq!(broken_calculator.parse("Expr", "3*(4-4)").unwrap(), 0i32);
        assert_eq!(broken_calculator.parse("Expr", "2*4-3").unwrap(), 5i32);
    }

    #[test]
    fn stringify_choice_sequence_terminal_from_str() {
        let mut p: Parser<()> = Parser::new();
        p.add_rule_str("Start", "'A' 'B' 'C' | 'D'", None);
        assert_eq!(format!("{}", p), "Start -> ('A' 'B' 'C' | 'D')");
        let mut p: Parser<()> = Parser::new();
        p.add_rule_str("XYZ", "(\'A\' | \'B\' | \'C\') \'D\'", None);
        assert_eq!(format!("{}", p), "XYZ -> ('A' | 'B' | 'C') 'D'");
    }

    #[test]
    fn stringify_non_terminal_from_str() {
        let mut p: Parser<()> = Parser::new();
        p.add_rule_str("XYZ", "(A | 'B' | C) 'D'", None);
        assert_eq!(format!("{}", p), "XYZ -> (A | 'B' | C) 'D'");
    }

    #[test]
    fn stringify_quantifiers_from_str() {
        let mut p: Parser<()> = Parser::new();
        p.add_rule_str("Start", "(A+ | 'B' | C*)? 'D'", None);
        assert_eq!(format!("{}", p), "Start -> (A+ | 'B' | C*)? 'D'");
    }

    #[test]
    fn validate_from_str() {
        let mut parser: Parser<()> = Parser::new();
        parser.add_rule_str("Start", "'a'", None);
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));

        let mut parser: Parser<()> = Parser::new();
        parser.add_rule_str("Start", "'a' 'b'", None);
        assert!(parser.validate("Start", "a b"));
        assert!(!parser.validate("Start", "a a"));
        assert!(!parser.validate("Start", "b a a"));

        let mut parser: Parser<()> = Parser::new();
        parser.add_rule_str("Start", "('a' | 'b') 'c'", None);
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(!parser.validate("Start", "a b"));

        let mut parser: Parser<()> = Parser::new();
        parser.add_rule_str("Start", "('a' | Second) 'c'", None);
        parser.add_rule_str("Second", "('c' 'd') | 'b'", None);
        assert!(parser.validate("Start", "a c"));
        assert!(parser.validate("Start", "b c"));
        assert!(parser.validate("Start", "c d c"));
        assert!(!parser.validate("Start", "c d b c "));
    }

    #[test]
    fn validate_quantifiers_str() {
        let mut parser: Parser<()> = Parser::new();
        parser.add_rule_str("Start", "'a'?", None);
        assert!(parser.validate("Start", ""));
        assert!(parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "b"));

        let mut parser: Parser<()> = Parser::new();
        parser.add_rule_str("Start", "'a'+ 'b'+", None);
        assert!(parser.validate("Start", "a a a b b b"));
        assert!(parser.validate("Start", "a b b"));
        assert!(parser.validate("Start", "a a b"));
        assert!(!parser.validate("Start", "a"));
        assert!(!parser.validate("Start", "a a b a"));

        let mut parser: Parser<()> = Parser::new();
        parser.add_rule_str("Start", "Second+ | ('a'+ 'b'+)*", None);
        parser.add_rule_str("Second", "'c' 'd'", None);
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
