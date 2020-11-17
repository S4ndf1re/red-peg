#[cfg(test)]
mod stringify {
    use red_peg::parser::*;
    #[test]
    fn stringify() {
        {
            let rule = ParseRule {
                left_side: String::from("Start"),
                right_side: ChoiceParsingExpresion::new(vec![
                    SequenceParsingExpression::new(vec![
                        TerminalParsingExpression::new("A"),
                        TerminalParsingExpression::new("B"),
                        TerminalParsingExpression::new("C"),
                    ]),
                    TerminalParsingExpression::new("D"),
                ]),
            };
            assert_eq!(format!("{}", rule), "Start -> ('A' 'B' 'C' | 'D')");
        }
        {
            let rule = ParseRule {
                left_side: String::from("XYZ"),
                right_side: SequenceParsingExpression::new(vec![
                    ChoiceParsingExpresion::new(vec![
                        TerminalParsingExpression::new("A"),
                        TerminalParsingExpression::new("B"),
                        TerminalParsingExpression::new("C"),
                    ]),
                    TerminalParsingExpression::new("D"),
                ]),
            };
            assert_eq!(format!("{}", rule), "XYZ -> ('A' | 'B' | 'C') 'D'");
        }
    }
}