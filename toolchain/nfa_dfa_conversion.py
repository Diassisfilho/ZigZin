import json
import csv
from collections import deque
from typing import Dict, List, Optional, Set, Tuple, Any


def read_zigzin_states_types(file_path: str) -> Dict[int, str]:
    """
    Reads the ZigZin-NFA-states-types.json file and converts it into an NFA accept structure.
    Returns a dictionary where each key is an accept state and the value is its label.
    """
    with open(file_path, "r") as f:
        data = json.load(f)
    # data is expected to have "initial" and "final" keys
    finals = data.get("final", [])
    accept = {}
    for state, label in finals:
        accept[int(state)] = label
    return accept


class NFA:
    """
    Representation of an NFA.
    transitions: A dictionary where the key is a tuple (state, symbol) and symbol is either a character or None (for ε-transitions).
    start: The start state.
    accept: A mapping from an accept state to its label.
    """
    def __init__(self, transitions: Dict[Tuple[int, Optional[str]], List[int]], start: int, accept: Dict[int, str]):
        self.transitions = transitions
        self.start = start
        self.accept = accept


class DFA:
    """
    Representation of a DFA.
    transitions: A dictionary where the key is a tuple (dfa_state, symbol) and the value is the next state.
    start: The start state (always 0 in our conversion).
    accept: A mapping from an accept state to its label.
    """
    def __init__(self, transitions: Dict[Tuple[int, str], int], start: int, accept: Dict[int, str]):
        self.transitions = transitions
        self.start = start
        self.accept = accept

    def __repr__(self):
        return f"DFA(start={self.start}, transitions={self.transitions}, accept={self.accept})"


def epsilon_closure(nfa: NFA, states: Set[int]) -> Set[int]:
    """
    Computes the epsilon closure of a set of NFA states.
    """
    closure = set(states)
    stack = list(states)

    while stack:
        state = stack.pop()
        # ε-transitions are represented by None
        next_states = nfa.transitions.get((state, None), [])
        for next_state in next_states:
            if next_state not in closure:
                closure.add(next_state)
                stack.append(next_state)
    return closure


def move_nfa(nfa: NFA, states: Set[int], symbol: str) -> Set[int]:
    """
    Given a set of NFA states and a symbol, returns the set of states reachable by that symbol.
    """
    result = set()
    for state in states:
        next_states = nfa.transitions.get((state, symbol), [])
        for next_state in next_states:
            result.add(next_state)
    return result


def convert_nfa_to_dfa(nfa: NFA, alphabet: Set[str]) -> DFA:
    """
    Converts an NFA to a DFA using the subset construction algorithm.
    `alphabet` is the set of input symbols (excluding ε).
    """
    dfa_transitions: Dict[Tuple[int, str], int] = {}
    dfa_accept: Dict[int, str] = {}

    # We use frozenset to represent a set of NFA states in a hashable way.
    state_mapping: Dict[frozenset, int] = {}
    dfa_states: List[frozenset] = []

    # Start state (apply epsilon closure)
    start_set = {nfa.start}
    start_closure = frozenset(epsilon_closure(nfa, start_set))
    state_mapping[start_closure] = 0
    dfa_states.append(start_closure)

    # Precompute the set of NFA accept states for easy lookup.
    nfa_accept_set = set(nfa.accept.keys())
    if start_closure & nfa_accept_set:
        # Join all the labels from the intersecting accept states.
        labels = [nfa.accept[s] for s in start_closure if s in nfa.accept]
        dfa_accept[0] = ", ".join(labels)

    queue = deque([0])
    while queue:
        current_index = queue.popleft()
        current_state_set = dfa_states[current_index]
        # For each symbol in the alphabet, compute the set of reachable states.
        for symbol in alphabet:
            # Apply move and then epsilon closure.
            move_set = move_nfa(nfa, set(current_state_set), symbol)
            if not move_set:
                continue
            next_closure = frozenset(epsilon_closure(nfa, move_set))
            if next_closure in state_mapping:
                next_index = state_mapping[next_closure]
            else:
                next_index = len(dfa_states)
                state_mapping[next_closure] = next_index
                dfa_states.append(next_closure)
                queue.append(next_index)
                if next_closure & nfa_accept_set:
                    labels = [nfa.accept[s] for s in next_closure if s in nfa.accept]
                    dfa_accept[next_index] = ", ".join(labels)
            dfa_transitions[(current_index, symbol)] = next_index

    return DFA(transitions=dfa_transitions, start=0, accept=dfa_accept)


