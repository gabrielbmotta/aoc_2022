#%%
from pathlib import Path

def read_lines(file):
    with open(file, "r") as f:
        data = f.read().splitlines()
    return data

if __name__ == '__main__':
    file = Path(__file__).parent / "test.txt"
    lines = read_lines(file)
    print(lines)

# %%
