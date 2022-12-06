from pathlib import Path

def read_lines(path):
    with open(path, 'r') as f:
        return f.readlines()
    
if __name__ == '__main__':
    path = Path(__file__).parent / 'input.txt'
    lines = read_lines(path)[0]
    nchar = 0
    marker = []
    max_marker = 4
    n_total = 0

    for i in range(len(lines)):
        # check next four char if same as current
        # if same, add to marker
        current = lines[i]
        marker.append(current)
        # past four chars
        if len(marker) > max_marker:
            past_four = lines[i-4:i]
            if len(set(past_four)) == max_marker:
                n_total = i 
                break

    print("Marker: ", past_four)
    print("Total: ", n_total)