def read_nfa_transitions_csv(file_path: str) -> Dict[Tuple[int, Optional[str]], List[int]]:
    """
    Reads a CSV file of NFA transitions and returns a dictionary.
    The CSV is expected to have three columns: From, Input, To.
    Lines starting with a comment (e.g., "//") will be skipped.
    An input symbol of an empty string is interpreted as None.
    """
    transitions: Dict[Tuple[int, Optional[str]], List[int]] = {}

    with open(file_path, newline="") as csvfile:
        reader = csv.reader(csvfile)
        for row in reader:
            # Skip empty rows or comment lines.
            if not row or (row[0].strip().startswith("//")):
                continue
            if len(row) != 3:
                continue

            try:
                frm = int(row[0].strip())
                input_str = row[1].strip()
                symbol = input_str[0] if input_str else None
                to = int(row[2].strip())
            except ValueError:
                continue

            key = (frm, symbol)
            transitions.setdefault(key, []).append(to)
    return transitions

def write_dfa_to_csv(dfa: DFA, file_path: str) -> None:
    """
    Writes the DFA transitions to a CSV file.
    The CSV will have three columns: From, Input, and To.
    """
    with open(file_path, "w", newline="") as csvfile:
        writer = csv.writer(csvfile)
        # Write header
        writer.writerow(["From", "Input", "To"])
        # Write each transition
        for (from_state, input_symbol), to_state in dfa.transitions.items():
            writer.writerow([str(from_state), str(input_symbol), str(to_state)])

def get_alphabet_from_transitions(transitions: Dict[Tuple[int, Optional[str]], Any]) -> List[str]:
    """
    Given a dictionary of transitions where keys are tuples (state, symbol),
    returns a sorted list of unique input symbols (alphabet), excluding None (ε-transitions).
    """
    alphabet = {symbol for (_, symbol) in transitions.keys() if symbol is not None}
    return sorted(alphabet)

def write_dfa_accept_to_json(dfa_accept: Dict[int, str], file_path: str) -> None:
    """
    Converts the DFA accept mapping into a JSON file.
    The output JSON format is a list of lists: [[state, label], [state, label]].
    
    :param dfa_accept: Dictionary mapping DFA accept states (int) to their label (str).
    :param file_path: Path to the output JSON file.
    """
    # Convert the dictionary into a list of [state, label] pairs.
    data = [[state, label] for state, label in dfa_accept.items()]
    
    # Write the data to a JSON file.
    with open(file_path, "w") as json_file:
        json.dump(data, json_file)


# Example usage (you can remove or comment this out when integrating into your project)
if __name__ == "__main__":
    # Example: read accept states from a JSON file.
    # accept = read_zigzin_states_types("ZigZin-NFA-states-types.json")
    # print("Accept states:", accept)

    # Example: read transitions from a CSV file.
    # transitions = read_nfa_transitions_csv("nfa_transitions.csv")
    # print("Transitions:", transitions)

    # Build an example NFA and convert it to a DFA.
    # sample_transitions = {
    #     (0, None): [1],
    #     (1, 'a'): [2],
    #     (2, None): [3],
    #     (3, 'b'): [4],
    # }
    # sample_accept = {4: "accept"}
    # nfa = NFA(transitions=sample_transitions, start=0, accept=sample_accept)
    # alphabet = {'a', 'b'}
    # dfa = convert_nfa_to_dfa(nfa, alphabet)
    # print("DFA:", dfa)

    transitions = read_nfa_transitions_csv("automato/all-Zigzin-NFA-transitions.csv")
    accept = read_zigzin_states_types("automato/Zigzin-NFA-states-types.json")
    nfa = NFA(transitions,0,accept)
    dfa = convert_nfa_to_dfa(nfa, alphabet=get_alphabet_from_transitions(transitions))
    write_dfa_to_csv(dfa,"automato/all-Zigzin-DFA-transitions.csv")
    write_dfa_accept_to_json(dfa.accept, "automato/all-Zigzin-DFA-final-states.json")
