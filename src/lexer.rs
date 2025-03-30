use std::collections::{HashMap, HashSet, VecDeque, BTreeSet};
use std::error::Error;
use std::fs;
use std::fs::File;
use serde::Deserialize;

#[derive(Deserialize)]
struct ZigZinStatesTypes {
    initial: Vec<usize>,
    #[serde(rename = "final")]
    finals: Vec<(usize, String)>,
}

/// Reads the ZigZin-NFA-states-types.json file and converts it into an NFA accept structure.
/// Returns a HashMap where each key is an accept state and the value is its label.
pub fn read_zigzin_states_types(file_path: &str) -> Result<HashMap<usize, String>, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let states: ZigZinStatesTypes = serde_json::from_str(&content)?;
    let mut accept = HashMap::new();
    for (state, label) in states.finals {
        accept.insert(state, label);
    }
    Ok(accept)
}

/// Representation of an NFA.
/// Transitions are stored in a HashMap where the key is a tuple of a state and an optional input symbol.
/// An input symbol of None represents an ε (epsilon) transition.
#[derive(Debug, Clone)]
pub struct NFA {
    pub transitions: HashMap<(usize, Option<char>), Vec<usize>>,
    pub start: usize,
    /// Mapping from an accept state to its label.
    pub accept: HashMap<usize, String>,
}

/// Representation of a DFA.
/// Transitions are stored in a HashMap where the key is a tuple of a DFA state and an input symbol.
#[derive(Debug, Clone)]
pub struct DFA {
    pub transitions: HashMap<(usize, char), usize>,
    pub start: usize,
    /// Mapping from an accept state to its label.
    pub accept: HashMap<usize, String>,
}

/// Computes the epsilon closure of a set of NFA states.
pub fn epsilon_closure(nfa: &NFA, states: &HashSet<usize>) -> HashSet<usize> {
    let mut closure = states.clone();
    let mut stack: Vec<usize> = states.iter().cloned().collect();

    while let Some(state) = stack.pop() {
        if let Some(next_states) = nfa.transitions.get(&(state, None)) {
            for &next in next_states {
                if closure.insert(next) {
                    stack.push(next);
                }
            }
        }
    }
    closure
}

/// Given a set of NFA states and a symbol, returns the set of states reachable by that symbol.
pub fn move_nfa(nfa: &NFA, states: &HashSet<usize>, symbol: char) -> HashSet<usize> {
    let mut result = HashSet::new();

    for &state in states {
        if let Some(next_states) = nfa.transitions.get(&(state, Some(symbol))) {
            for &next in next_states {
                result.insert(next);
            }
        }
    }
    result
}

/// Converts an NFA to a DFA using the subset construction algorithm.
/// `alphabet` is the set of input symbols (excluding ε).
pub fn convert_nfa_to_dfa(nfa: &NFA, alphabet: &std::collections::HashSet<char>) -> DFA {
    use std::collections::BTreeSet;
    let mut dfa_transitions = std::collections::HashMap::new();
    let mut dfa_accept: std::collections::HashMap<usize, String> = std::collections::HashMap::new();

    let mut state_mapping: std::collections::HashMap<BTreeSet<usize>, usize> = std::collections::HashMap::new();
    let mut dfa_states: Vec<BTreeSet<usize>> = Vec::new();

    let mut start_set = BTreeSet::new();
    start_set.insert(nfa.start);
    let start_set_hash: std::collections::HashSet<usize> = start_set.iter().cloned().collect();
    let start_closure: BTreeSet<usize> = epsilon_closure(nfa, &start_set_hash).into_iter().collect();

    state_mapping.insert(start_closure.clone(), 0);
    dfa_states.push(start_closure.clone());

    // Convert the keys of nfa.accept to a BTreeSet for comparison.
    let nfa_accept_btree: BTreeSet<usize> = nfa.accept.keys().cloned().collect();
    
    if !start_closure.is_disjoint(&nfa_accept_btree) {
        let labels: Vec<String> = start_closure
            .iter()
            .filter_map(|s| nfa.accept.get(s))
            .cloned()
            .collect();
        dfa_accept.insert(0, labels.join(", "));
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);

    while let Some(current_index) = queue.pop_front() {
        let current_state_set = dfa_states[current_index].clone();
        for &symbol in alphabet {
            let move_set = move_nfa(nfa, &current_state_set.iter().cloned().collect::<std::collections::HashSet<_>>(), symbol);
            if move_set.is_empty() {
                continue;
            }
            let next_closure: BTreeSet<usize> = epsilon_closure(nfa, &move_set).into_iter().collect();

            let next_index = if let Some(&index) = state_mapping.get(&next_closure) {
                index
            } else {
                let new_index = dfa_states.len();
                state_mapping.insert(next_closure.clone(), new_index);
                dfa_states.push(next_closure.clone());
                queue.push_back(new_index);
                if !next_closure.is_disjoint(&nfa_accept_btree) {
                    let labels: Vec<String> = next_closure
                        .iter()
                        .filter_map(|s| nfa.accept.get(s))
                        .cloned()
                        .collect();
                    dfa_accept.insert(new_index, labels.join(", "));
                }
                new_index
            };
            dfa_transitions.insert((current_index, symbol), next_index);
        }
    }

    DFA {
        transitions: dfa_transitions,
        start: 0,
        accept: dfa_accept,
    }
}

