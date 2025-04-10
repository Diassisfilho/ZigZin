import argparse
import xml.etree.ElementTree as ET
import csv
import string

# Set up command line arguments
parser = argparse.ArgumentParser(description='Convert a JFF automaton file to a CSV transition table.')
parser.add_argument('input_file', help='Path to the JFF automaton file')
parser.add_argument('output_file', help='Path for the output CSV file')
args = parser.parse_args()

# Parse the input JFF file
tree = ET.parse(args.input_file)
root = tree.getroot()

transitions = []

# Iterate over each transition element in the XML
for transition in root.iter('transition'):
    from_state = transition.find('from').text
    to_state = transition.find('to').text
    read = transition.find('read').text
    # Use 'ε' to denote an empty string (if the read tag is empty)
    if read is None or read == "":
        read = None
    
    # Expand transitions for ranges [0-9] and [a-z]
    if read == "[0-9]":
        for c in map(str, range(10)):
            transitions.append([from_state, c, to_state])
    elif read == "[a-z]":
        for c in string.ascii_lowercase:
            transitions.append([from_state, c, to_state])
    elif read == "[A-Z]":
        for c in string.ascii_uppercase:
            transitions.append([from_state, c, to_state])
    else:
        transitions.append([from_state, read, to_state])

# Write the transitions to a CSV file
with open(args.output_file, 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['From', 'Input', 'To'])
    writer.writerows(transitions)

print(f"Transition table exported to {args.output_file}")