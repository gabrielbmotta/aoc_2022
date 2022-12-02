#%%
from pathlib import Path

# define rules
OPPONEND = {'A':'rock', 'B':'paper', 'C':'scissor'}
ME = {'X':'rock', 'Y':'paper', 'Z':'scissor'}
POINTS = {'rock':1, 'paper':2, 'scissor':3}

def read_data(input_file):
    with open(input_file, "r") as f:
        data = f.read()

    # split the data into lines
    return data.splitlines()

# part 1

def get_play(game):
    return OPPONEND[game[0]], ME[game[2]]

def compute_score(opp, me):
    # rock paper scissor
    result = 0
    if opp == me:
        result = POINTS[me] + 3
    elif (opp == 'rock' and me == 'scissor') or (opp == 'paper' and me == 'rock') or (opp == 'scissor' and me == 'paper'):
        result = POINTS[me] + 0
    else:
        result = POINTS[me] + 6

    return result


if __name__ == "__main__":
    input_file = Path(__file__).parent / "input.txt"
    games = read_data(input_file)
    score = 0

    for game in games:
        opp, me = get_play(game)
        score = score + compute_score(opp, me)

    print(f"Score: {score}")

# %%
