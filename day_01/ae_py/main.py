fn = "input.txt"
with open(fn) as f:
    maxCal = curCal = 0
    for line in f:
        if line == "\n":
            if curCal > maxCal:
                maxCal = curCal
            curCal = 0
        else:
            curCal += int(line)
print(maxCal)