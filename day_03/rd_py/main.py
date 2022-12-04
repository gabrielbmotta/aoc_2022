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

def get_groups_of_three(lines):
    """ returns the groups of three lines """
    groups = []
    for i in range(0, len(lines), 3):
        groups.append(lines[i:i+3])
    return groups

def get_duplicates_in_groups(group):
    """ returns the duplicates in a group of three lines """
    duplicates = set()

    for letter in group[0]:
        if letter in group[1] and letter in group[2]:
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

    groups = get_groups_of_three(lines)
    total_priority = 0
    for group in groups:
        duplicates = get_duplicates_in_groups(group)
        for element in duplicates:
            priotity = get_priority(element)
            total_priority += priotity
    print(f"Total priority {total_priority}")



# %%
