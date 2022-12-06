from pathlib import Path

def read_lines(path):
    with open(path, 'r') as f:
        return f.readlines()
    
if __name__ == '__main__':
    path = Path(__file__).parent / 'input.txt'
    lines = read_lines(path)[0]
    nchar = 0
    marker = []
    message = 14
    package = 4
    max_marker = 14
    n_total = 0
    package_detected = False
    for i in range(len(lines)):
        # check next four char if same as current
        # if same, add to marker
        current = lines[i]
        marker.append(current)
        # past four chars
        if package_detected:
            if len(marker) > (message + package):
                past_fourteen = lines[i-message:i]
                if len(set(past_fourteen)) == message:
                    n_message = i 
                    print(i)
                    break
        else:
            if len(marker) > (package):
                past_four = lines[i-package:i]
                if len(set(past_four)) == package:
                    n_total = i 
                    package_detected = True


    print("Package: ", past_four)
    print("Total package: ", n_total)
    print("Message: ", past_fourteen)
    print("Total message: ", n_message)
