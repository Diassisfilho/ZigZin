use std::collections::{HashMap,HashSet};
use std::error::Error;

mod lexer;
use lexer::{
    read_zigzin_states_types,convert_nfa_to_dfa, process_file_input, read_nfa_transitions_csv, write_dfa_to_csv, DFA, NFA,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Read NFA transitions from CSV file.
    let transitions = read_nfa_transitions_csv("automato/all-Zigzin-NFA-transitions.csv")?;

    // Create a HashSet of accept states [1,2,3,4,5,6,7,8,9,10,11].
    let accept_states: HashMap<usize, String> = read_zigzin_states_types("automato/Zigzin-NFA-states-types.json")?;

    // Define the NFA (assuming start state is 0).
    let nfa = NFA {
        transitions,
        start: 0,
        accept: accept_states.clone(),
    };

    // Create the alphabet for the NFA.
    let mut alphabet = HashSet::new();
    for (&(_, symbol), _) in &nfa.transitions {
        if let Some(ch) = symbol {
            alphabet.insert(ch);
        }
    }

    // Convert the NFA to a DFA.
    let dfa = convert_nfa_to_dfa(&nfa, &alphabet);

    // write_dfa_to_csv(&dfa, "dfa_transitions.csv")?;
    // println!("DFA transitions written to dfa_transitions.csv");

    println!("dfa acc: {:?}", dfa.accept);
    // println!("nfa acc: {:?}", nfa.accept);

    // Process input from a file using the DFA.
    let (accepted, final_state) = process_file_input(&dfa, "lexer_first_test.txt")?;
    if accepted {
        println!("Input accepted. Final state: {}", final_state);
    } else {
        println!(
            "Input rejected or error encountered. Last state: {}",
            final_state
        );
    }

    Ok(())
}
