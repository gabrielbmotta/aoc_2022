#%%
from pathlib import Path
import re
import json

# I know ... :D

def read_pile(file):
    with open(file, "r") as f:
        pile = json.load(f)
    return pile

def read_lines(file):
    with open(file, "r") as f:
        lines = f.read().splitlines()
    return lines

def get_instruction(instruction):
    regex = re.compile(r'\d+')
    regex.findall(instruction)
    n_, from_, to_ = regex.findall(instruction)
    return [int(n_), from_, to_]

def move9000(pile, instruction):
    n_, from_, to_ = get_instruction(instruction)
    return_pile = pile.copy()
    for i in range(n_):
        return_pile[to_].append(return_pile[from_].pop())
    return return_pile

def move9001(pile, instruction):
    n_, from_, to_ = get_instruction(instruction)
    return_pile = pile.copy()

    temp_crates = return_pile[from_][-n_:]
    return_pile[from_] = return_pile[from_][:-n_]

    return_pile[to_] = return_pile[to_] + temp_crates

    return return_pile

def print_results(pile):
    top_crates = [last_element(pile[key]) for key in pile.keys()]
    print(''.join(top_crates))

def last_element(crate):
    if crate == []:
        return ' '
    else:
        return crate[-1]


if __name__ == "__main__":
    input_instructions = Path(__file__).parent / "input_instruction.txt"    
    pile_file = Path(__file__).parent / "input_pile.json" 

    instructions = read_lines(input_instructions)
    
    # task 1
    pile = read_pile(pile_file)
    for instruction in instructions:
        pile = move9000(pile, instruction)
    print_results(pile)

    # task 2
    pile = read_pile(pile_file)
    for instruction in instructions:
        pile = move9001(pile, instruction)
    print_results(pile)
# %%
