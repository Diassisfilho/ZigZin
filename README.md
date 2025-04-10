# ZigZin

ZigZin is a front-end compiler implementation to an subset of zig language in Rust that processes input files using a Deterministic Finite Automaton (DFA).

## Prerequisites

- Rust (tested with rustc 1.81.0)
- Cargo (Rust's package manager)

## Project Structure

```
ZigZin/
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   └── tokens.rs
├── automato/
│   ├── DFA-transitions.csv
│   └── DFA-final-states.json
│   └── NFA-transitions.csv
│   └── NFA-final-states.json
└── README.md
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Diassisfilho/ZigZin.git
cd ZigZin
```

2. Build the project:
```bash
cargo build --release
```

## Usage

Run the program with an input file:

```bash
cargo run -- path/to/your/input/file
```

### Input Files

The program requires two configuration files in the `automato` directory:

1. `DFA-transitions.csv`: Contains the DFA transitions
2. `DFA-final-states.json`: Contains the accept states and their corresponding labels

### Example

```bash
cargo run -- tests/lexer_input_test.zig
```

## File Formats

### DFA Transitions (CSV)
The transitions file should be in CSV format with the following structure:
```csv
current_state,input_character,next_state
```

### Accept States (JSON)
The accept states file should be in JSON format with the following structure:
```json
[
    [1,"exclamation"],
    [2,"double quotes"],
    ...
]
```

## Error Handling

The program will display appropriate error messages if:
- The input file path is not provided
- The configuration files are missing or malformed
- There are invalid transitions in the input

## License
MIT 