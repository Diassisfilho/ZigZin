use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use crate::tokens::Token;

/// Representation of a DFA.
/// Transitions are stored in a HashMap where the key is a tuple of a DFA state and an input symbol.
#[derive(Debug, Clone)]
pub struct DFA {
    pub transitions: HashMap<(usize, char), usize>,
    pub start: usize,
    /// Mapping from an accept state to its label.
    pub accept: HashMap<usize, String>,
}

#[derive(Debug, Deserialize)]
struct DfaTransitionRecord {
    From: usize,
    Input: String,
    To: usize,
}

/// Reads a CSV file with DFA transitions and returns a HashMap of transitions.
/// The CSV file is expected to have headers: "From,Input,To". The Input field should contain a single character.
pub fn read_transitions_from_csv(file_path: &str) -> Result<HashMap<(usize, char), usize>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut transitions: HashMap<(usize, char), usize> = HashMap::new();

    for result in csv_reader.deserialize() {
        let record: DfaTransitionRecord = result?;
        let mut chars = record.Input.chars();
        let ch = chars.next().ok_or("Empty input field")?;
        if chars.next().is_some() {
            return Err("Expected a single character for input field".into());
        }
        transitions.insert((record.From, ch), record.To);
    }
    
    Ok(transitions)
}

/// Reads a JSON file containing DFA final states and returns a HashMap mapping
/// accept states (usize) to their label (String).
///
/// The JSON file is expected to be formatted as an array of arrays, e.g.:
/// [
///   [0, "Initial"],
///   [1, "double quotes"],
///   ...
/// ]
pub fn read_accept_states_from_json(file_path: &str) -> Result<HashMap<usize, String>, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let records: Vec<(usize, String)> = serde_json::from_str(&content)?;
    let mut accept_states = HashMap::new();
    for (state, label) in records {
        accept_states.insert(state, label);
    }
    Ok(accept_states)
}

/// Helper function that computes the line and column number for a given index in the input.
fn compute_line_and_column(input: &[char], index: usize) -> (usize, usize) {
    let mut line: usize = 1;
    let mut column: usize = 1;
    for &ch in &input[0..index] {
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

/// Processes the input string, scanning it using the provided DFA and returning tokens.
/// If an invalid transition is encountered, the function panics with the line and column of the error.
pub fn process_input(dfa: &DFA, input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let input_chars: Vec<char> = input.chars().collect();
    let len = input_chars.len();
    
    let mut i = 0;
    while i < len {
        // Skip whitespace characters.
        if input_chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Start at the DFA's start state.
        let mut current_state = dfa.start;
        // Track the last encountered accepting state and its index.
        let mut last_accept_state: Option<usize> = None;
        let mut last_accept_index = i;
        let mut j = i;
        
        while j < len {
            let ch = input_chars[j];
            if let Some(&next_state) = dfa.transitions.get(&(current_state, ch)) {
                current_state = next_state;
                // Record the last accepting state's index.
                if dfa.accept.contains_key(&current_state) {
                    last_accept_state = Some(current_state);
                    last_accept_index = j + 1;
                }
                j += 1;
            } else {
                break;
            }
        }
        
        if let Some(state) = last_accept_state {
            let lexeme: String = input_chars[i..last_accept_index].iter().collect();
            let token_label = dfa.accept.get(&state).unwrap().clone();
            tokens.push(Token::new(token_label, lexeme));
            i = last_accept_index;
        } else {
            // When no valid transition exists, compute line and column and panic with an error message.
            let (line, column) = compute_line_and_column(&input_chars, i);
            panic!(
                "ZigZin compiler: Lexer error at line {}, column {}: Unexpected token '{}'",
                line, column, input_chars[i]
            );
        }
    }
    tokens
}

pub fn process_file_input(dfa: &DFA, file_path: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(process_input(dfa, content.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input_accepted() {
        // let mut transitions = HashMap::new();
        // transitions.insert((0, 'a'), 1);
        // let start = 0;
        // let mut accept = HashMap::new();
        // accept.insert(1, "accepted".to_string());
    
        // let dfa = DFA {
        //     transitions,
        //     start,
        //     accept,
        // };
    
        // let (result, state, label) = process_input(&dfa, "a".to_string());
        // assert_eq!(result, true);
        // assert_eq!(state, 1);
        // assert_eq!(label, "accepted".to_string());
    }
}