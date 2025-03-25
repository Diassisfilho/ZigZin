use std::collections::{HashMap, HashSet, VecDeque, BTreeSet};
use std::error::Error;

/// Representation of an NFA.
/// Transitions are stored in a HashMap where the key is a tuple of a state and an optional input symbol.
/// An input symbol of None represents an ε (epsilon) transition.
#[derive(Debug, Clone)]
pub struct NFA {
    pub transitions: HashMap<(usize, Option<char>), Vec<usize>>,
    pub start: usize,
    pub accept: HashSet<usize>,
}

/// Representation of a DFA.
/// Transitions are stored in a HashMap where the key is a tuple of a DFA state and an input symbol.
#[derive(Debug, Clone)]
pub struct DFA {
    pub transitions: HashMap<(usize, char), usize>,
    pub start: usize,
    pub accept: HashSet<usize>,
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
pub fn convert_nfa_to_dfa(nfa: &NFA, alphabet: &HashSet<char>) -> DFA {
    let mut dfa_transitions = HashMap::new();
    let mut dfa_accept = HashSet::new();
    
    // Mapping from a set of NFA states (as a DFA state) to its index.
    let mut state_mapping: HashMap<BTreeSet<usize>, usize> = HashMap::new();
    let mut dfa_states: Vec<BTreeSet<usize>> = Vec::new();
    
    // Start with the epsilon closure of the NFA's start state.
    let mut start_set = BTreeSet::new();
    start_set.insert(nfa.start);
    let start_set_hash: HashSet<usize> = start_set.into_iter().collect();
    let start_closure: BTreeSet<usize> = epsilon_closure(nfa, &start_set_hash).into_iter().collect();
    
    // Assign index 0 to the start closure.
    state_mapping.insert(start_closure.clone(), 0);
    dfa_states.push(start_closure.clone());
    
    // Mark the DFA state as accepting if any NFA state in the set is accepting.
    if !start_closure.is_disjoint(&nfa.accept.iter().cloned().collect::<BTreeSet<_>>()) {
        dfa_accept.insert(0);
    }
    
    let mut queue = VecDeque::new();
    queue.push_back(0);
    
    while let Some(current_index) = queue.pop_front() {
        let current_state_set = dfa_states[current_index].clone();
        
        // Process each symbol in the alphabet.
        for &symbol in alphabet {
            // Compute the move and then the epsilon closure.
            let move_set = move_nfa(nfa, &current_state_set.iter().cloned().collect::<HashSet<_>>(), symbol);
            if move_set.is_empty() {
                continue;
            }
            let next_closure: BTreeSet<usize> = epsilon_closure(nfa, &move_set).into_iter().collect();
            
            // Check if this state set has already been encountered.
            let next_index = if let Some(&index) = state_mapping.get(&next_closure) {
                index
            } else {
                let new_index = dfa_states.len();
                state_mapping.insert(next_closure.clone(), new_index);
                dfa_states.push(next_closure.clone());
                queue.push_back(new_index);
                
                // If any state in the closure is an accepting state in the NFA, mark it as accepting.
                if !next_closure.is_disjoint(&nfa.accept.iter().cloned().collect::<BTreeSet<_>>()) {
                    dfa_accept.insert(new_index);
                }
                new_index
            };
            
            // Record the DFA transition.
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

pub fn read_nfa_convert_to_dfa(path: &str) -> Option<(DFA,NFA)> {
    // Read NFA transitions from CSV file.
    let transitions = match read_nfa_transitions_csv(path) {
        Ok(t) => t,
        Err(_) => return None,
    };

    // Define the NFA.
    // For demonstration, we assume the start state is 0.
    // Adjust the accept states as needed. Here we assume state 14 is accepting.
    let accept_states = [
        5, 6, 7, 10, 12, 13, 14, 18, 19, 20, 21, 22, 23, 24, 30, 31, 32, 33, 34,
    ]
    .iter()
    .cloned()
    .collect();

    let nfa = NFA {
        transitions,
        start: 0,
        accept: accept_states,
    };

    // Build the alphabet for the NFA: gather all symbols from transitions (ignoring None).
    let mut alphabet = HashSet::new();
    for (&(_, symbol), _) in &nfa.transitions {
        if let Some(ch) = symbol {
            alphabet.insert(ch);
        }
    }

    // Convert the NFA to a DFA.
    let dfa = convert_nfa_to_dfa(&nfa, &alphabet);

    return Some((dfa,nfa));
}