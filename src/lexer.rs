use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
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

/// Processes an input file using the provided DFA.
/// It returns a tuple:
///   - A boolean indicating if the input ended in an accept state (true for accepted).
///   - The state number at which processing stopped.
/// If a transition for a character is missing, processing stops and the function returns false along with the last valid state.
pub fn process_input(dfa: &DFA, input: String) -> (bool, usize, String) {
    let mut current_state = dfa.start;

    for ch in input.chars() {
        if let Some(&next_state) = dfa.transitions.get(&(current_state, ch)) {
            current_state = next_state;
        } else {
            return (false, current_state, "".to_string());
        }
    }
    
    let accepted = dfa.accept.contains_key(&current_state);
    let (_,label) = dfa.accept.get_key_value(&current_state).expect(&"");
    (accepted, current_state, label.to_string())
}

pub fn process_file_input(dfa: &DFA, file_path: &str) -> Result<(bool, usize, String), Box<dyn Error>> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(process_input(dfa, content))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input_accepted() {
        let mut transitions = HashMap::new();
        transitions.insert((0, 'a'), 1);
        let start = 0;
        let mut accept = HashMap::new();
        accept.insert(1, "accepted".to_string());
    
        let dfa = DFA {
            transitions,
            start,
            accept,
        };
    
        let (result, state, label) = process_input(&dfa, "a".to_string());
        assert_eq!(result, true);
        assert_eq!(state, 1);
        assert_eq!(label, "accepted".to_string());
    }
}