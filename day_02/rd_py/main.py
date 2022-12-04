#%%
from pathlib import Path

def read_data(input_file):
    with open(input_file, "r") as f:
        data = f.read()

    # split the data into lines
    return data.splitlines()

def get_play1(letter_op,letter_me):
    opp = get_opponend(letter_op)
    me = get_me(letter_me)
    return opp, me
    
def get_me(letter):
    me = {'X':'rock', 'Y':'paper', 'Z':'scissor'}
    return me[letter]

def get_letters(game):
    return game[0], game[2]

def get_instruction(letter):
    instructions = {'X':'loose', 'Y':'draw', 'Z':'win'}
    return instructions[letter]

def get_opponend(letter):
    opponed = {'A':'rock', 'B':'paper', 'C':'scissor'}
    return opponed[letter]

def get_points_play(play):
    points_play = {'rock':1, 'paper':2, 'scissor':3}
    return points_play[play]

def get_points_outcome(outcome):
    points_outcome = {'win':6, 'draw':3, 'loose':0}
    return points_outcome[outcome]

def is_draw(opp, me):
    return opp == me

def is_loose(opp, me):
    if (opp == 'rock' and me == 'scissor') or (opp == 'paper' and me == 'rock') or (opp == 'scissor' and me == 'paper'):
        return True
    else:
        return False

def get_draw(opp):
    return opp

def get_win(opp):
    if opp == 'rock':
        return 'paper'
    elif opp == 'paper':
        return 'scissor'
    else:
        return 'rock'

def get_loose(opp):
    if opp == 'rock':
        return 'scissor'
    elif opp == 'paper':
        return 'rock'
    else:
        return 'paper'

def compute_score_1(opp, me):
    opp, me = get_play1(opp,me)
    # rock paper scissor
    outcome = ''
    if is_draw(opp, me):
        outcome = 'draw'
    elif is_loose(opp, me):
        outcome = 'loose'
    else:
        outcome = 'win'

    return get_points_play(me) + get_points_outcome(outcome)
    
def compute_score_2(opp,me):
    opp = get_opponend(opp)
    instruction = get_instruction(me)

    if instruction == 'draw':
        me = get_draw(opp)

    elif instruction == 'win':
        me = get_win(opp)
    
    else:
        me = get_loose(opp)

    return get_points_play(me) + get_points_outcome(instruction)

if __name__ == "__main__":
    input_file = Path(__file__).parent / "input.txt"
    games = read_data(input_file)
    score_1 = 0
    score_2 = 0
    for game in games:
        opp, me = get_letters(game)
        score_1 = score_1 + compute_score_1(opp, me)
        score_2 = score_2 + compute_score_2(opp, me)
    print(f"Score 1: {score_1}")
    print(f"Score 2: {score_2}")

# %%
