import hashlib
input_string = "yzbqklnj"

counter = 1
while True:
    string_to_check = input_string + str(counter)
    output = hashlib.md5(str.encode(string_to_check)).hexdigest()
    if output[:6] == '000000':
        break
    counter += 1
print(f'{counter} is the right prefix')
