#%%
from pathlib import Path
import re

# I know ... :D
INSTRUCTIONS = {
    '1':['Z', 'T', 'F', 'R', 'W', 'J', 'G'],
    '2':['G', 'W', 'M'],    
    '3':['J', 'N', 'H', 'G'],
    '4':['J', 'R', 'C', 'N', 'W'],
    '5':['W', 'F', 'S', 'B', 'G', 'Q', 'V', 'M'],
    '6':['S', 'R', 'T', 'D', 'V', 'W', 'C'],
    '7':['H', 'B', 'N', 'C', 'D', 'Z', 'G', 'V'],
    '8':['S', 'J', 'N', 'M', 'G', 'C'],
    '9':['G', 'P', 'N', 'W', 'C', 'J', 'D','L']
}

TEST = {
    '1':['Z', 'N'],
    '2':['M', 'C', 'D'],
    '3':['P'],
}

def read_lines(file):
    with open(file, "r") as f:
        lines = f.read().splitlines()
    return lines

def get_instruction(instruction):
    regex = re.compile(r'\d+')
    regex.findall(instruction)
    n_, from_, to_ = regex.findall(instruction)
    return [int(n_), from_, to_]

def get_pile():
    pile = {
    '1':['Z', 'T', 'F', 'R', 'W', 'J', 'G'],
    '2':['G', 'W', 'M'],    
    '3':['J', 'N', 'H', 'G'],
    '4':['J', 'R', 'C', 'N', 'W'],
    '5':['W', 'F', 'S', 'B', 'G', 'Q', 'V', 'M'],
    '6':['S', 'R', 'T', 'D', 'V', 'W', 'C'],
    '7':['H', 'B', 'N', 'C', 'D', 'Z', 'G', 'V'],
    '8':['S', 'J', 'N', 'M', 'G', 'C'],
    '9':['G', 'P', 'N', 'W', 'C', 'J', 'D','L']
}

    return pile

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
    instructions = read_lines(input_instructions)

    # task 1
    pile = get_pile()

    for instruction in instructions:
        pile = move9000(pile, instruction)
    print_results(pile)

    # task 2
    pile = get_pile()
    for instruction in instructions:
        pile = move9001(pile, instruction)
    print_results(pile)
# %%
