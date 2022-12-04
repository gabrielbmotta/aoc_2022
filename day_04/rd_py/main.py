from pathlib import Path

def read_lines(file):
    with open(file, "r") as f:
        lines = f.read().splitlines()
    return lines
    
def split_assignment(assignment,splitter):
    return assignment.split(splitter)

def to_int(range):
    return [int(i) for i in range]

def get_range(assignment):
    assignment = split_assignment(assignment,'-')
    return to_int(assignment)

def is_in_range(a,b):
    if a[0] >= b[0] and a[1] <= b[1]:
        return True
    else:
        return False

def fully_contains(a,b):
    if is_in_range(a,b):
        return True
    elif is_in_range(b,a):
        return True
    else:
        return False

if __name__ == '__main__':
    file = Path(__file__).parent / "input.txt"
    lines = read_lines(file)
    overlapping = 0
    for assignment in lines:
        elf1,elf2 = split_assignment(assignment,',')
        range1 = get_range(elf1)
        range2 = get_range(elf2)
        if fully_contains(range1,range2):
            overlapping += 1
    print(f"Overlapping: {overlapping}")

