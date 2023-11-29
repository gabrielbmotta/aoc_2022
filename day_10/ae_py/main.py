read = 0
execute = 1
register = 1
phase = read
programCounter = 0
cycle = 1
addend = 0
answers = []
op = None
line = ["." for x in range(40)]

with open("C:\\Advent of Code\\aoc_2022\\day_10\\ae_py\\input.txt", "r") as f:
    inputLines = f.readlines()

out = open("C:\\Advent of Code\\aoc_2022\\day_10\\ae_py\\out.txt", "w")

for x in range(240):
    #Beginning of cycle
    if phase == read:
        instruction = inputLines[programCounter]
        op = instruction.split(" ")[0].strip()
        if op == "addx":
            addend = int(instruction.split(" ")[1].strip())
        else:
            programCounter += 1

    #During cycle
    if abs((cycle%40-1)-register) <=1: #Check if sprite intersects with drawn pixel
        line[(cycle%40)-1] = "#"

    #End of cycle
    if phase == execute: # phase == execute
        programCounter += 1
        if op == "addx":
            register += addend

    if cycle%40 == 0: #If end of row
        #Print row
        out.write("".join(line) + "\n")
        line = ["." for x in range(40)]

    if op == "addx": #Only go to execute phase for addx op
        phase = (phase+1)%2
    
    cycle += 1