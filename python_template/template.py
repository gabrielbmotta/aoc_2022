print("Hello world!")

file = open("test.txt", mode='r')
lines = file.readlines()

for line in lines:
    val1, val2 = line.split()
    print(val1 + ", " + val2)
    