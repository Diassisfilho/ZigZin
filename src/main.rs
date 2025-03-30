use std::collections::HashMap;
use std::error::Error;

mod lexer;
use lexer::{
    process_file_input, DFA, read_accept_states_from_json, read_transitions_from_csv
};

fn main() -> Result<(), Box<dyn Error>> {
    // Read NFA transitions from CSV file.
    let transitions = read_transitions_from_csv("automato/all-Zigzin-DFA-transitions.csv")?;

    // Create a HashSet of accept states [1,2,3,4,5,6,7,8,9,10,11].
    let accept: HashMap<usize, String> = read_accept_states_from_json("automato/all-Zigzin-DFA-final-states.json")?;

    // Define the NFA (assuming start state is 0).
    let dfa = DFA {
        transitions,
        start: 0,
        accept,
    };

    println!("dfa acc: {:?}", dfa.accept);

    // Process input from a file using the DFA.
    let (accepted, final_state) = process_file_input(&dfa, "tests/lexer_first_test.txt")?;
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
