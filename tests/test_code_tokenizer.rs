#[cfg(test)]
mod code_tokenizer {
    use red_peg::code_tokenizer::*;
    #[test]
    #[should_panic]
    fn state_system_panic() {
        let mut t = CodeTokenizer::new("Hallo Welt!");
        t.push_state();
        t.pop_state();
        t.pop_state();
    }
}