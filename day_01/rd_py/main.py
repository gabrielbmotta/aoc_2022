from pathlib import Path

def read_input_lines(file):
    """ Read input file and return list of lines 
    args:
        file (str): path to input file
    returns:
        list: list of lines"""
    with open(file, "r") as f:
        input_data = f.read().splitlines()
    return input_data

def get_calories_per_elf(input_data):
    """ Get calories per elf from input data
    args:
        input_data (list): list of lines
    returns:
        list: calories per elf"""
    elf = 0
    calories_per_elf = []
    ncalories = 0
    for calories in input_data:
        if calories == "":
            # increment elf
            elf += 1
            # append added calories
            calories_per_elf.append(ncalories)
            # reset calories counter
            ncalories = 0
        else:
            ncalories += int(calories)

    return calories_per_elf

def sort_calories(calories):
    """ Sort calories per elf
    args:
        calories_per_elf (list): list of calories
    returns:
        list: sorted list of calories"""
    return sorted(calories, reverse=True)

def get_max_calories_and_elf(calories_per_elf):
    """ Get max calories per elf
    args:
        calories_per_elf (list): list of calories per elf
    returns:
        int, int: max calories and elf"""
    max_calories = max(calories_per_elf)
    max_elf = calories_per_elf.index(max_calories)
    return max_calories, max_elf

def sum_n_max_calories(sorted_calories, n):
    """ Sum the n max calories
    args:
        sorted_calories (list): sorted list of calories per elf
        n (int): number of max calories to sum
    returns:
        int: number of n max calories"""
    return sum(sorted_calories[:n])

if __name__ == "__main__":
    file = Path(__file__).parent / "input.txt"
    calories = read_input_lines(file)
    calories_per_elf = get_calories_per_elf(calories)
    max_calories, max_elf = get_max_calories_and_elf(calories_per_elf)
    print(f"Max calories: {max_calories} by elf {max_elf}")
    sorted_calories_per_elf = sort_calories(calories_per_elf)
    max3_calories = sum_n_max_calories(sorted_calories_per_elf, 3)
    print(f"Sum of 3 max calories: {max3_calories}")


