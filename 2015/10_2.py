input_string = "1113122113"


def process(line):
    new_line = []
    num_count = 1
    for idx, num in enumerate(line):
        if idx == len(line) - 1:
            new_line.append(num_count)
            new_line.append(int(num))
        elif num == line[idx + 1]:
            num_count += 1
        else:
            new_line.append(num_count)
            new_line.append(int(num))
            num_count = 1
    return new_line


new_string = input_string
for i in range(0, 50):
    new_string = process(new_string)

print(len(new_string))
