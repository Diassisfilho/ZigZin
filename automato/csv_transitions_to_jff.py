import csv
import xml.etree.ElementTree as ET
import argparse

def create_jff(states, transitions, initial_state, final_states):
    # Create the root element and add type and automaton subelements.
    root = ET.Element("structure")
    type_elem = ET.SubElement(root, "type")
    type_elem.text = "fa"
    automaton = ET.SubElement(root, "automaton")
    
    # Create state elements.
    # For a simple layout, we assign x coordinate = id * 100.0, and y fixed to 100.0
    for state in sorted(states, key=lambda s: int(s)):
        state_elem = ET.SubElement(automaton, "state", id=state, name=f"q{state}")
        # Default positions
        ET.SubElement(state_elem, "x").text = str(float(state)*100.0)
        ET.SubElement(state_elem, "y").text = "100.0"
        # Mark initial if this is the chosen initial state
        if state == initial_state:
            ET.SubElement(state_elem, "initial")
        # Mark as final if in final_states list
        if state in final_states:
            ET.SubElement(state_elem, "final")
    
    # Create transition elements.
    for t in transitions:
        trans_elem = ET.SubElement(automaton, "transition")
        ET.SubElement(trans_elem, "from").text = t["from"]
        ET.SubElement(trans_elem, "to").text = t["to"]
        # If read is empty string, JFLAP usually expects an empty tag
        read_text = t["read"].strip()
        ET.SubElement(trans_elem, "read").text = read_text if read_text != "" else ""
    
    return root

def main():
    parser = argparse.ArgumentParser(description="Convert CSV with transitions into a JFLAP .jff file.")
    parser.add_argument("csv_file", help="Input CSV file with columns: from, read, to")
    parser.add_argument("jff_file", help="Output JFF file name")
    args = parser.parse_args()
    
    transitions = []
    states = set()
    
    with open(args.csv_file, newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            # Collect transition data
            frm = row["From"].strip()
            read_val = row["Input"].strip()
            to = row["To"].strip()
            transitions.append({"from": frm, "read": read_val, "to": to})
            states.add(frm)
            states.add(to)
    
    # For demonstration, set initial state as the one with smallest numeric id and
    # final state as the one with largest numeric id.
    sorted_states = sorted(states, key=lambda s: int(s))
    initial_state = sorted_states[0]
    final_states = { sorted_states[-1] }
    
    jff_root = create_jff(states, transitions, initial_state, final_states)
    
    # Write to file with pretty printing.
    tree = ET.ElementTree(jff_root)
    tree.write(args.jff_file, encoding="utf-8", xml_declaration=True)
    print(f"JFF file created: {args.jff_file}")

if __name__ == "__main__":
    main()