/// Reads a CSV file of NFA transitions using the `csv` crate and returns a HashMap.
/// The CSV is expected to have three columns: From,Input,To.
/// Lines starting with a comment (e.g., "//") will be skipped.
/// An input symbol of an empty string is interpreted as None.
pub fn read_nfa_transitions_csv(file_path: &str) -> Result<HashMap<(usize, Option<char>), Vec<usize>>, Box<dyn Error>> {
    // Configure the reader to ignore lines starting with '/'
    let mut rdr = csv::ReaderBuilder::new()
        .comment(Some(b'/'))
        .from_path(file_path)?;
    
    let mut transitions = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        // Expecting three columns: from, input, to.
        if record.len() != 3 {
            continue;
        }

        let from: usize = record.get(0).unwrap().trim().parse()?;
        let input_str = record.get(1).unwrap().trim();
        let symbol = if input_str.is_empty() {
            None
        } else {
            Some(input_str.chars().next().unwrap())
        };
        let to: usize = record.get(2).unwrap().trim().parse()?;

        transitions.entry((from, symbol))
            .or_insert_with(Vec::new)
            .push(to);
    }

    Ok(transitions)
}

// pub fn read_nfa_convert_to_dfa(path: &str) -> Option<(DFA,NFA)> {
//     // Read NFA transitions from CSV file.
//     let transitions = match read_nfa_transitions_csv(path) {
//         Ok(t) => t,
//         Err(_) => return None,
//     };

//     // Define the NFA.
//     // For demonstration, we assume the start state is 0.
//     // Adjust the accept states as needed. Here we assume state 14 is accepting.
//     let accept_states = read_zigzin_states_types("automato/Zigzin-NFA-states-types.json")?;

//     let nfa = NFA {
//         transitions,
//         start: 0,
//         accept: accept_states,
//     };

//     // Build the alphabet for the NFA: gather all symbols from transitions (ignoring None).
//     let mut alphabet = HashSet::new();
//     for (&(_, symbol), _) in &nfa.transitions {
//         if let Some(ch) = symbol {
//             alphabet.insert(ch);
//         }
//     }

//     // Convert the NFA to a DFA.
//     let dfa = convert_nfa_to_dfa(&nfa, &alphabet);

//     return Some((dfa,nfa));
// }

pub fn process_file_input(dfa: &DFA, file_path: &str) -> Result<(bool, usize), Box<dyn Error>> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(process_input(dfa, content))
}

/// Processes an input file using the provided DFA.
/// It returns a tuple:
///   - A boolean indicating if the input ended in an accept state (true for accepted).
///   - The state number at which processing stopped.
/// If a transition for a character is missing, processing stops and the function returns false along with the last valid state.
pub fn process_input(dfa: &DFA, input: String) -> (bool, usize) {
    let mut current_state = dfa.start;

    for ch in input.chars() {
        if let Some(&next_state) = dfa.transitions.get(&(current_state, ch)) {
            current_state = next_state;
        } else {
            return (false, current_state);
        }
    }

    let accepted = dfa.accept.contains_key(&current_state);
    (accepted, current_state)
}

/// Writes the DFA transitions to a CSV file.
/// The file will have a header row "From,Input,To".
pub fn write_dfa_to_csv(dfa: &DFA, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut wtr = csv::Writer::from_writer(file);

    // Write header
    wtr.write_record(&["From", "Input", "To"])?;

    // Write each transition.
    for ((from, input), tos) in &dfa.transitions {
        // Convert the transition values to string
        let record = [from.to_string(), input.to_string(), tos.to_string()];
        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
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
    
        let (result, state) = process_input(&dfa, "a".to_string());
        assert_eq!(result, true);
        assert_eq!(state, 1);
    }
}