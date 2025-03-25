mod lexer;
use crate::lexer::read_nfa_convert_to_dfa;

fn main() {
    // Unwrap the Option and store the tuple in a variable.
    let conversion = read_nfa_convert_to_dfa("automato/all-Zigzin-NFA-transitions.csv")
        .expect("Conversion failed");

    // Destructure the tuple into DFA and NFA variables.
    let (dfa, nfa) = conversion;

    // Now you can use dfa and nfa after this point.
    println!("DFA: {:?}", dfa);
    println!("NFA: {:?}", nfa);

    // Additional processing using dfa and nfa...
}