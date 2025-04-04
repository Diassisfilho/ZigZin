use std::collections::HashMap;
use std::env;
use std::error::Error;

pub mod lexer;
use lexer::{
    process_file_input, DFA, read_accept_states_from_json, read_transitions_from_csv
};
pub mod tokens;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <input file path>", args[0]);
        std::process::exit(1);
    }
    
    // The file path is passed as the first command line argument.
    let file_path = &args[1];

    // Read NFA transitions from CSV file.
    let transitions = read_transitions_from_csv("automato/DFA-transitions.csv")?;

    // Create a HashSet of accept states [1,2,3,4,5,6,7,8,9,10,11].
    let accept: HashMap<usize, String> = read_accept_states_from_json("automato/DFA-final-states.json")?;

    // Define the NFA (assuming start state is 0).
    let dfa = DFA {
        transitions,
        start: 0,
        accept,
    };

    // Process input from a file using the DFA.
    let tokens = process_file_input(&dfa, &file_path)?;
    println!("{:?}",tokens);
    
    Ok(())
}
