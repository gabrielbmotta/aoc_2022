#%%
from pathlib import Path

def read_lines(file):
    with open(file, "r") as f:
        lines = f.read().splitlines()
    return lines

def construct_pile(strings):
    n_piles = len(strings[0])
    height = len(strings) - 1
    piles = []
    for i in range(0, n_piles):
        pile_i = []
        for p in range(0,height):
            pile_i.append(strings[i,p])
        piles.append(pile_i)


if __name__ == "__main__":
    input_instructions = Path(__file__).parent / "input_instruction.txt"
    input_pile = Path(__file__).parent / "input_pile.txt"
    
    instructions = read_lines(input_instructions)
    pile = read_lines(input_pile)

    print(pile)

# %%
