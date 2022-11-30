print("Hello world!")

# We open the file test.txt and kee track of it.
# Because this is python we assume it succeeds and don't error check :)
file = open("test.txt", mode='r')

# We read in the file as a list,
# each list entry is a line of the file
lines = file.readlines()

# We iterate over each line,
for line in lines:
    # and split the line into two characters.
    val1, val2 = line.split()
    
    # We print the characters with a comma separating them.
    print(val1 + ", " + val2)
