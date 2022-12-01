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

def get_max_calories_and_elf(input_data):
    """ Get max calories and elf from input data
    args:
        input_data (list): list of lines
    returns:
        int, int: max calories and elf"""
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

    # find elf with max calories
    max_calories = max(calories_per_elf)
    max_elf = calories_per_elf.index(max_calories)
    return max_calories, max_elf

if __name__ == "__main__":
    file = Path(__file__).parent / "input.txt"
    calories = read_input_lines(file)

    max_calories, max_elf = get_max_calories_and_elf(calories)
    print(f"Elf {max_elf} has the most calories: {max_calories}")
