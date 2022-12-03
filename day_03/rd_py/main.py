#%%
from pathlib import Path

def read_lines(file):
    with open(file, "r") as f:
        data = f.read().splitlines()
    return data

def get_compartments(rucksack):
    """ splits string rucksack into two eqaul sized compartments """
    n = len(rucksack)
    return rucksack[:n // 2], rucksack[n // 2:]

def get_priority(letter):
    """ returns the priority of a letter lowercase a-z ranges from 1-26, A-Z ranges from 27-52 """
    if letter.islower():
        return ord(letter) - 96
    else:
        return ord(letter) - 64 + 26

def get_duplicates(comp1, comp2):
    """ returns the duplicate letters in two compartments """
    duplicates = set()
    for letter in comp1:
        if letter in comp2:
            duplicates.add(letter)
    return duplicates

if __name__ == '__main__':
    file = Path(__file__).parent / "input.txt"
    lines = read_lines(file)
    total_priority = 0
    for rucksack in lines:
        comp1, comp2 = get_compartments(rucksack)
        duplicates = get_duplicates(comp1, comp2)
        for element in duplicates:
            priotity = get_priority(element)
            total_priority += priotity
    print(f"Total priority {total_priority}")


# get_priotity




# %%
