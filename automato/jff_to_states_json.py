import argparse
import xml.etree.ElementTree as ET
import json

# Set up command line arguments
parser = argparse.ArgumentParser(description='Convert a JFF automaton file to a JSON file with initial and final state IDs.')
parser.add_argument('input_file', help='Path to the JFF automaton file')
parser.add_argument('output_file', help='Path for the output JSON file')
args = parser.parse_args()

# Parse the input JFF file
tree = ET.parse(args.input_file)
root = tree.getroot()

initial_states = []
final_states = []

# Iterate over each state element in the JFF file
for state in root.iter('state'):
    # Get the state id and convert it to int if possible
    state_id = state.get('id')
    try:
        state_id = int(state_id)
    except (ValueError, TypeError):
        pass

    # Check if this state is marked as initial or final
    if state.find('initial') is not None:
        initial_states.append(state_id)
    if state.find('final') is not None:
        final_states.append(state_id)

output_data = {
    "initial": initial_states,
    "final": final_states
}

# Write the output JSON file
with open(args.output_file, 'w') as jsonfile:
    json.dump(output_data, jsonfile, indent=4)

print(f"State information exported to {args.output_file}")