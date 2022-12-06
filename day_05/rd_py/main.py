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
    return INSTRUCTIONS

def move(pile, instruction):
    n_, from_, to_ = instruction
    return_pile = pile.copy()
    for i in range(n_):
        return_pile[to_].append(return_pile[from_].pop())
    return return_pile


if __name__ == "__main__":
    input_instructions = Path(__file__).parent / "input_instruction.txt"
    input_pile = Path(__file__).parent / "input_pile.txt"
    

    instructions = read_lines(input_instructions)
    pile = get_pile()

    for instruction in instructions:
        instruct = get_instruction(instruction)
        pile = move(pile, instruct)

    top_crates = [pile[key][-1] for key in pile.keys()]
    print(pile)
    print(''.join(top_crates))


# %%
