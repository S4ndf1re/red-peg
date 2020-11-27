use regex::Regex;
use std::fmt;

#[derive(fmt::Debug)]
pub struct CodeTokenizer {
    code: String,
    states: Vec<usize>,
}

impl CodeTokenizer {
    pub fn new(code: &str) -> CodeTokenizer {
        return CodeTokenizer {
            code: String::from(code),
            states: vec![0],
        };
    }
    pub fn is_empty(&self) -> bool {
        let index = *self.states.last().expect("No state left!");
        self.code.len() <= index
    }

    pub fn match_string(&mut self, string: &str) -> bool {
        if self.code.len() < self.get_state() + string.len() {
            return false;
        }
        self.skip_whitespaces();
        if &self.code[self.get_state()..(self.get_state() + string.len())] == string {
            *self.states.last_mut().unwrap() += string.len();
            self.skip_whitespaces();
            return true;
        } else {
            return false;
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Some(ch) = self.code.chars().nth(self.get_state()) {
            if ch.is_whitespace() {
                *self.states.last_mut().unwrap() += 1;
            } else {
                break;
            }
        }
    }

    pub fn match_regex(&mut self, regex: &Regex) -> bool {
        self.skip_whitespaces();
        match regex.find_at(self.code.as_str(), self.get_state()) {
            Some(res) => {
                if self.get_state() != res.start() {
                    false
                } else {
                    *self.states.last_mut().unwrap() += res.range().len();
                    self.skip_whitespaces();
                    true
                }
            }
            None => false,
        }
    }

    pub fn push_state(&mut self) -> usize {
        self.states
            .push(self.states.last().expect("No current state!").clone());
        *self.states.last().unwrap()
    }
    /* Takes the topmost value in the stack, saves it, pops it off the stack
     * and writes it to the new top-entry.
     */
    pub fn update_state(&mut self) -> usize {
        let current_state = *self.states.last().expect("Nu current state!");
        self.states.pop();
        let state_below = self
            .states
            .last_mut()
            .expect("No state below the current one!");
        *state_below = current_state;
        current_state
    }
    pub fn get_state(&self) -> usize {
        *self.states.last().expect("No state left!")
    }

    pub fn only_one_state_left(&self) -> bool {
        self.states.len() == 1
    }

    pub fn pop_state(&mut self) {
        self.states.pop().expect("No state left to pop!");
        if self.states.is_empty() {
            panic!("You can't pop the last state!");
        }
    }

    pub fn get_substr(&self, start: usize, end: usize) -> &str {
        &self.code[start..end]
    }
}
